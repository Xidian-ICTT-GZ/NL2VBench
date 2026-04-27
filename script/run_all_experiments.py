#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
One-click experiment launcher for NL2VerusBench.

Default behavior:
- pipelines: p1, p2, p3
- variants: zeroshot, fewshot, fewshot+COT, fewshot+LTM
- models: all models declared in config/config.yaml
- generation count: pass@5 (passed explicitly to the underlying scripts)

It can also run in --dry-run mode to print the command matrix without
calling any model APIs.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional

try:
    import yaml  # type: ignore
except Exception:  # pragma: no cover
    yaml = None

ROOT = Path(__file__).resolve().parents[1]
SCRIPT_DIR = ROOT / "script"

PIPELINE_VARIANT_TO_SCRIPT = {
    "p1": {
        "zeroshot": "run_p1_zeroshot.py",
        "fewshot": "run_p1_fewshot.py",
        "fewshot+COT": "run_p1_fewshot+COT.py",
        "fewshot+LTM": "run_p1_fewshot+LTM.py",
    },
    "p2": {
        "zeroshot": "run_p2_zeroshot.py",
        "fewshot": "run_p2_fewshot.py",
        "fewshot+COT": "run_p2_fewshot+COT.py",
        "fewshot+LTM": "run_p2_fewshot+LTM.py",
    },
    "p3": {
        "zeroshot": "run_p3_zeroshot.py",
        "fewshot": "run_p3_fewshot.py",
        "fewshot+COT": "run_p3_fewshot+COT.py",
        "fewshot+LTM": "run_p3_fewshot+LTM.py",
    },
}

ALL_PIPELINES = ["p1", "p2", "p3"]
ALL_VARIANTS = ["zeroshot", "fewshot", "fewshot+COT", "fewshot+LTM"]


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def naive_yaml_load(text: str) -> Dict[str, Any]:
    result: Dict[str, Any] = {}
    stack: List[tuple[int, Dict[str, Any]]] = [(-1, result)]
    for raw in text.splitlines():
        line = raw.rstrip()
        if not line or line.strip().startswith("#"):
            continue
        indent = len(raw) - len(raw.lstrip(" "))
        while stack and indent <= stack[-1][0]:
            stack.pop()
        parent = stack[-1][1]
        if ":" not in line:
            continue
        key, val = line.strip().split(":", 1)
        key = key.strip()
        val = val.strip()
        if val == "":
            node: Dict[str, Any] = {}
            parent[key] = node
            stack.append((indent, node))
        else:
            if val.startswith('"') and val.endswith('"'):
                parsed: Any = val[1:-1]
            elif val.lower() in ("true", "false"):
                parsed = val.lower() == "true"
            else:
                try:
                    parsed = int(val)
                except Exception:
                    try:
                        parsed = float(val)
                    except Exception:
                        parsed = val
            parent[key] = parsed
    return result


def load_config(config_path: Path) -> Dict[str, Any]:
    text = read_text(config_path)
    if yaml is not None:
        try:
            loaded = yaml.safe_load(text)  # type: ignore[attr-defined]
            return loaded or {}
        except Exception:
            pass
    return naive_yaml_load(text)


def get_models(config_path: Path) -> List[str]:
    cfg = load_config(config_path)
    models = cfg.get("models", {}) if isinstance(cfg, dict) else {}
    if isinstance(models, dict):
        return list(models.keys())
    return []


def build_command(
    script_name: str,
    *,
    config: Path,
    dataset: Path,
    out_dir: Path,
    model: str,
    workers: int,
    timeout: int,
    max_tokens: int,
    limit: Optional[int],
) -> List[str]:
    script_path = SCRIPT_DIR / script_name
    cmd = [
        sys.executable,
        str(script_path),
        "--config",
        str(config),
        "--dataset",
        str(dataset),
        "--out-dir",
        str(out_dir),
        "--model",
        model,
        "--workers",
        str(workers),
        "--timeout",
        str(timeout),
        "--max-tokens",
        str(max_tokens),
    ]
    if limit is not None:
        cmd += ["--limit", str(limit)]

    # Explicit pass@5 for reproducibility. LTM uses --pass-k, others use --pass-at.
    if "fewshot+LTM" in script_name:
        cmd += ["--pass-k", "5"]
    else:
        cmd += ["--pass-at", "5"]

    return cmd


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="One-click batch launcher for NL2VerusBench experiments")
    parser.add_argument("--config", default=str(ROOT / "config" / "config.yaml"), help="Path to config.yaml")
    parser.add_argument("--dataset", default=str(ROOT / "dataset" / "NL2VBench.jsonl"), help="Path to dataset jsonl")
    parser.add_argument("--out-dir", default=str(ROOT / "generation"), help="Base directory for experiment outputs")
    parser.add_argument("--pipelines", nargs="*", default=ALL_PIPELINES, choices=ALL_PIPELINES, help="Pipelines to run")
    parser.add_argument("--variants", nargs="*", default=ALL_VARIANTS, choices=ALL_VARIANTS, help="Variants to run")
    parser.add_argument("--models", nargs="*", default=None, help="Model keys to run; default = all models in config")
    parser.add_argument("--limit", type=int, default=None, help="Limit number of samples per run")
    parser.add_argument("--workers", type=int, default=8, help="Number of workers passed to each run script")
    parser.add_argument("--timeout", type=int, default=300, help="Timeout passed to each run script")
    parser.add_argument("--max-tokens", type=int, default=2048, help="Max tokens passed to each run script")
    parser.add_argument("--dry-run", action="store_true", help="Print commands only; do not execute")
    parser.add_argument("--stop-on-error", action="store_true", help="Stop on the first failed run")

    args = parser.parse_args(argv)

    config_path = Path(args.config)
    dataset_path = Path(args.dataset)
    out_dir = Path(args.out_dir)

    if args.models is None:
        model_keys = get_models(config_path)
    else:
        model_keys = list(args.models)

    if not model_keys:
        print("No models found. Check config/models.", file=sys.stderr)
        return 2

    runs: List[List[str]] = []
    for pipeline in args.pipelines:
        for variant in args.variants:
            script_name = PIPELINE_VARIANT_TO_SCRIPT[pipeline][variant]
            for model in model_keys:
                runs.append(
                    build_command(
                        script_name,
                        config=config_path,
                        dataset=dataset_path,
                        out_dir=out_dir,
                        model=model,
                        workers=args.workers,
                        timeout=args.timeout,
                        max_tokens=args.max_tokens,
                        limit=args.limit,
                    )
                )

    print(f"Planned runs: {len(runs)}")
    for i, cmd in enumerate(runs, start=1):
        pretty = " ".join(cmd)
        print(f"[{i:03d}] {pretty}")

    if args.dry_run:
        return 0

    failures: List[str] = []
    for i, cmd in enumerate(runs, start=1):
        print(f"\n=== Running {i}/{len(runs)} ===")
        print(" ".join(cmd))
        proc = subprocess.run(cmd, cwd=str(ROOT))
        if proc.returncode != 0:
            failures.append(" ".join(cmd))
            print(f"Run failed with exit code {proc.returncode}", file=sys.stderr)
            if args.stop_on_error:
                break

    if failures:
        print(f"\nCompleted with {len(failures)} failed run(s).", file=sys.stderr)
        return 1

    print("\nAll runs completed successfully.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
