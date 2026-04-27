#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Pipeline-1 (Few-shot + LTM): generate Verus code using few-shot exemplars with Long-Term Memory prompting.
Matches the style and output structure of p1_fewshot with LTM prompts.
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


def write_jsonl(path: str, rows: Iterable[Dict[str, Any]]) -> None:
	os.makedirs(os.path.dirname(path), exist_ok=True)
	with open(path, 'w', encoding='utf-8') as f:
		for row in rows:
			f.write(json.dumps(row, ensure_ascii=False) + '\n')


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
	parser = argparse.ArgumentParser(description='Run Pipeline-1 (Few-shot + LTM) with Verus exemplars (pass@5)')
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
	prompt_path = os.path.join(WORKDIR, 'prompt', 'p1_fewshot+LTM.py')
	ns: Dict[str, Any] = {}
	with open(prompt_path, 'r', encoding='utf-8') as f:
		exec(f.read(), ns, ns)
	system_prompt: str = ns['SYSTEM_PROMPT']
	user_prompt_tmpl: str = ns['USER_PROMPT']

	fewshot_map = load_fewshot_replacements()
	system_prompt = apply_replacements(system_prompt, fewshot_map)
	user_prompt_tmpl = apply_replacements(user_prompt_tmpl, fewshot_map)

	# Load dataset
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
		for row in samples:
			sample_id = row.get('sample_id')
			description = row.get('description')
			function_signature = row.get('function_signature', '')
			msgs = build_messages(system_prompt, user_prompt_tmpl, description, function_signature)
			print('---- SAMPLE (dry-run) ----')
			print('sample_id:', sample_id)
			print('messages:', json.dumps(msgs, ensure_ascii=False, indent=2))
			sample_dir = os.path.join(args.out_dir, str(sample_id), 'p1_fewshot+LTM')
			model_dir = os.path.join(sample_dir, model_name)
			for k in range(args.pass_k):
				output_file = os.path.join(model_dir, f"{k:02d}.rs")
				print(f'would write to: {output_file}')
		print(f"Dry run complete. Prepared {len(samples)} messages with pass@{args.pass_k}.")
		return 0

	base_max_tokens = int(args.max_tokens) if args.max_tokens is not None else chat_max_tokens
	cur_max_tokens = max(128, min(base_max_tokens, tokens_cap))
	cur_timeout = int(args.timeout)

	def process_one(row: Dict[str, Any]) -> Tuple[str, List[Optional[str]], int, float]:
		sample_id = row.get('sample_id')
		description = row.get('description')
		function_signature = row.get('function_signature', '')
		msgs = build_messages(system_prompt, user_prompt_tmpl, description, function_signature)

		paths_written: List[Optional[str]] = []
		total_chars = 0
		start_time = time.time()

		for k in range(args.pass_k):
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
				logging.error(f"API error for {sample_id} (iteration {k}): {resp['error']}")
				paths_written.append(None)
				continue

			choices = resp.get('choices', []) or []
			if not choices:
				logging.warning(f"Empty choices in response for {sample_id} (iteration {k})")
				paths_written.append(None)
				continue

			msg0 = choices[0].get('message') or {}
			content = (msg0.get('content') or '').strip()
			if not content:
				logging.warning(f"Empty content in message for {sample_id} (iteration {k})")
				paths_written.append(None)
				continue

			body = extract_rust_block(content).strip()
			total_chars += len(content)

			# Determine output path
			sample_dir = os.path.join(args.out_dir, str(sample_id), 'p1_fewshot+LTM')
			model_dir = os.path.join(sample_dir, model_name)
			os.makedirs(model_dir, exist_ok=True)
			fname = f"{k:02d}.rs"
			fpath = os.path.join(model_dir, fname)

			header = (
				f"// Generated by p1_fewshot+LTM at {datetime.now(timezone.utc).isoformat()}\n"
				f"// sample_id: {sample_id}\n"
				f"// model: {model_name}, temperature: {temperature}, seed: {seed}\n"
				f"// iteration: {k}/{args.pass_k}\n\n"
			)
			with open(fpath, 'w', encoding='utf-8') as rf:
				rf.write(header)
				rf.write(body if body else content)

			paths_written.append(fpath)

		duration = time.time() - start_time
		return (str(sample_id), paths_written, total_chars, duration)

	logging.info(f"Starting processing of {len(samples)} samples with {args.workers} workers")
	logging.info(f"Model: {model_name}, Max tokens: {cur_max_tokens}, Timeout: {cur_timeout}s")
	logging.info(f"Pass@K: {args.pass_k} (generating {args.pass_k} outputs per sample)")

	results: List[Tuple[str, List[Optional[str]], int, float]] = []
	total_chars = 0
	total_duration = 0.0
	global_start_time = time.time()

	with ThreadPoolExecutor(max_workers=max(1, args.workers)) as ex:
		futures = {ex.submit(process_one, row): row for row in samples}
		with tqdm(total=len(samples), desc="Processing samples", unit="sample") as pbar:
			for fut in as_completed(futures):
				sid, paths, chars, duration = fut.result()
				results.append((sid, paths, chars, duration))
				total_chars += chars
				total_duration += duration
				pbar.update(1)

	total_wall_time = time.time() - global_start_time
	ok_samples = sum(1 for _, paths, _, _ in results if any(p for p in paths))
	total_outputs = sum(len([p for p in paths if p]) for _, paths, _, _ in results)
	failed_count = len(results) - ok_samples
	avg_time_per_sample = total_duration / len(results) if results else 0
	parallel_efficiency = (total_duration / total_wall_time) if total_wall_time > 0 else 0

	print("\n" + "=" * 50)
	print("PROCESSING SUMMARY (P1: FEW-SHOT + LTM)")
	print("=" * 50)
	print(f"Total Samples Processed: {ok_samples}/{len(samples)}")
	print(f"Total Outputs Generated: {total_outputs} (pass@{args.pass_k})")
	print(f"Failed Samples:          {failed_count}")
	print(f"Total Generated Chars:   {total_chars:,}")
	print("-" * 50)
	print(f"Total Wall Time:         {total_wall_time:.2f} s")
	print(f"Total Compute Time:      {total_duration:.2f} s")
	print(f"Avg Time per Sample:     {avg_time_per_sample:.2f} s")
	print(f"Parallel Efficiency:     {parallel_efficiency:.2f}x")
	print("=" * 50)
	print(f"Output saved to: {args.out_dir}")
	print("=" * 50)

	return 0


if __name__ == '__main__':
	sys.exit(main())
