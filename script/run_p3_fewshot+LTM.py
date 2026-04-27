#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Pipeline-3 (Few-shot + LTM): Two-step generation (Spec + Code from Spec) using few-shot exemplars with Long-Term Memory prompting.
Matches the style and output structure of p3_fewshot with LTM prompts.
"""

import argparse
import json
import os
import sys
import time
import logging
from datetime import datetime, timezone
from typing import Any, Dict, Iterable, List, Optional, Tuple
from concurrent.futures import ThreadPoolExecutor, as_completed
from tqdm import tqdm

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

try:
	import yaml  # type: ignore
except Exception:  # pragma: no cover
	yaml = None

import urllib.request
import urllib.error
import ssl

WORKDIR = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
FEWSHOT_DIR = os.path.join(WORKDIR, 'dataset', 'fewshot')


# ---------- IO helpers ----------

def read_text(path: str) -> str:
	with open(path, 'r', encoding='utf-8') as f:
		return f.read()


def naive_yaml_load(text: str) -> Dict[str, Any]:
	"""
	Minimal indentation-based YAML loader for simple key/value + 1-2 levels nesting.
	Falls back when PyYAML is unavailable. Tailored for current config.yaml layout.
	"""
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
			return yaml.safe_load(text)  # type: ignore
		except Exception:
			pass
	return naive_yaml_load(text)


def read_jsonl(path: str) -> Iterable[Dict[str, Any]]:
	with open(path, 'r', encoding='utf-8') as f:
		for line in f:
			s = line.strip()
			if not s:
				continue
			yield json.loads(s)


def build_messages(system_prompt: str, user_prompt_tmpl: str, **kwargs) -> List[Dict[str, str]]:
	user_content = user_prompt_tmpl.format(**kwargs)
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


# ---------- Few-shot helpers ----------

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


# ---------- Main ----------


def main(argv: Optional[List[str]] = None) -> int:
	parser = argparse.ArgumentParser(description='Run Pipeline-3 (Few-shot + LTM): Two-step Spec + Code from Spec (pass@5)')
	parser.add_argument('--config', default=os.path.join(WORKDIR, 'config', 'config.yaml'), help='Path to config.yaml')
	parser.add_argument('--dataset', default=os.path.join(WORKDIR, 'dataset', 'NL2VBench.jsonl'), help='Path to dataset jsonl')
	parser.add_argument('--out-dir', default=os.path.join(WORKDIR, 'generation'), help='Base directory for experiment outputs (default: generation)')
	parser.add_argument('--limit', type=int, default=None, help='Limit number of samples')
	parser.add_argument('--model', default='gpt-5.2', help='Model key in config.models to use')
	parser.add_argument('--timeout', type=int, default=300, help='HTTP timeout seconds')
	parser.add_argument('--max-tokens', type=int, default=4096, help='Override chat_max_tokens for this run')
	parser.add_argument('--dry-run', action='store_true', help='Do not call API; just print prompts and exit')
	parser.add_argument('--workers', type=int, default=8, help='Number of concurrent workers (default: 8)')
	parser.add_argument('--pass-k', type=int, default=5, help='Number of generations per sample (default: 5 for pass@5)')

	args = parser.parse_args(argv)

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
	
	print(f"Using API endpoint: {endpoint}", flush=True)
	print(f"Using model: {model_name}\n", flush=True)

	# Load prompts with few-shot replacements
	prompt_path = os.path.join(WORKDIR, 'prompt', 'p3_fewshot+LTM.py')
	ns: Dict[str, Any] = {}
	with open(prompt_path, 'r', encoding='utf-8') as f:
		exec(f.read(), ns, ns)
	
	system_prompt_spec: str = ns['SYSTEM_PROMPT_SPEC_ONLY']
	user_prompt_spec_tmpl: str = ns['USER_PROMPT_SPEC_ONLY']
	system_prompt_code: str = ns['SYSTEM_PROMPT_CODE_FROM_SPEC']
	user_prompt_code_tmpl: str = ns['USER_PROMPT_CODE_FROM_SPEC']

	fewshot_map = load_fewshot_replacements()
	system_prompt_spec = apply_replacements(system_prompt_spec, fewshot_map)
	user_prompt_spec_tmpl = apply_replacements(user_prompt_spec_tmpl, fewshot_map)
	system_prompt_code = apply_replacements(system_prompt_code, fewshot_map)
	user_prompt_code_tmpl = apply_replacements(user_prompt_code_tmpl, fewshot_map)

	# Load and slice dataset
	samples: List[Dict[str, Any]] = []
	for row in read_jsonl(args.dataset):
		if not row.get('description'):
			continue
		samples.append(row)
		if args.limit is not None and len(samples) >= args.limit:
			break

	if not samples:
		print('No samples to process (check dataset/limit).', file=sys.stderr)
		return 1

	if args.dry_run:
		for i, row in enumerate(samples[:1]):  # Just show first sample
			sample_id = row.get('sample_id')
			description = row.get('description')
			function_signature = row.get('function_signature', '')
			
			print('---- STEP 1: SPEC GENERATION (dry-run) ----')
			msgs = build_messages(system_prompt_spec, user_prompt_spec_tmpl, description=description, function_signature=function_signature)
			print('sample_id:', sample_id)
			print('messages:', json.dumps(msgs, ensure_ascii=False, indent=2))
			
			print('\n---- STEP 2: CODE FROM SPEC (dry-run) ----')
			print('(would use generated spec from step 1)')
			msgs = build_messages(system_prompt_code, user_prompt_code_tmpl, function_signature=function_signature, spec="<generated_spec>")
			print('messages:', json.dumps(msgs, ensure_ascii=False, indent=2))
			
			sample_dir = os.path.join(args.out_dir, str(sample_id), 'p3_fewshot+LTM', model_name)
			print(f'\nwould write to:')
			for k in range(args.pass_k):
				print(f'  {os.path.join(sample_dir, f"spec{k:02d}.rs")}')
				print(f'  {os.path.join(sample_dir, f"complete{k:02d}.rs")}')
		print(f"\nDry run complete. Prepared {len(samples)} samples with pass@{args.pass_k}.")
		return 0

	# Determine max_tokens
	base_max_tokens = int(args.max_tokens) if args.max_tokens is not None else chat_max_tokens
	cur_max_tokens = max(128, min(base_max_tokens, tokens_cap))
	cur_timeout = int(args.timeout)

	# Per-sample processing function
	def process_one(row: Dict[str, Any]) -> Tuple[str, List[Optional[str]], int, float]:
		sample_id = row.get('sample_id')
		description = row.get('description')
		function_signature = row.get('function_signature', '')
		
		start_time = time.time()
		total_chars = 0
		paths_written: List[Optional[str]] = []
		
		sample_dir = os.path.join(args.out_dir, str(sample_id), 'p3_fewshot+LTM', model_name)
		os.makedirs(sample_dir, exist_ok=True)
		
		for k in range(args.pass_k):
			# ===== STEP 1: Generate Spec =====
			msgs_spec = build_messages(system_prompt_spec, user_prompt_spec_tmpl,
										 description=description, function_signature=function_signature)
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
			
			# Check for errors in spec generation
			if 'error' in resp_spec:
				logger.error(f"API error (spec generation) for {sample_id} (iteration {k}): {resp_spec['error']}")
				paths_written.append(None)
				continue
			
			choices = resp_spec.get('choices', []) or []
			if not choices:
				logger.warning(f"Empty choices in spec generation for {sample_id} (iteration {k})")
				paths_written.append(None)
				continue
			
			msg_spec = choices[0].get('message') or {}
			content_spec = (msg_spec.get('content') or '').strip()
			if not content_spec:
				logger.warning(f"Empty content in spec generation for {sample_id} (iteration {k})")
				paths_written.append(None)
				continue
			
			body_spec = extract_rust_block(content_spec).strip()
			total_chars += len(content_spec)
			
			# ===== STEP 2: Generate Code from Spec =====
			msgs_code = build_messages(system_prompt_code, user_prompt_code_tmpl,
										 function_signature=function_signature, spec=body_spec)
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
			
			# Determine file names
			spec_fname = f"spec{k:02d}.rs"
			complete_fname = f"complete{k:02d}.rs"
			
			spec_file = os.path.join(sample_dir, spec_fname)
			complete_file = os.path.join(sample_dir, complete_fname)
			
			# Write spec file
			header_spec = (
				f"// Generated by p3_fewshot+LTM (spec) at {datetime.now(timezone.utc).isoformat()}\n"
				f"// sample_id: {sample_id}\n"
				f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
				f"// iteration: {k}/{args.pass_k}\n\n"
			)
			with open(spec_file, 'w', encoding='utf-8') as f:
				f.write(header_spec)
				f.write(body_spec if body_spec else content_spec)
			
			# Check for errors in code generation
			if 'error' in resp_code:
				logger.error(f"API error (code generation) for {sample_id} (iteration {k}): {resp_code['error']}")
				# Write empty complete file to maintain pairing
				header_complete = (
					f"// Generated by p3_fewshot+LTM (code FAILED) at {datetime.now(timezone.utc).isoformat()}\n"
					f"// sample_id: {sample_id}\n"
					f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
					f"// iteration: {k}/{args.pass_k}\n"
					f"// ERROR: code generation failed\n\n"
				)
				with open(complete_file, 'w', encoding='utf-8') as f:
					f.write(header_complete)
					f.write(body_spec if body_spec else content_spec)  # Use spec as fallback
				paths_written.append(spec_file)
				continue
			
			choices_code = resp_code.get('choices', []) or []
			if not choices_code:
				logger.warning(f"Empty choices in code generation for {sample_id} (iteration {k})")
				header_complete = (
					f"// Generated by p3_fewshot+LTM (code FAILED) at {datetime.now(timezone.utc).isoformat()}\n"
					f"// sample_id: {sample_id}\n"
					f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
					f"// iteration: {k}/{args.pass_k}\n"
					f"// ERROR: empty choices in code\n\n"
				)
				with open(complete_file, 'w', encoding='utf-8') as f:
					f.write(header_complete)
					f.write(body_spec if body_spec else content_spec)
				paths_written.append(spec_file)
				continue
			
			msg_code = choices_code[0].get('message') or {}
			content_code = (msg_code.get('content') or '').strip()
			if not content_code:
				logger.warning(f"Empty content in code generation for {sample_id} (iteration {k})")
				header_complete = (
					f"// Generated by p3_fewshot+LTM (code FAILED) at {datetime.now(timezone.utc).isoformat()}\n"
					f"// sample_id: {sample_id}\n"
					f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
					f"// iteration: {k}/{args.pass_k}\n"
					f"// ERROR: empty content in code\n\n"
				)
				with open(complete_file, 'w', encoding='utf-8') as f:
					f.write(header_complete)
					f.write(body_spec if body_spec else content_spec)
				paths_written.append(spec_file)
				continue
			
			body_code = extract_rust_block(content_code).strip()
			total_chars += len(content_code)
			
			# Write complete file (spec + code)
			header_complete = (
				f"// Generated by p3_fewshot+LTM (spec + code) at {datetime.now(timezone.utc).isoformat()}\n"
				f"// sample_id: {sample_id}\n"
				f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
				f"// iteration: {k}/{args.pass_k}\n\n"
			)
			with open(complete_file, 'w', encoding='utf-8') as f:
				f.write(header_complete)
				f.write(body_code if body_code else content_code)
			
			paths_written.append(complete_file)
		
		duration = time.time() - start_time
		return (str(sample_id), paths_written, total_chars, duration)

	logging.info(f"Starting processing of {len(samples)} samples with {args.workers} workers")
	logging.info(f"Model: {model_name}, Max tokens: {cur_max_tokens}, Timeout: {cur_timeout}s")
	logging.info(f"Pass@K: {args.pass_k} (generating {args.pass_k} outputs per sample)")

	results: List[Tuple[str, List[Optional[str]], int, float]] = []
	total_chars_all = 0
	total_duration = 0.0
	global_start_time = time.time()

	with ThreadPoolExecutor(max_workers=max(1, args.workers)) as ex:
		futures = {ex.submit(process_one, row): row for row in samples}
		with tqdm(total=len(samples), desc="Processing samples", unit="sample") as pbar:
			for fut in as_completed(futures):
				sid, paths, chars, duration = fut.result()
				results.append((sid, paths, chars, duration))
				total_chars_all += chars
				total_duration += duration
				pbar.update(1)

	total_wall_time = time.time() - global_start_time
	ok_samples = sum(1 for _, paths, _, _ in results if any(p for p in paths))
	total_outputs = sum(len([p for p in paths if p]) for _, paths, _, _ in results)
	failed_count = len(results) - ok_samples
	avg_time_per_sample = total_duration / len(results) if results else 0
	parallel_efficiency = (total_duration / total_wall_time) if total_wall_time > 0 else 0

	print("\n" + "=" * 58)
	print("PROCESSING SUMMARY (P3: FEW-SHOT + LTM)")
	print("=" * 58)
	print(f"Total Samples Processed: {ok_samples}/{len(samples)}")
	print(f"Total Outputs Generated: {total_outputs} (pass@{args.pass_k})")
	print(f"Failed Samples:          {failed_count}")
	print(f"Total Generated Chars:   {total_chars_all:,}")
	print("-" * 58)
	print(f"Total Wall Time:         {total_wall_time:.2f} s")
	print(f"Total Compute Time:      {total_duration:.2f} s")
	print(f"Avg Time per Sample:     {avg_time_per_sample:.2f} s")
	print(f"Parallel Efficiency:     {parallel_efficiency:.2f}x")
	print("=" * 58)
	print(f"Output saved to: {args.out_dir}")
	print("=" * 58)

	return 0


if __name__ == '__main__':
	sys.exit(main())
