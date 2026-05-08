#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""Unified verifier for NL2VerusBench generation outputs.

What it does:
- Verifies pass@5 for all experiments found under generation/
- Derives a pass@1 result from a chosen generation index (default: 4th generation)
- Writes results in the same style as the existing verify_pass5_* and
    verify_pass1 folder
- Writes a CSV summary with pass@1 and pass@5 success rates per experiment

Assumptions about generation/ layout:
- p1 pipelines:   00.rs ~ 04.rs
- p2 pipelines:   code00.rs / complete00.rs ~ code04.rs / complete04.rs
- p3 pipelines:   spec00.rs / complete00.rs ~ spec04.rs / complete04.rs

For verification, only the final artifact is checked:
- p1: 00.rs ~ 04.rs
- p2/p3: complete00.rs ~ complete04.rs
"""

from __future__ import annotations

import argparse
import csv
import json
import os
import re
import subprocess
import sys
import tempfile
from concurrent.futures import ThreadPoolExecutor, as_completed
from dataclasses import dataclass
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Dict, Iterable, List, Optional, Sequence, Tuple

try:
    import yaml  # type: ignore
except Exception:  # pragma: no cover
    yaml = None

try:
    from tqdm import tqdm
except Exception:  # pragma: no cover
    tqdm = None

ROOT = Path(__file__).resolve().parents[2]
GEN_ROOT = ROOT / "generation"
VERIFY_ROOT = Path(__file__).resolve().parent
SUMMARY_TABLES_DIR = VERIFY_ROOT / "summary_tables"

PIPELINE_VARIANT_TO_DIR = {
    "p1": {
        "zeroshot": "p1_zeroshot",
        "fewshot": "p1_fewshot",
        "fewshot+COT": "p1_fewshot+COT",
        "fewshot+LTM": "p1_fewshot+LTM",
    },
    "p2": {
        "zeroshot": "p2_zeroshot",
        "fewshot": "p2_fewshot",
        "fewshot+COT": "p2_fewshot+COT",
        "fewshot+LTM": "p2_fewshot+LTM",
    },
    "p3": {
        "zeroshot": "p3_zeroshot",
        "fewshot": "p3_fewshot",
        "fewshot+COT": "p3_fewshot+COT",
        "fewshot+LTM": "p3_fewshot+LTM",
    },
}

PASS5_OUTPUT_DIRS = {
    "zeroshot": VERIFY_ROOT / "verify_pass5_zeroshot",
    "fewshot": VERIFY_ROOT / "verify_pass5_fewshot",
    "fewshot+COT": VERIFY_ROOT / "verify_pass5_fewshot_COT",
    "fewshot+LTM": VERIFY_ROOT / "verify_pass5_fewshot_LTM",
}

DEFAULT_PIPELINES = ["p1", "p2", "p3"]
DEFAULT_VARIANTS = ["zeroshot", "fewshot", "fewshot+COT", "fewshot+LTM"]


@dataclass(frozen=True)
class ExperimentKey:
    pipeline: str
    variant: str
    model: str

    @property
    def pipeline_variant(self) -> str:
        return PIPELINE_VARIANT_TO_DIR[self.pipeline][self.variant]

    @property
    def safe_name(self) -> str:
        return f"{self.pipeline_variant}_{self.model}".replace("+", "plus")


@dataclass
class SampleResult:
    sample_id: str
    pipeline: str
    variant: str
    model: str
    status: str
    pass5_result: bool
    pass1_result: bool
    pass5_details: List[Dict[str, Any]]
    pass1_detail: Dict[str, Any]
    total_files: int
    error: Optional[str] = None
    file_to_repair: Optional[str] = None


def read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def naive_yaml_load(text: str) -> Dict[str, Any]:
    result: Dict[str, Any] = {}
    stack: List[Tuple[int, Dict[str, Any]]] = [(-1, result)]
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


def load_dataset_ids(dataset_path: Path) -> List[str]:
    ids: List[str] = []
    with dataset_path.open("r", encoding="utf-8") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            obj = json.loads(line)
            sid = obj.get("sample_id")
            if sid:
                ids.append(str(sid))
    return ids


def ordinal_name(n: int) -> str:
    mapping = {
        1: "first",
        2: "second",
        3: "third",
        4: "fourth",
        5: "fifth",
    }
    return mapping.get(n, f"gen{n}")


def ensure_chat_endpoint(base_url: str) -> str:
    if base_url.rstrip("/").endswith("/chat/completions"):
        return base_url
    return base_url.rstrip("/") + "/chat/completions"


def extract_rust_block(text: str) -> str:
    if "```" not in text:
        return text
    start = text.find("```")
    if start == -1:
        return text
    newline_after = text.find("\n", start + 3)
    if newline_after == -1:
        return text
    end = text.find("```", newline_after + 1)
    if end == -1:
        return text
    return text[newline_after + 1 : end]


def read_prompt_module(path: Path) -> Dict[str, Any]:
    ns: Dict[str, Any] = {}
    exec(path.read_text(encoding="utf-8"), ns, ns)
    return ns


def build_messages(system_prompt: str, user_prompt_tmpl: str, **kwargs: Any) -> List[Dict[str, str]]:
    return [
        {"role": "system", "content": system_prompt.strip()},
        {"role": "user", "content": user_prompt_tmpl.format(**kwargs).strip()},
    ]


def run_verus(verus_cmd: str, rs_file: Path, timeout: int) -> Tuple[bool, str]:
    try:
        with tempfile.TemporaryDirectory() as tmpdir:
            proc = subprocess.run(
                [verus_cmd, str(rs_file)],
                capture_output=True,
                text=True,
                timeout=timeout,
                cwd=tmpdir,
            )
        return proc.returncode == 0, proc.stdout + proc.stderr
    except subprocess.TimeoutExpired:
        return False, f"Verification timeout ({timeout}s)"
    except FileNotFoundError:
        return False, f"Verus command not found: {verus_cmd}"
    except Exception as e:
        return False, f"Error running verus: {e}"


def get_file_for_index(pipeline: str, model_dir: Path, idx: int) -> Path:
    if pipeline == "p1":
        return model_dir / f"{idx:02d}.rs"
    return model_dir / f"complete{idx:02d}.rs"


def get_pass5_files(pipeline: str, model_dir: Path) -> List[Path]:
    return [get_file_for_index(pipeline, model_dir, idx) for idx in range(5)]


def select_file_to_repair(pass5_files: Sequence[Path]) -> str:
    existing = [f for f in pass5_files if f.exists()]
    if not existing:
        return "00.rs"
    return existing[0].name


def verify_one_sample(
    sample_id: str,
    pipeline: str,
    variant: str,
    model: str,
    generation_dir: Path,
    pass1_generation: int,
    verus_cmd: str,
    timeout: int,
) -> SampleResult:
    exp_dir = generation_dir / sample_id / PIPELINE_VARIANT_TO_DIR[pipeline][variant]
    model_dir = exp_dir / model

    if not model_dir.exists():
        return SampleResult(
            sample_id=sample_id,
            pipeline=pipeline,
            variant=variant,
            model=model,
            status="not_found",
            pass5_result=False,
            pass1_result=False,
            pass5_details=[],
            pass1_detail={"file": None, "success": False, "output": f"Model directory not found: {model_dir}"},
            total_files=0,
            error=f"Model directory not found: {model_dir}",
        )

    pass5_details: List[Dict[str, Any]] = []
    passed_files: List[str] = []
    pass1_detail: Dict[str, Any] = {"file": None, "success": False, "output": ""}

    pass1_index = pass1_generation - 1
    pass5_files = get_pass5_files(pipeline, model_dir)
    for idx, rs_file in enumerate(pass5_files):
        if rs_file.exists():
            success, output = run_verus(verus_cmd, rs_file, timeout)
        else:
            success = False
            output = f"Missing file: {rs_file}"
        detail = {"file": rs_file.name, "success": success, "output": output}
        pass5_details.append(detail)
        if success:
            passed_files.append(rs_file.name)
        if idx == pass1_index:
            pass1_detail = detail.copy()

    # If the chosen generation is missing, record that explicitly.
    if pass1_index < 0 or pass1_index >= len(pass5_files):
        pass1_detail = {
            "file": None,
            "success": False,
            "output": f"Chosen pass@1 generation {pass1_generation} not available (only {len(pass5_files)} files found)",
        }

    pass5_result = len(passed_files) > 0
    pass1_result = bool(pass1_detail.get("success", False))

    result = SampleResult(
        sample_id=sample_id,
        pipeline=pipeline,
        variant=variant,
        model=model,
        status="completed",
        pass5_result=pass5_result,
        pass1_result=pass1_result,
        pass5_details=pass5_details,
        pass1_detail=pass1_detail,
        total_files=len(pass5_files),
        error=None,
        file_to_repair=None if pass5_result else select_file_to_repair(pass5_files),
    )
    return result


def save_json(path: Path, obj: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(obj, ensure_ascii=False, indent=2), encoding="utf-8")


def save_jsonl(path: Path, rows: Iterable[Dict[str, Any]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        for row in rows:
            f.write(json.dumps(row, ensure_ascii=False) + "\n")


def aggregate_summary(results: Sequence[SampleResult], *, metric: str) -> Dict[str, Any]:
    total = 0
    success_count = 0
    for r in results:
        if r.status != "completed":
            continue
        total += 1
        if metric == "pass5" and r.pass5_result:
            success_count += 1
        if metric == "pass1" and r.pass1_result:
            success_count += 1
    return {
        "total_samples": total,
        "passed_samples": success_count,
        "overall_rate": (success_count / total) if total else 0.0,
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }


def main(argv: Optional[List[str]] = None) -> int:
    parser = argparse.ArgumentParser(description="Unified verifier for generation outputs")
    parser.add_argument("--config", default=str(ROOT / "config" / "config.yaml"), help="Path to config.yaml")
    parser.add_argument("--dataset", default=str(ROOT / "dataset" / "NL2VBench.jsonl"), help="Path to dataset jsonl")
    parser.add_argument("--generation-root", default=str(GEN_ROOT), help="Generation output root")
    parser.add_argument("--pipelines", nargs="*", default=DEFAULT_PIPELINES, choices=DEFAULT_PIPELINES, help="Pipelines to verify")
    parser.add_argument("--variants", nargs="*", default=DEFAULT_VARIANTS, choices=DEFAULT_VARIANTS, help="Variants to verify")
    parser.add_argument("--models", nargs="*", default=None, help="Optional model filter")
    parser.add_argument("--pass1-generation", type=int, default=4, help="Which pass@5 generation to use for pass@1 extraction (1=00, 2=01, 3=02, 4=03, 5=04)")
    parser.add_argument("--max-workers", type=int, default=4, help="Parallel workers")
    parser.add_argument("--timeout", type=int, default=120, help="Verus timeout per file (seconds)")
    parser.add_argument("--dry-run", action="store_true", help="Print planned tasks and exit")
    parser.add_argument("--clean-output", action="store_true", help="Delete current output files before writing new ones")
    args = parser.parse_args(argv)

    config_path = Path(args.config)
    dataset_path = Path(args.dataset)
    generation_root = Path(args.generation_root)

    cfg = load_config(config_path)
    verus_cfg = cfg.get("verus", {}) if isinstance(cfg, dict) else {}
    verus_cmd = str(verus_cfg.get("verus_path") or "verus")

    # Verify that Verus exists only when we actually run.
    if not args.dry_run:
        try:
            subprocess.run([verus_cmd, "--version"], capture_output=True, check=True)
        except Exception:
            print(f"Verus command not found or failed: {verus_cmd}", file=sys.stderr)
            return 1

    sample_ids = load_dataset_ids(dataset_path)
    if not sample_ids:
        print("No samples found in dataset.", file=sys.stderr)
        return 1

    model_filter = set(args.models) if args.models else None
    pass1_name = ordinal_name(args.pass1_generation)
    pass1_out_dir = VERIFY_ROOT / "verify_pass1"
    pass1_out_dir.mkdir(parents=True, exist_ok=True)

    # Discover models present in generation tree for each experiment.
    discovered: Dict[ExperimentKey, set[str]] = {}
    for pipeline in args.pipelines:
        for variant in args.variants:
            exp_dir_name = PIPELINE_VARIANT_TO_DIR[pipeline][variant]
            model_names: set[str] = set()
            for sample_id in sample_ids:
                exp_dir = generation_root / sample_id / exp_dir_name
                if not exp_dir.exists():
                    continue
                for child in exp_dir.iterdir():
                    if child.is_dir():
                        model_names.add(child.name)
            if model_filter is not None:
                model_names &= model_filter
            for model in sorted(model_names):
                discovered.setdefault(ExperimentKey(pipeline, variant, model), set()).add(model)

    tasks: List[Tuple[str, str, str, str]] = []
    for sample_id in sample_ids:
        for key in sorted(discovered.keys(), key=lambda k: (k.pipeline, k.variant, k.model)):
            tasks.append((sample_id, key.pipeline, key.variant, key.model))

    print(f"Planned tasks: {len(tasks)}")
    if args.dry_run:
        for sample_id, pipeline, variant, model in tasks[:20]:
            key = ExperimentKey(pipeline, variant, model)
            exp_dir = generation_root / sample_id / key.pipeline_variant / model
            pass5_preview = [p for p in get_pass5_files(pipeline, exp_dir) if p.exists()]
            print(f"{sample_id} | {key.pipeline_variant} | {model}")
            for p in pass5_preview:
                print(f"  pass5: {p}")
            idx = args.pass1_generation - 1
            if pipeline == "p1":
                print(f"  pass1: {exp_dir / f'{idx:02d}.rs'}")
            else:
                print(f"  pass1: {exp_dir / f'complete{idx:02d}.rs'}")
        return 0

    if args.clean_output:
        for d in PASS5_OUTPUT_DIRS.values():
            if d.exists():
                for p in d.glob("*"):
                    if p.is_file():
                        p.unlink()
        if pass1_out_dir.exists():
            for p in pass1_out_dir.glob("*"):
                if p.is_file():
                    p.unlink()
        if SUMMARY_TABLES_DIR.exists():
            for p in SUMMARY_TABLES_DIR.glob("verification_rates.csv"):
                p.unlink()

    results_by_exp: Dict[Tuple[str, str, str], List[SampleResult]] = {}

    with ThreadPoolExecutor(max_workers=max(1, args.max_workers)) as executor:
        futures = {
            executor.submit(
                verify_one_sample,
                sample_id,
                pipeline,
                variant,
                model,
                generation_root,
                args.pass1_generation,
                verus_cmd,
                args.timeout,
            ): (sample_id, pipeline, variant, model)
            for sample_id, pipeline, variant, model in tasks
        }

        iterator = as_completed(futures)
        if tqdm is not None:
            iterator = tqdm(iterator, total=len(futures), desc="Verifying generation", unit="sample")

        for fut in iterator:
            sample_id, pipeline, variant, model = futures[fut]
            try:
                res = fut.result()
            except Exception as e:
                res = SampleResult(
                    sample_id=sample_id,
                    pipeline=pipeline,
                    variant=variant,
                    model=model,
                    status="error",
                    pass5_result=False,
                    pass1_result=False,
                    pass5_details=[],
                    pass1_detail={"file": None, "success": False, "output": str(e)},
                    total_files=0,
                    error=str(e),
                )
            results_by_exp.setdefault((pipeline, variant, model), []).append(res)

    # Write outputs per experiment.
    csv_rows: List[Dict[str, Any]] = []
    pass1_rows: List[Dict[str, Any]] = []
    pass1_batch = {
        "generated_from": str(generation_root),
        "output_dir": str(pass1_out_dir),
        "pass1_generation": args.pass1_generation,
        "pass1_generation_name": pass1_name,
        "total_experiments": 0,
        "total_samples": 0,
        "total_succeeded": 0,
        "total_failed": 0,
        "overall_success_rate": 0.0,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "experiments": [],
    }

    for (pipeline, variant, model), results in sorted(results_by_exp.items()):
        key = ExperimentKey(pipeline, variant, model)
        pass5_dir = PASS5_OUTPUT_DIRS[variant]
        pass5_dir.mkdir(parents=True, exist_ok=True)
        pass5_path = pass5_dir / f"{key.safe_name}.json"
        pass5_summary_path = pass5_dir / f"{key.safe_name}_summary.json"

        pass5_payload = [
            {
                "sample_id": r.sample_id,
                "pipeline": f"{pipeline}_{variant}",
                "model": model,
                "status": r.status,
                "pass5_result": r.pass5_result,
                "passed_files": [d["file"] for d in r.pass5_details if d.get("success")],
                "total_files": r.total_files,
                "verification_details": r.pass5_details,
                "timestamp": datetime.now(timezone.utc).isoformat(),
                **({"file_to_repair": r.file_to_repair} if r.file_to_repair else {}),
                **({"error": r.error} if r.error else {}),
            }
            for r in results
        ]
        save_json(pass5_path, pass5_payload)

        pass5_summary = aggregate_summary(results, metric="pass5")
        pass5_summary.update({"pipeline": pipeline, "variant": variant, "model": model})
        save_json(pass5_summary_path, pass5_summary)

        # pass@1 JSONL file, using the chosen generation.
        pass1_path = pass1_out_dir / f"{key.safe_name}.jsonl"
        pass1_summary = aggregate_summary(results, metric="pass1")
        pass1_summary.update({
            "type": "summary",
            "category": f"{pipeline}_{variant}",
            "model": model,
            "pass1_generation": args.pass1_generation,
            "pass1_generation_name": pass1_name,
        })
        pass1_rows = [pass1_summary]
        for r in results:
            row: Dict[str, Any] = {
                "type": "result",
                "sample_id": r.sample_id,
                "success": r.pass1_result,
            }
            if not r.pass1_result:
                row["error"] = r.pass1_detail.get("output", "")
            pass1_rows.append(row)
        save_jsonl(pass1_path, pass1_rows)

        pass1_batch["experiments"].append({
            "experiment": key.safe_name,
            "pipeline": pipeline,
            "variant": variant,
            "model": model,
            "total": pass1_summary["total_samples"],
            "succeeded": pass1_summary["passed_samples"],
            "failed": pass1_summary["total_samples"] - pass1_summary["passed_samples"],
            "success_rate": pass1_summary["overall_rate"],
        })
        pass1_batch["total_experiments"] += 1
        pass1_batch["total_samples"] += pass1_summary["total_samples"]
        pass1_batch["total_succeeded"] += pass1_summary["passed_samples"]
        pass1_batch["total_failed"] += pass1_summary["total_samples"] - pass1_summary["passed_samples"]

        csv_rows.append({
            "experiment": key.safe_name,
            "pipeline": pipeline,
            "variant": variant,
            "model": model,
            "total_samples": pass5_summary["total_samples"],
            "pass1_rate": f"{pass1_summary['overall_rate'] * 100:.2f}%",
            "pass5_rate": f"{pass5_summary['overall_rate'] * 100:.2f}%",
        })

    if pass1_batch["total_samples"] > 0:
        pass1_batch["overall_success_rate"] = pass1_batch["total_succeeded"] / pass1_batch["total_samples"]

    save_json(pass1_out_dir / "_batch_summary.json", pass1_batch)

    SUMMARY_TABLES_DIR.mkdir(parents=True, exist_ok=True)
    csv_path = SUMMARY_TABLES_DIR / "pass1_pass5_verification_rates.csv"
    with csv_path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(
            f,
            fieldnames=[
                "experiment",
                "pipeline",
                "variant",
                "model",
                "total_samples",
                "pass1_rate",
                "pass5_rate",
            ],
        )
        writer.writeheader()
        writer.writerows(csv_rows)

    print("Done.")
    print(f"Pass@5 outputs: {len(csv_rows)} experiments")
    print(f"Pass@1 outputs: {pass1_out_dir}")
    print(f"CSV summary: {csv_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
