#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
通用补跑脚本：补跑 p1/p2/p3 实验中缺失的 Pass@5 文件
支持的实验类型：
	- p1: 单步生成，文件命名 00.rs ~ 04.rs
	- p2: 两步生成 (code -> spec)，文件命名 code00.rs/complete00.rs ~ code04.rs/complete04.rs
	- p3: 两步生成 (spec -> code)，文件命名 spec00.rs/complete00.rs ~ spec04.rs/complete04.rs
支持的实验变体：
	- fewshot+LTM
	- fewshot
	- fewshot+COT
	- zeroshot
"""

import argparse
import json
import os
import sys
import time
import logging
from datetime import datetime, timezone
from typing import Any, Dict, List, Optional, Tuple
from concurrent.futures import ThreadPoolExecutor, as_completed
from tqdm import tqdm

logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')

try:
	import yaml
except Exception:
	yaml = None

import urllib.request
import urllib.error
import ssl

WORKDIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FEWSHOT_DIR = os.path.join(WORKDIR, 'dataset', 'fewshot')

# Pipeline 配置（与实验变体无关）
PIPELINE_CONFIG = {
	'p1': {
		'file_pattern': '{idx:02d}.rs',  # 单文件模式
		'steps': 1,
	},
	'p2': {
		'file_patterns': ['code{idx:02d}.rs', 'complete{idx:02d}.rs'],  # 双文件模式
		'steps': 2,
		'step1_name': 'code',
		'step2_name': 'complete',
	},
	'p3': {
		'file_patterns': ['spec{idx:02d}.rs', 'complete{idx:02d}.rs'],  # 双文件模式
		'steps': 2,
		'step1_name': 'spec',
		'step2_name': 'complete',
	},
}

# 实验变体配置
VARIANT_CONFIG = {
	'fewshot+LTM': {
		'dir_names': {
			'p1': 'p1_fewshot+LTM',
			'p2': 'p2_fewshot+LTM',
			'p3': 'p3_fewshot+LTM',
		},
		'prompt_files': {
			'p1': 'p1_fewshot+LTM.py',
			'p2': 'p2_fewshot+LTM.py',
			'p3': 'p3_fewshot+LTM.py',
		},
		'use_fewshot_replacements': True,
	},
	'fewshot': {
		'dir_names': {
			'p1': 'p1_fewshot',
			'p2': 'p2_fewshot',
			'p3': 'p3_fewshot',
		},
		'prompt_files': {
			'p1': 'p1_fewshot.py',
			'p2': 'p2_fewshot.py',
			'p3': 'p3_fewshot.py',
		},
		'use_fewshot_replacements': True,
	},
	'fewshot+COT': {
		'dir_names': {
			'p1': 'p1_fewshot+COT',
			'p2': 'p2_fewshot+COT',
			'p3': 'p3_fewshot+COT',
		},
		'prompt_files': {
			'p1': 'p1_fewshot+COT.py',
			'p2': 'p2_fewshot+COT.py',
			'p3': 'p3_fewshot+COT.py',
		},
		'use_fewshot_replacements': True,
	},
	'zeroshot': {
		'dir_names': {
			'p1': 'p1_zeroshot',
			'p2': 'p2_zeroshot',
			'p3': 'p3_zeroshot',
		},
		'prompt_files': {
			'p1': 'p1_zeroshot.py',
			'p2': 'p2_zeroshot.py',
			'p3': 'p3_zeroshot.py',
		},
		'use_fewshot_replacements': False,
	},
}


def read_text(path: str) -> str:
	with open(path, 'r', encoding='utf-8') as f:
		return f.read()


def naive_yaml_load(text: str) -> Dict[str, Any]:
	result: Dict[str, Any] = {}
	stack: List[Tuple[int, Dict[str, Any]]] = [(-1, result)]
	for raw in text.splitlines():
		line = raw.rstrip()
		if not line or line.strip().startswith('#'):
			continue
		indent = len(raw) - len(raw.lstrip(' '))
		while stack and indent <= stack[-1][0]:
			stack.pop()
		parent = stack[-1][1]

		if ':' in line:
			key, val = line.strip().split(':', 1)
			key = key.strip()
			val = val.strip()
			if val == '' or val is None:
				node: Dict[str, Any] = {}
				parent[key] = node
				stack.append((indent, node))
			else:
				if val.startswith('"') and val.endswith('"'):
					parsed = val[1:-1]
				elif val.lower() in ('true', 'false'):
					parsed = val.lower() == 'true'
				else:
					try:
						if '.' in val:
							parsed = float(val)
						else:
							parsed = int(val)
					except Exception:
						parsed = val
				parent[key] = parsed
	return result


def load_config(config_path: str) -> Dict[str, Any]:
	text = read_text(config_path)
	if yaml is not None:
		try:
			return yaml.safe_load(text)
		except Exception:
			pass
	return naive_yaml_load(text)


def read_jsonl(path: str) -> List[Dict[str, Any]]:
	rows = []
	with open(path, 'r', encoding='utf-8') as f:
		for line in f:
			s = line.strip()
			if not s:
				continue
			rows.append(json.loads(s))
	return rows


def build_messages(system_prompt: str, user_prompt_tmpl: str, description: str, function_signature: str = '') -> List[Dict[str, str]]:
	user_content = user_prompt_tmpl.format(description=description, function_signature=function_signature)
	return [
		{"role": "system", "content": system_prompt.strip()},
		{"role": "user", "content": user_content.strip()},
	]


def extract_rust_block(text: str) -> str:
	if '```' not in text:
		return text
	start = text.find('```')
	if start == -1:
		return text
	newline_after = text.find('\n', start + 3)
	if newline_after == -1:
		return text
	end = text.find('```', newline_after + 1)
	if end == -1:
		return text
	return text[newline_after + 1:end]


def ensure_chat_endpoint(base_url: str) -> str:
	if base_url.rstrip('/').endswith('/chat/completions'):
		return base_url
	return base_url.rstrip('/') + '/chat/completions'


def chat_completion(
	*,
	endpoint: str,
	api_key: str,
	model: str,
	messages: List[Dict[str, str]],
	temperature: float,
	max_tokens: int,
	n: int,
	seed: Optional[int] = None,
	timeout: int = 120,
) -> Dict[str, Any]:
	payload: Dict[str, Any] = {
		"model": model,
		"messages": messages,
		"temperature": temperature,
		"max_tokens": max_tokens,
		"n": n,
	}
	if seed is not None:
		payload["seed"] = seed

	data = json.dumps(payload).encode('utf-8')
	headers = {
		"Content-Type": "application/json",
		"Authorization": f"Bearer {api_key}",
	}

	req = urllib.request.Request(endpoint, data=data, headers=headers, method='POST')
	try:
		ctx = ssl.create_default_context()
		ctx.check_hostname = False
		ctx.verify_mode = ssl.CERT_NONE

		with urllib.request.urlopen(req, timeout=timeout, context=ctx) as resp:
			resp_text = resp.read().decode('utf-8')
			return json.loads(resp_text)
	except urllib.error.HTTPError as e:
		body = e.read().decode('utf-8', errors='ignore') if hasattr(e, 'read') else ''
		return {"error": {"status": e.code, "reason": e.reason, "body": body}}
	except Exception as e:
		return {"error": {"message": str(e)}}


FEWSHOT_FILES = {
	'exp1description': 'exp1description.txt',
	'exp1function_signature': 'exp1function_signature.txt',
	'exp1verus': 'exp1verus.rs',
	'exp1spec': 'exp1spec.rs',
	'exp1code': 'exp1code.rs',
	'exp2description': 'exp2description.txt',
	'exp2function_signature': 'exp2function_signature.txt',
	'exp2verus': 'exp2verus.rs',
	'exp2spec': 'exp2spec.rs',
	'exp2code': 'exp2code.rs',
}


def load_fewshot_replacements() -> Dict[str, str]:
	mapping: Dict[str, str] = {}
	for key, fname in FEWSHOT_FILES.items():
		path = os.path.join(FEWSHOT_DIR, fname)
		if os.path.exists(path):
			mapping[key] = read_text(path).strip()
	return mapping


def apply_replacements(text: str, mapping: Dict[str, str]) -> str:
	out = text
	for key, val in mapping.items():
		out = out.replace(f"<{key}>", val)
	return out


def safe_format(template: str, **kwargs: str) -> str:
	"""
	Safely format a template with user-provided text by avoiding brace conflicts
	inside the template itself (e.g., Verus code snippets with {}).
	"""
	placeholders: Dict[str, str] = {}
	for key, val in kwargs.items():
		ph = f"__PLACEHOLDER_{key}__"
		placeholders[key] = ph
		template = template.replace(f"{{{key}}}", ph)

	# Escape any remaining braces in the template
	template = template.replace('{', '{{').replace('}', '}}')

	# Restore placeholders with raw values
	for key, ph in placeholders.items():
		val = kwargs.get(key, '')
		if not isinstance(val, str):
			val = str(val)
		template = template.replace(ph, val)

	return template


def find_missing_files(
	exp_dir: str,
	model_name: str,
	pipeline: str,
	dir_name: str,
	*,
	pass_k: int,
	use_model_subdir: bool,
	task_ids: Optional[List[str]] = None,
) -> Dict[str, List[int]]:
	"""
	扫描 exp_dir 下所有任务的 {dir_name}/{model_name}/ 目录，
	返回每个任务缺失的文件序号（0-4）
	"""
	config = PIPELINE_CONFIG[pipeline]

	missing = {}
	iter_ids = task_ids if task_ids is not None else os.listdir(exp_dir)
	for task_id in iter_ids:
		if use_model_subdir:
			task_path = os.path.join(exp_dir, task_id, dir_name, model_name)
		else:
			task_path = os.path.join(exp_dir, task_id, dir_name)
		if not os.path.isdir(task_path):
			missing[task_id] = list(range(pass_k))
			continue
		
		existing_files = set(os.listdir(task_path))
		existing_indices = set()
		
		if config['steps'] == 1:
			# P1: 检查 00.rs ~ 04.rs
			for idx in range(pass_k):
				fname = config['file_pattern'].format(idx=idx)
				if fname in existing_files:
					existing_indices.add(idx)
		else:
			# P2/P3: 检查配对文件，只有两个文件都存在才算完整
			for idx in range(pass_k):
				file1 = config['file_patterns'][0].format(idx=idx)
				file2 = config['file_patterns'][1].format(idx=idx)
				if file1 in existing_files and file2 in existing_files:
					existing_indices.add(idx)
		
		expected = set(range(pass_k))
		missing_indices = sorted(expected - existing_indices)
		if missing_indices:
			missing[task_id] = missing_indices
	
	return missing


def main(argv: Optional[List[str]] = None) -> int:
	parser = argparse.ArgumentParser(description='通用补跑脚本：补跑 p1/p2/p3 实验缺失的 Pass@5 文件')
	parser.add_argument('--pipeline', choices=['p1', 'p2', 'p3'], default='p1', help='Pipeline type: p1, p2, or p3')
	parser.add_argument('--variant', choices=['fewshot+LTM', 'fewshot', 'fewshot+COT', 'zeroshot'], default='fewshot+LTM', help='Experiment variant: fewshot+LTM, fewshot, fewshot+COT, or zeroshot')
	parser.add_argument('--config', default=os.path.join(WORKDIR, 'config', 'config.yaml'), help='Path to config.yaml')
	parser.add_argument('--dataset', default=os.path.join(WORKDIR, 'dataset', 'NL2VBench.jsonl'), help='Path to dataset jsonl')
	parser.add_argument('--exp-dir', default=os.path.join(WORKDIR, 'generation'), help='Experiment generation directory')
	parser.add_argument('--model', default='deepseek-v3.2', help='Model key in config.models to use')
	parser.add_argument('--timeout', type=int, default=300, help='HTTP timeout seconds')
	parser.add_argument('--max-tokens', type=int, default=4096, help='Override chat_max_tokens for this run')
	parser.add_argument('--pass-k', type=int, default=5, help='Number of generations per sample to backfill (default: 5 for pass@5)')
	parser.add_argument('--workers', type=int, default=8, help='Number of concurrent workers')
	parser.add_argument('--log-file', default=None, help='Write logs to a file to avoid breaking the progress bar')

	args = parser.parse_args(argv)

	# Reconfigure logging after args parsing (keep progress bar readable)
	if args.log_file:
		logging.basicConfig(
			level=logging.INFO,
			format='%(asctime)s - %(levelname)s - %(message)s',
			filename=args.log_file,
			filemode='a',
			force=True,
		)
	else:
		logging.basicConfig(
			level=logging.INFO,
			format='%(asctime)s - %(levelname)s - %(message)s',
			force=True,
		)
	pipeline = args.pipeline
	variant = args.variant
	config = PIPELINE_CONFIG[pipeline]
	variant_cfg = VARIANT_CONFIG[variant]
	dir_name = variant_cfg['dir_names'][pipeline]
	prompt_file = variant_cfg['prompt_files'][pipeline]
	pass_k = int(args.pass_k)
	use_model_subdir = True

	cfg = load_config(args.config)
	models = cfg.get('models', {})
	if args.model not in models:
		print(f"Model '{args.model}' not found in config.models", file=sys.stderr)
		return 2

	m = models[args.model]
	api_key = m.get('api_key')
	base_url = m.get('base_url')
	model_name = m.get('model_name') or args.model
	max_n = int(m.get('max_n', 1))

	gen = cfg.get('generation', {})
	chat_max_tokens = int(gen.get('chat_max_tokens', 2048))
	temperature = float(gen.get('temperature', 0))
	seed = gen.get('seed')
	try:
		seed = int(seed) if seed is not None else None
	except Exception:
		seed = None
	tokens_cap = int(gen.get('max_tokens_cap', 8192))

	if not api_key or not base_url:
		print('api_key/base_url missing for selected model', file=sys.stderr)
		return 2

	endpoint = ensure_chat_endpoint(str(base_url))

	# Load prompts with optional few-shot replacements
	prompt_path = os.path.join(WORKDIR, 'prompt', prompt_file)
	ns: Dict[str, Any] = {}
	with open(prompt_path, 'r', encoding='utf-8') as f:
		exec(f.read(), ns, ns)

	fewshot_map = load_fewshot_replacements() if variant_cfg['use_fewshot_replacements'] else {}
	
	# 根据 pipeline 类型加载不同的 prompt
	if pipeline == 'p1':
		system_prompt: str = apply_replacements(ns['SYSTEM_PROMPT'], fewshot_map)
		user_prompt_tmpl: str = apply_replacements(ns['USER_PROMPT'], fewshot_map)
	elif pipeline == 'p2':
		system_prompt_code: str = apply_replacements(ns['SYSTEM_PROMPT_CODE'], fewshot_map)
		user_prompt_code_tmpl: str = apply_replacements(ns['USER_PROMPT_CODE'], fewshot_map)
		system_prompt_spec: str = apply_replacements(ns['SYSTEM_PROMPT_SPEC'], fewshot_map)
		user_prompt_spec_tmpl: str = apply_replacements(ns['USER_PROMPT_SPEC'], fewshot_map)
	else:  # p3
		system_prompt_spec: str = apply_replacements(ns['SYSTEM_PROMPT_SPEC_ONLY'], fewshot_map)
		user_prompt_spec_tmpl: str = apply_replacements(ns['USER_PROMPT_SPEC_ONLY'], fewshot_map)
		system_prompt_code: str = apply_replacements(ns['SYSTEM_PROMPT_CODE_FROM_SPEC'], fewshot_map)
		user_prompt_code_tmpl: str = apply_replacements(ns['SYSTEM_PROMPT_CODE_FROM_SPEC'], fewshot_map)

	# Load dataset and create lookup
	all_samples = read_jsonl(args.dataset)
	sample_lookup = {row['sample_id']: row for row in all_samples if 'sample_id' in row}

	# Find missing files (use dataset task ids to catch missing directories)
	missing_files = find_missing_files(
		args.exp_dir,
		model_name,
		pipeline,
		dir_name,
		pass_k=pass_k,
		use_model_subdir=use_model_subdir,
		task_ids=sorted(sample_lookup.keys()),
	)
	if not missing_files:
		print(f"No missing files found for {dir_name}. All Pass@5 outputs are complete!")
		return 0

	total_missing = sum(len(indices) for indices in missing_files.values())
	print(f"Found {len(missing_files)} tasks with {total_missing} missing files:")
	for task_id, indices in sorted(missing_files.items()):
		print(f"  {task_id}: missing {indices}")
	print()

	# Build task list
	tasks_to_run: List[Tuple[str, int, Dict[str, Any]]] = []
	for task_id, missing_indices in missing_files.items():
		if task_id not in sample_lookup:
			logging.warning(f"Task {task_id} not found in dataset, skipping")
			continue
		sample = sample_lookup[task_id]
		for idx in missing_indices:
			tasks_to_run.append((task_id, idx, sample))

	if not tasks_to_run:
		print("No valid tasks to run.")
		return 0

	cur_max_tokens = max(128, min(args.max_tokens, tokens_cap))
	cur_timeout = int(args.timeout)

	def get_model_dir(task_id: str) -> str:
		if use_model_subdir:
			return os.path.join(args.exp_dir, task_id, dir_name, model_name)
		return os.path.join(args.exp_dir, task_id, dir_name)

	def process_one_p1(task_tuple: Tuple[str, int, Dict[str, Any]]) -> Tuple[str, int, Optional[str]]:
		"""P1 pipeline: 单步生成完整 Verus 代码"""
		task_id, idx, sample = task_tuple
		description = sample.get('description', '')
		function_signature = sample.get('function_signature', '')
		
		msgs = build_messages(system_prompt, user_prompt_tmpl, description, function_signature)
		
		resp = chat_completion(
			endpoint=endpoint,
			api_key=str(api_key),
			model=str(model_name),
			messages=msgs,
			temperature=temperature,
			max_tokens=cur_max_tokens,
			n=min(1, max_n),
			seed=seed,
			timeout=cur_timeout,
		)

		if 'error' in resp:
			logging.error(f"API error for {task_id}[{idx}]: {resp['error']}")
			return (task_id, idx, None)

		choices = resp.get('choices', []) or []
		if not choices:
			logging.warning(f"Empty choices for {task_id}[{idx}]")
			return (task_id, idx, None)

		msg0 = choices[0].get('message') or {}
		content = (msg0.get('content') or '').strip()
		if not content:
			logging.warning(f"Empty content for {task_id}[{idx}]")
			return (task_id, idx, None)

		body = extract_rust_block(content).strip()

		# Write file
		model_dir = get_model_dir(task_id)
		os.makedirs(model_dir, exist_ok=True)
		fpath = os.path.join(model_dir, f"{idx:02d}.rs")

		header = (
			f"// Generated by 补跑脚本 at {datetime.now(timezone.utc).isoformat()}\n"
			f"// sample_id: {task_id}\n"
			f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
			f"// iteration: {idx}/{pass_k}\n\n"
		)
		with open(fpath, 'w', encoding='utf-8') as rf:
			rf.write(header)
			rf.write(body if body else content)

		return (task_id, idx, fpath)

	def process_one_p2(task_tuple: Tuple[str, int, Dict[str, Any]]) -> Tuple[str, int, Optional[str]]:
		"""P2 pipeline: 第一步生成 code，第二步生成 spec (complete)"""
		task_id, idx, sample = task_tuple
		description = sample.get('description', '')
		function_signature = sample.get('function_signature', '')
		
		# Step 1: 生成 code
		msgs_code = build_messages(system_prompt_code, user_prompt_code_tmpl, description, function_signature)
		resp_code = chat_completion(
			endpoint=endpoint,
			api_key=str(api_key),
			model=str(model_name),
			messages=msgs_code,
			temperature=temperature,
			max_tokens=cur_max_tokens,
			n=min(1, max_n),
			seed=seed,
			timeout=cur_timeout,
		)

		if 'error' in resp_code:
			logging.error(f"API error for {task_id}[{idx}] step1: {resp_code['error']}")
			return (task_id, idx, None)

		choices = resp_code.get('choices', []) or []
		if not choices:
			logging.warning(f"Empty choices for {task_id}[{idx}] step1")
			return (task_id, idx, None)

		msg0 = choices[0].get('message') or {}
		code_content = (msg0.get('content') or '').strip()
		if not code_content:
			logging.warning(f"Empty content for {task_id}[{idx}] step1")
			return (task_id, idx, None)

		code_body = extract_rust_block(code_content).strip()

		# Step 2: 生成 spec (complete) 基于 code
		user_content_spec = safe_format(
			user_prompt_spec_tmpl,
			description=description,
			function_signature=function_signature,
			code=code_body if code_body else code_content,
		)
		msgs_spec = [
			{"role": "system", "content": system_prompt_spec.strip()},
			{"role": "user", "content": user_content_spec.strip()},
		]
		resp_spec = chat_completion(
			endpoint=endpoint,
			api_key=str(api_key),
			model=str(model_name),
			messages=msgs_spec,
			temperature=temperature,
			max_tokens=cur_max_tokens,
			n=min(1, max_n),
			seed=seed,
			timeout=cur_timeout,
		)

		if 'error' in resp_spec:
			logging.error(f"API error for {task_id}[{idx}] step2: {resp_spec['error']}")
			return (task_id, idx, None)

		choices_spec = resp_spec.get('choices', []) or []
		if not choices_spec:
			logging.warning(f"Empty choices for {task_id}[{idx}] step2")
			return (task_id, idx, None)

		msg_spec = choices_spec[0].get('message') or {}
		spec_content = (msg_spec.get('content') or '').strip()
		if not spec_content:
			logging.warning(f"Empty content for {task_id}[{idx}] step2")
			return (task_id, idx, None)

		complete_body = extract_rust_block(spec_content).strip()

		# Write both files
		model_dir = get_model_dir(task_id)
		os.makedirs(model_dir, exist_ok=True)
		
		header = (
			f"// Generated by 补跑脚本 at {datetime.now(timezone.utc).isoformat()}\n"
			f"// sample_id: {task_id}\n"
			f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
			f"// iteration: {idx}/{pass_k}\n\n"
		)
		
		code_path = os.path.join(model_dir, f"code{idx:02d}.rs")
		complete_path = os.path.join(model_dir, f"complete{idx:02d}.rs")
		with open(code_path, 'w', encoding='utf-8') as f:
			f.write(header)
			f.write(code_body if code_body else code_content)
		with open(complete_path, 'w', encoding='utf-8') as f:
			f.write(header)
			f.write(complete_body if complete_body else spec_content)

		return (task_id, idx, f"{code_path}, {complete_path}")

	def process_one_p3(task_tuple: Tuple[str, int, Dict[str, Any]]) -> Tuple[str, int, Optional[str]]:
		"""P3 pipeline: 第一步生成 spec，第二步生成 code (complete)"""
		task_id, idx, sample = task_tuple
		description = sample.get('description', '')
		function_signature = sample.get('function_signature', '')
		
		# Step 1: 生成 spec
		msgs_spec = build_messages(system_prompt_spec, user_prompt_spec_tmpl, description, function_signature)
		resp_spec = chat_completion(
			endpoint=endpoint,
			api_key=str(api_key),
			model=str(model_name),
			messages=msgs_spec,
			temperature=temperature,
			max_tokens=cur_max_tokens,
			n=min(1, max_n),
			seed=seed,
			timeout=cur_timeout,
		)

		if 'error' in resp_spec:
			logging.error(f"API error for {task_id}[{idx}] step1: {resp_spec['error']}")
			return (task_id, idx, None)

		choices = resp_spec.get('choices', []) or []
		if not choices:
			logging.warning(f"Empty choices for {task_id}[{idx}] step1")
			return (task_id, idx, None)

		msg0 = choices[0].get('message') or {}
		spec_content = (msg0.get('content') or '').strip()
		if not spec_content:
			logging.warning(f"Empty content for {task_id}[{idx}] step1")
			return (task_id, idx, None)

		spec_body = extract_rust_block(spec_content).strip()

		# Step 2: 生成 code (complete) 基于 spec
		user_content_code = safe_format(
			user_prompt_code_tmpl,
			description=description,
			function_signature=function_signature,
			spec=spec_body if spec_body else spec_content,
		)
		msgs_code = [
			{"role": "system", "content": system_prompt_code.strip()},
			{"role": "user", "content": user_content_code.strip()},
		]
		resp_code = chat_completion(
			endpoint=endpoint,
			api_key=str(api_key),
			model=str(model_name),
			messages=msgs_code,
			temperature=temperature,
			max_tokens=cur_max_tokens,
			n=min(1, max_n),
			seed=seed,
			timeout=cur_timeout,
		)

		if 'error' in resp_code:
			logging.error(f"API error for {task_id}[{idx}] step2: {resp_code['error']}")
			return (task_id, idx, None)

		choices_code = resp_code.get('choices', []) or []
		if not choices_code:
			logging.warning(f"Empty choices for {task_id}[{idx}] step2")
			return (task_id, idx, None)

		msg_code = choices_code[0].get('message') or {}
		code_content = (msg_code.get('content') or '').strip()
		if not code_content:
			logging.warning(f"Empty content for {task_id}[{idx}] step2")
			return (task_id, idx, None)

		complete_body = extract_rust_block(code_content).strip()

		# Write both files
		model_dir = get_model_dir(task_id)
		os.makedirs(model_dir, exist_ok=True)
		
		header = (
			f"// Generated by 补跑脚本 at {datetime.now(timezone.utc).isoformat()}\n"
			f"// sample_id: {task_id}\n"
			f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
			f"// iteration: {idx}/{pass_k}\n\n"
		)
		
		spec_path = os.path.join(model_dir, f"spec{idx:02d}.rs")
		complete_path = os.path.join(model_dir, f"complete{idx:02d}.rs")
		with open(spec_path, 'w', encoding='utf-8') as f:
			f.write(header)
			f.write(spec_body if spec_body else spec_content)
		with open(complete_path, 'w', encoding='utf-8') as f:
			f.write(header)
			f.write(complete_body if complete_body else code_content)

		return (task_id, idx, f"{spec_path}, {complete_path}")

	# 选择对应的处理函数
	process_one = {'p1': process_one_p1, 'p2': process_one_p2, 'p3': process_one_p3}[pipeline]

	logging.info(f"Starting補跑 {len(tasks_to_run)} missing files with {args.workers} workers")
	logging.info(f"Pipeline: {dir_name}, Variant: {variant}, Model: {model_name}, Pass@K: {pass_k}, Max tokens: {cur_max_tokens}, Timeout: {cur_timeout}s")

	results: List[Tuple[str, int, Optional[str]]] = []
	with ThreadPoolExecutor(max_workers=max(1, args.workers)) as ex:
		futures = {ex.submit(process_one, task): task for task in tasks_to_run}
		with tqdm(
			total=len(tasks_to_run),
			desc=f"補跑缺失文件 ({dir_name})",
			unit="file",
			file=sys.stdout,
			dynamic_ncols=True,
			leave=True,
		) as pbar:
			for fut in as_completed(futures):
				task_id, idx, path = fut.result()
				results.append((task_id, idx, path))
				pbar.update(1)

	success = sum(1 for _, _, p in results if p)
	failed = len(results) - success

	print("\n" + "=" * 50)
	print("補跑完成")
	print("=" * 50)
	print(f"Pipeline: {dir_name}")
	print(f"成功: {success}/{len(results)}")
	print(f"失败: {failed}")
	print("=" * 50)

	return 0


if __name__ == '__main__':
	sys.exit(main())
