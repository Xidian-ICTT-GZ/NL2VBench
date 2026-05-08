#!/usr/bin/env python3
"""
Analyze pass@5 verification failures under evaluation/verify and regroup them into error families.

Outputs:
- detailed pass@5 error classification JSON
- detailed full-category CSV
- family-level JSON
- family-level CSV
"""

from __future__ import annotations

import argparse
import csv
import json
import re
from collections import Counter, defaultdict
from pathlib import Path
from typing import Any, DefaultDict, Dict, Iterable, List, Tuple

from batch_classify_errors import classify_error_detailed, extract_error_context


PASS5_DIRS = [
    "verify_pass5_fewshot",
    "verify_pass5_fewshot_COT",
    "verify_pass5_fewshot_LTM",
    "verify_pass5_zeroshot",
]

DEFAULT_BASE_DIR = Path(__file__).resolve().parents[1] / "verify"
DEFAULT_OUTPUT_DIR = Path(__file__).resolve().parent / "outputs" / "pass5_error_analysis"


def map_to_family_6(main: str, sub: str) -> str:
    """Map detailed error categories to six high-level families."""
    if main == "syntax_error":
        return "F1_syntax"

    if main in {"import_error", "undefined_symbol", "main_function"}:
        return "F2_environment"

    if main in {"type_error", "ownership_error", "rust_compile_error"}:
        return "F3_type"

    if main == "verus_syntax":
        if sub in {"ghost_error", "spec_function_error", "proof_block_error", "exec_mode_error"}:
            return "F4_spec_construction"
        return "F1_syntax"

    if main == "verus_verification":
        return "F5_verification_logic"

    return "F6_other"


FAMILY_NAMES = {
    "F1_syntax": "Syntax errors",
    "F2_environment": "Environment issues",
    "F3_type": "Type / ownership / compile errors",
    "F4_spec_construction": "Verus spec construction",
    "F5_verification_logic": "Verus verification logic",
    "F6_other": "Other",
}

FAMILY_ORDER = [
    "F1_syntax",
    "F2_environment",
    "F3_type",
    "F4_spec_construction",
    "F5_verification_logic",
    "F6_other",
]

FAMILY_COLUMN_NAMES = {
    "F1_syntax": "F1_Syntax_Errors",
    "F2_environment": "F2_Environment_Issues",
    "F3_type": "F3_Type_Related",
    "F4_spec_construction": "F4_Verus_Spec",
    "F5_verification_logic": "F5_Verification_Logic",
    "F6_other": "F6_Other",
}


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as f:
        return json.load(f)


def save_json(path: Path, obj: Any) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8") as f:
        json.dump(obj, f, indent=2, ensure_ascii=False)


def save_csv(path: Path, rows: List[Dict[str, Any]], fieldnames: List[str]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fieldnames)
        writer.writeheader()
        writer.writerows(rows)


def format_count_and_percentage(count: int, denominator: int, decimals: int = 1) -> str:
    pct = (count / denominator * 100) if denominator else 0.0
    return f"{count} ({pct:.{decimals}f}%)"


def iter_pass5_result_files(base_dir: Path) -> Iterable[Tuple[str, Path]]:
    """Yield only pass@5 result JSON files and skip pass@1 files and summaries."""
    for dir_name in PASS5_DIRS:
        dir_path = base_dir / dir_name
        if not dir_path.exists():
            continue

        for path in sorted(dir_path.glob("*.json")):
            if path.stem.endswith("_summary"):
                continue
            if not re.match(r"^p[123]_", path.stem):
                continue
            yield dir_name, path


def split_experiment_name(stem: str) -> Tuple[str, str]:
    """Split `p1_fewshot_model-name` into `(category, model)`.

    The model name is taken from the last underscore chunk, which matches the
    current file naming convention in `evaluation/verify`.
    """
    if "_" not in stem:
        return stem, "unknown"
    category, model = stem.rsplit("_", 1)
    return category, model


def analyze_pass5_results(base_dir: Path) -> Dict[str, Any]:
    """Scan pass@5 results and collect detailed error statistics."""
    global_stats: Dict[str, Any] = {
        "result_files": 0,
        "verification_attempts": 0,
        "passed_attempts": 0,
        "failed_attempts": 0,
        "by_experiment": defaultdict(
            lambda: {
                "attempts": 0,
                "passed": 0,
                "failed": 0,
                "families": Counter(),
            }
        ),
    }

    error_main_categories: Counter[str] = Counter()
    error_sub_categories: Counter[str] = Counter()
    error_full_categories: Counter[Tuple[str, str]] = Counter()
    error_contexts: DefaultDict[str, List[Dict[str, Any]]] = defaultdict(list)

    for dir_name, json_file in iter_pass5_result_files(base_dir):
        global_stats["result_files"] += 1
        print(f"Analyzing: {dir_name}/{json_file.name}")

        try:
            samples = load_json(json_file)
        except Exception as e:
            print(f"  Skip unreadable file: {e}")
            continue

        if not isinstance(samples, list):
            print("  Skip non-list JSON file")
            continue

        category, model = split_experiment_name(json_file.stem)
        exp_key = f"{category}::{model}"
        exp_stats = global_stats["by_experiment"][exp_key]

        for sample in samples:
            if not isinstance(sample, dict):
                continue

            sample_id = str(sample.get("sample_id", "unknown"))
            verification_details = sample.get("verification_details", [])
            if not isinstance(verification_details, list):
                continue

            for detail in verification_details:
                if not isinstance(detail, dict):
                    continue

                global_stats["verification_attempts"] += 1
                exp_stats["attempts"] += 1

                success = bool(detail.get("success", False))
                if success:
                    global_stats["passed_attempts"] += 1
                    exp_stats["passed"] += 1
                    continue

                global_stats["failed_attempts"] += 1
                exp_stats["failed"] += 1

                error_msg = str(detail.get("output", ""))
                main_cat, sub_cat = classify_error_detailed(error_msg)
                full_cat = f"{main_cat}::{sub_cat}"
                family = map_to_family_6(main_cat, sub_cat)

                error_main_categories[main_cat] += 1
                error_sub_categories[sub_cat] += 1
                error_full_categories[(main_cat, sub_cat)] += 1
                exp_stats["families"][family] += 1

                if len(error_contexts[full_cat]) < 5:
                    error_contexts[full_cat].append(
                        {
                            "source_dir": dir_name,
                            "file": json_file.name,
                            "sample_id": sample_id,
                            "context": extract_error_context(error_msg),
                        }
                    )

    return {
        "global_stats": global_stats,
        "error_main_categories": error_main_categories,
        "error_sub_categories": error_sub_categories,
        "error_full_categories": error_full_categories,
        "error_contexts": error_contexts,
    }


def build_full_category_rows(analysis: Dict[str, Any]) -> List[Dict[str, Any]]:
    total_failed = analysis["global_stats"]["failed_attempts"]
    rows: List[Dict[str, Any]] = []

    for rank, ((main_cat, sub_cat), count) in enumerate(
        analysis["error_full_categories"].most_common(),
        1,
    ):
        rows.append(
            {
                "rank": rank,
                "main_category": main_cat,
                "sub_category": sub_cat,
                "count": count,
                "percentage": f"{(count / total_failed * 100) if total_failed else 0.0:.2f}%",
            }
        )
    return rows


def analyze_families(analysis: Dict[str, Any]) -> Dict[str, Any]:
    total_failed = analysis["global_stats"]["failed_attempts"]

    family_stats: Counter[str] = Counter()
    family_examples: DefaultDict[str, List[Dict[str, Any]]] = defaultdict(list)

    for (main, sub), count in analysis["error_full_categories"].items():
        family = map_to_family_6(main, sub)
        family_stats[family] += count
        family_examples[family].append(
            {
                "main": main,
                "sub": sub,
                "count": count,
                "percentage": (count / total_failed * 100) if total_failed else 0.0,
            }
        )

    for family in list(family_examples.keys()):
        family_examples[family] = sorted(family_examples[family], key=lambda x: x["count"], reverse=True)[:5]

    family_distribution = []
    for rank, (family, count) in enumerate(family_stats.most_common(), 1):
        family_distribution.append(
            {
                "rank": rank,
                "family": family,
                "name": FAMILY_NAMES.get(family, family),
                "count": count,
                "percentage": f"{(count / total_failed * 100) if total_failed else 0.0:.2f}%",
            }
        )

    return {
        "total_failed": total_failed,
        "family_distribution": family_distribution,
        "family_examples": family_examples,
        "mapping_function": "map_to_family_6",
    }


def build_family_rows(family_report: Dict[str, Any]) -> List[Dict[str, Any]]:
    rows: List[Dict[str, Any]] = []
    for item in family_report["family_distribution"]:
        rows.append(
            {
                "rank": item["rank"],
                "family": item["family"],
                "name": item["name"],
                "count": item["count"],
                "percentage": item["percentage"],
            }
        )
    return rows


def build_experiment_family_rows(analysis: Dict[str, Any]) -> List[Dict[str, Any]]:
    rows: List[Dict[str, Any]] = []
    experiments = analysis["global_stats"]["by_experiment"]

    for experiment in sorted(experiments.keys()):
        stats = experiments[experiment]
        total = stats["attempts"]
        passed = stats["passed"]
        failed = stats["failed"]
        experiment_name = experiment.replace("::", "_")

        row: Dict[str, Any] = {
            "experiment": experiment_name,
            "total": total,
            "passed": passed,
            "failed": failed,
            "success_rate": f"{(passed / total * 100) if total else 0.0:.2f}%",
        }

        families: Counter[str] = stats.get("families", Counter())
        for family_key in FAMILY_ORDER:
            col_name = FAMILY_COLUMN_NAMES[family_key]
            row[col_name] = format_count_and_percentage(families.get(family_key, 0), failed, decimals=1)

        rows.append(row)

    return rows


def print_summary(analysis: Dict[str, Any], family_report: Dict[str, Any]) -> None:
    stats = analysis["global_stats"]
    total_attempts = stats["verification_attempts"]
    total_failed = stats["failed_attempts"]
    total_passed = stats["passed_attempts"]

    print("\n" + "=" * 80)
    print("Pass@5 verification failure summary")
    print("=" * 80)
    print(f"Result files:         {stats['result_files']}")
    print(f"Verification attempts: {total_attempts}")
    print(f"Passed attempts:      {total_passed} ({(total_passed / total_attempts * 100) if total_attempts else 0.0:.2f}%)")
    print(f"Failed attempts:      {total_failed} ({(total_failed / total_attempts * 100) if total_attempts else 0.0:.2f}%)")

    print("\nTop error families:")
    for item in family_report["family_distribution"][:10]:
        print(f"  {item['family']:22s} {item['count']:7d} {item['percentage']}")


def main() -> int:
    parser = argparse.ArgumentParser(description="Analyze pass@5 verification failures and regroup them into families")
    parser.add_argument("--base-directory", type=Path, default=DEFAULT_BASE_DIR, help="Path to evaluation/verify")
    parser.add_argument("--output-directory", type=Path, default=DEFAULT_OUTPUT_DIR, help="Where to write analysis files")
    args = parser.parse_args()

    if not args.base_directory.exists():
        print(f"Error: base directory does not exist: {args.base_directory}")
        return 1

    args.output_directory.mkdir(parents=True, exist_ok=True)

    analysis = analyze_pass5_results(args.base_directory)
    family_report = analyze_families(analysis)

    full_rows = build_full_category_rows(analysis)
    family_rows = build_family_rows(family_report)
    experiment_rows = build_experiment_family_rows(analysis)

    total_failed = analysis["global_stats"]["failed_attempts"]
    total_attempts = analysis["global_stats"]["verification_attempts"]
    pass5_rate = (analysis["global_stats"]["passed_attempts"] / total_attempts * 100) if total_attempts else 0.0

    detailed_report = {
        "source_base_directory": str(args.base_directory),
        "global_stats": {
            "result_files": analysis["global_stats"]["result_files"],
            "verification_attempts": total_attempts,
            "passed_attempts": analysis["global_stats"]["passed_attempts"],
            "failed_attempts": total_failed,
            "pass_rate": pass5_rate,
            "by_experiment": dict(analysis["global_stats"]["by_experiment"]),
        },
        "error_main_categories": analysis["error_main_categories"],
        "error_sub_categories": analysis["error_sub_categories"],
        "error_full_categories": {
            f"{main}::{sub}": count for (main, sub), count in analysis["error_full_categories"].items()
        },
        "error_contexts": dict(analysis["error_contexts"]),
    }

    family_output = {
        "source_base_directory": str(args.base_directory),
        "total_failed": total_failed,
        "family_distribution": family_report["family_distribution"],
        "family_examples": dict(family_report["family_examples"]),
        "mapping_function": family_report["mapping_function"],
    }

    save_json(args.output_directory / "pass5_complete_error_analysis.json", detailed_report)
    save_json(args.output_directory / "pass5_error_families_analysis.json", family_output)
    save_csv(
        args.output_directory / "pass5_full_category_counts.csv",
        full_rows,
        ["rank", "main_category", "sub_category", "count", "percentage"],
    )
    save_csv(
        args.output_directory / "pass5_error_family_counts.csv",
        family_rows,
        ["rank", "family", "name", "count", "percentage"],
    )
    save_csv(
        args.output_directory / "pass5_experiment_family_distribution.csv",
        experiment_rows,
        [
            "experiment",
            "total",
            "passed",
            "failed",
            "success_rate",
            "F1_Syntax_Errors",
            "F2_Environment_Issues",
            "F3_Type_Related",
            "F4_Verus_Spec",
            "F5_Verification_Logic",
            "F6_Other",
        ],
    )

    print_summary(analysis, family_report)
    print("\nSaved files:")
    print(f"  {args.output_directory / 'pass5_complete_error_analysis.json'}")
    print(f"  {args.output_directory / 'pass5_error_families_analysis.json'}")
    print(f"  {args.output_directory / 'pass5_full_category_counts.csv'}")
    print(f"  {args.output_directory / 'pass5_error_family_counts.csv'}")
    print(f"  {args.output_directory / 'pass5_experiment_family_distribution.csv'}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())#!/usr/bin/env python3
"""
按错误家族重新分类分析结果
"""
import json
from pathlib import Path
from typing import Dict, Tuple

def map_to_family_6(main, sub):
    """将错误映射到6个家族"""
    if main in ["syntax_error"]:
        return "F1_syntax"

    if main in ["import_error", "undefined_symbol", "main_function"]:
        return "F2_environment"

    if main in ["type_error", "ownership_error", "rust_compile_error"]:
        return "F3_type"

    if main == "verus_syntax":
        if sub in [
            "ghost_error",
            "spec_function_error",
            "proof_block_error",
            "exec_mode_error"
        ]:
            return "F4_spec_construction"
        else:
            return "F1_syntax"

    if main == "verus_verification":
        return "F5_verification_logic"

    return "F6_other"

def analyze_families(input_file: str):
    """分析错误家族分布"""
    
    # 读取原始分析结果
    with open(input_file, 'r') as f:
        data = json.load(f)
    
    # 统计家族分布
    family_stats = {}
    family_examples = {}
    
    # 计算总失败数
    total_errors = data['global_stats']['total_failed']
    
    # 从完整错误分类中提取
    for error_full, count in data['error_full_categories'].items():
        main, sub = error_full.split('::', 1)
        family = map_to_family_6(main, sub)
        
        if family not in family_stats:
            family_stats[family] = 0
            family_examples[family] = []
        
        family_stats[family] += count
        
        # 保存示例
        percentage = (count / total_errors) * 100
        family_examples[family].append({
            'main': main,
            'sub': sub,
            'count': count,
            'percentage': percentage
        })
    
    # 对每个家族的示例按数量排序并限制数量
    for family in family_examples:
        family_examples[family] = sorted(
            family_examples[family], 
            key=lambda x: x['count'], 
            reverse=True
        )[:5]
    
    # 排序
    sorted_families = sorted(family_stats.items(), key=lambda x: x[1], reverse=True)
    
    # 打印结果
    print("=" * 80)
    print("错误家族分布")
    print("=" * 80)
    print(f"总失败样本数: {total_errors:,}")
    print()
    
    family_names = {
        "F1_syntax": "语法错误",
        "F2_environment": "环境问题（导入/未定义/缺少main）",
        "F3_type": "类型相关（类型/所有权/Rust编译错误）",
        "F4_spec_construction": "Verus规范构建（ghost/spec/proof/exec）",
        "F5_verification_logic": "Verus验证逻辑",
        "F6_other": "其他"
    }
    
    for family, count in sorted_families:
        percentage = (count / total_errors) * 100
        name = family_names.get(family, family)
        print(f"{family:25s} | {name:40s} | {count:7,} ({percentage:5.2f}%)")
    
    print()
    print("=" * 80)
    print("各家族主要错误类型")
    print("=" * 80)
    
    for family, count in sorted_families:
        name = family_names.get(family, family)
        percentage = (count / total_errors) * 100
        print(f"\n{family} - {name}")
        print(f"总计: {count:,} ({percentage:.2f}%)")
        print("-" * 80)
        
        # 显示该家族的主要错误类型
        for example in family_examples[family]:
            print(f"  {example['main']:20s} :: {example['sub']:30s} "
                  f"{example['count']:7,} ({example['percentage']:5.2f}%)")
    
    # 保存结果
    output = {
        'family_distribution': {
            family: {
                'name': family_names.get(family, family),
                'count': count,
                'percentage': (count / total_errors) * 100
            }
            for family, count in sorted_families
        },
        'family_examples': family_examples,
        'total_errors': total_errors,
        'mapping_function': 'map_to_family_6'
    }
    
    output_file = Path(input_file).parent / 'error_families_analysis.json'
    with open(output_file, 'w') as f:
        json.dump(output, f, indent=2, ensure_ascii=False)
    
    print(f"\n✓ 详细结果已保存到: {output_file}")

if __name__ == '__main__':
    import sys
    
    if len(sys.argv) > 1:
        input_file = sys.argv[1]
    else:
        input_file = 'verify/error_types_distribution/complete_error_analysis.json'
    
    analyze_families(input_file)
