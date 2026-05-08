#!/usr/bin/env python3
# -*- coding: utf-8 -*-

import argparse
import json
import re
from pathlib import Path
from collections import defaultdict, Counter
from typing import Dict, List, Tuple


def classify_error_detailed(error_msg: str) -> Tuple[str, str]:
    """
    更细粒度的错误分类
    返回 (主分类, 子分类)
    """
    if not error_msg:
        return "no_error", "empty_error_message"
    
    error_lower = error_msg.lower()
    
    # Verus 导入和宏相关
    if re.search(r"cannot find macro `verus`", error_msg):
        return "import_error", "missing_verus_macro"
    if re.search(r"cannot find.*`vstd`", error_msg):
        return "import_error", "missing_vstd_module"
    if re.search(r"use of undeclared crate.*vstd", error_msg):
        return "import_error", "vstd_not_declared"
    
    # 语法错误 - 细分
    if re.search(r"expected `\{`, found", error_msg):
        return "syntax_error", "missing_curly_brace"
    if re.search(r"expected `;`", error_msg):
        return "syntax_error", "missing_semicolon"
    if re.search(r"expected `\)`, found", error_msg):
        return "syntax_error", "missing_closing_paren"
    if re.search(r"expected identifier, found", error_msg):
        return "syntax_error", "expected_identifier"
    if re.search(r"unexpected token", error_msg):
        return "syntax_error", "unexpected_token"
    if re.search(r"this file contains an unclosed delimiter", error_msg):
        return "syntax_error", "unclosed_delimiter"
    if re.search(r"expected one of", error_msg):
        return "syntax_error", "expected_one_of"
    if re.search(r"expected.*found `.*`", error_msg):
        return "syntax_error", "unexpected_symbol"
    if re.search(r"expected `->`, `where`, or `\{`", error_msg):
        return "syntax_error", "function_signature_error"
    
    # 符号未定义
    if re.search(r"cannot find value `.*` in this scope", error_msg):
        return "undefined_symbol", "undefined_value"
    if re.search(r"cannot find type `.*` in this scope", error_msg):
        return "undefined_symbol", "undefined_type"
    if re.search(r"cannot find function `.*`", error_msg):
        return "undefined_symbol", "undefined_function"
    if re.search(r"cannot find.*in this scope", error_msg):
        return "undefined_symbol", "undefined_generic"
    
    # main 函数
    if re.search(r"`main` function not found", error_msg):
        return "main_function", "missing_main"
    
    # 类型错误 - 细分
    if re.search(r"mismatched types.*expected `bool`", error_msg):
        return "type_error", "expected_bool"
    if re.search(r"mismatched types.*expected `int`", error_msg):
        return "type_error", "expected_int"
    if re.search(r"mismatched types", error_msg):
        return "type_error", "type_mismatch"
    if re.search(r"no implementation for `.*[+\-*/].*`", error_msg):
        return "type_error", "operator_not_implemented"
    
    # Verus 验证错误 - 细分
    if re.search(r"postcondition not satisfied", error_msg):
        return "verus_verification", "postcondition_failed"
    if re.search(r"precondition not satisfied", error_msg):
        return "verus_verification", "precondition_failed"
    if re.search(r"invariant not satisfied", error_msg):
        return "verus_verification", "invariant_failed"
    if re.search(r"assertion failed", error_msg):
        return "verus_verification", "assertion_failed"
    if re.search(r"ensures clause.*false", error_msg):
        return "verus_verification", "ensures_always_false"
    if re.search(r"requires clause.*false", error_msg):
        return "verus_verification", "requires_always_false"
    if re.search(r"decreases.*not satisfied", error_msg):
        return "verus_verification", "termination_proof_failed"
    
    # Verus 特定语法
    if re.search(r"proof block", error_msg):
        return "verus_syntax", "proof_block_error"
    if re.search(r"spec function", error_msg):
        return "verus_syntax", "spec_function_error"
    if re.search(r"ghost", error_msg):
        return "verus_syntax", "ghost_error"
    if re.search(r"exec", error_msg):
        return "verus_syntax", "exec_mode_error"
    if re.search(r"requires|ensures|invariant", error_msg) and not re.search(r"error\[E\d+\]", error_msg):
        return "verus_syntax", "spec_keyword_misuse"
    if re.search(r"verus!", error_msg):
        return "verus_syntax", "verus_macro_error"
    
    # 生命周期和借用
    if re.search(r"lifetime", error_msg):
        return "ownership_error", "lifetime_error"
    if re.search(r"borrow.*mutable", error_msg):
        return "ownership_error", "mutable_borrow_error"
    if re.search(r"cannot borrow", error_msg):
        return "ownership_error", "borrow_error"
    if re.search(r"moved value", error_msg):
        return "ownership_error", "use_after_move"
    
    # 超时
    if re.search(r"timeout|timed out", error_msg):
        return "timeout", "verification_timeout"
    
    # 编译错误代码
    error_code_match = re.search(r"error\[E(\d+)\]", error_msg)
    if error_code_match:
        code = error_code_match.group(1)
        return "rust_compile_error", f"E{code}"
    
    # 路径和文件系统错误
    if re.search(r"No such file|file not found|cannot find file", error_msg, re.IGNORECASE):
        return "file_system_error", "file_not_found"
    if re.search(r"permission denied", error_msg, re.IGNORECASE):
        return "file_system_error", "permission_denied"
    
    # Verus工具链错误
    if re.search(r"the generated executable.*contains a main", error_msg):
        return "verus_toolchain", "unexpected_main_in_library"
    if re.search(r"verus.*failed|verus.*error", error_msg, re.IGNORECASE):
        return "verus_toolchain", "verus_tool_error"
    
    # 宏展开错误
    if re.search(r"macro expansion", error_msg):
        return "macro_error", "macro_expansion_failed"
    if re.search(r"procedural macro", error_msg):
        return "macro_error", "proc_macro_error"
    
    # 特征/trait相关
    if re.search(r"trait.*not implemented", error_msg):
        return "trait_error", "trait_not_implemented"
    if re.search(r"trait bound", error_msg):
        return "trait_error", "trait_bound_not_satisfied"
    
    # 通用分类
    if re.search(r"error", error_lower):
        return "other_error", "unclassified_error"
    
    return "unknown", "unknown_error"


def extract_error_context(error_msg: str, max_len: int = 150) -> str:
    """提取错误的关键上下文"""
    if not error_msg:
        return ""
    
    # 提取第一个 error: 行
    lines = error_msg.split('\n')
    for line in lines:
        if 'error' in line.lower() and len(line.strip()) > 0:
            return line.strip()[:max_len]
    
    return error_msg[:max_len]


def analyze_pass1_results(jsonl_file: Path, global_stats: Dict, error_stats: Dict):
    """分析 pass@1 的 jsonl 结果文件"""
    print(f"  处理 pass@1: {jsonl_file.name}")
    
    # 解析文件名获取类别和模型
    filename = jsonl_file.stem
    parts = filename.split('_')
    if len(parts) >= 2:
        category = '_'.join(parts[:-1])  # p1_fewshot, p2_zeroshot 等
        model = parts[-1]
    else:
        category = filename
        model = "unknown"
    
    # 读取结果
    with open(jsonl_file, 'r') as f:
        for line in f:
            data = json.loads(line)
            
            if data.get('type') == 'result':
                sample_id = data.get('sample_id', 'unknown')
                success = data.get('success', False)
                error_msg = data.get('error', '')
                
                # 更新统计
                global_stats['total_samples'] += 1
                global_stats['by_category'][category]['total'] += 1
                global_stats['by_model'][model]['total'] += 1
                
                if success:
                    global_stats['total_succeeded'] += 1
                    global_stats['by_category'][category]['succeeded'] += 1
                    global_stats['by_model'][model]['succeeded'] += 1
                else:
                    global_stats['total_failed'] += 1
                    global_stats['by_category'][category]['failed'] += 1
                    global_stats['by_model'][model]['failed'] += 1
                    
                    # 分类错误
                    main_cat, sub_cat = classify_error_detailed(error_msg)
                    full_cat = f"{main_cat}::{sub_cat}"
                    
                    error_stats['main_categories'][main_cat] += 1
                    error_stats['sub_categories'][sub_cat] += 1
                    error_stats['full_categories'][(main_cat, sub_cat)] += 1
                    
                    # 保存错误示例
                    context = extract_error_context(error_msg)
                    if len(error_stats['contexts'][full_cat]) < 5:
                        error_stats['contexts'][full_cat].append({
                            'file': jsonl_file.name,
                            'sample_id': sample_id,
                            'context': context,
                            'source': 'pass@1'
                        })


def analyze_pass5_results(json_file: Path, global_stats: Dict, error_stats: Dict):
    """分析 pass@5 的 json 结果文件"""
    print(f"  处理 pass@5: {json_file.name}")
    
    # 解析文件名获取类别和模型
    filename = json_file.stem
    if filename.endswith('_summary'):
        return  # 跳过 summary 文件
    
    # 跳过其他非验证结果文件
    if not any(filename.startswith(prefix) for prefix in ['p1_', 'p2_', 'p3_']):
        print(f"    跳过非验证结果文件: {filename}")
        return
    
    parts = filename.split('_')
    if len(parts) >= 2:
        category = '_'.join(parts[:-1])  # p1_fewshot, p2_zeroshot 等
        model = parts[-1]
    else:
        category = filename
        model = "unknown"
    
    # 读取 JSON 数组
    try:
        with open(json_file, 'r') as f:
            samples = json.load(f)
        
        # 检查是否为列表
        if not isinstance(samples, list):
            print(f"    跳过非列表格式文件: {filename}")
            return
    except Exception as e:
        print(f"    读取文件失败: {e}")
        return
    
    for sample in samples:
        # 检查样本格式
        if not isinstance(sample, dict):
            continue
        
        # 检查是否包含必需字段
        if 'sample_id' not in sample and 'verification_details' not in sample:
            continue
        
        sample_id = sample.get('sample_id', 'unknown')
        verification_details = sample.get('verification_details', [])
        
        # 统计每个文件的验证结果
        for detail in verification_details:
            file_name = detail.get('file', 'unknown')
            success = detail.get('success', False)
            error_msg = detail.get('output', '')
            
            # 更新统计
            global_stats['total_samples'] += 1
            global_stats['by_category'][category]['total'] += 1
            global_stats['by_model'][model]['total'] += 1
            
            if success:
                global_stats['total_succeeded'] += 1
                global_stats['by_category'][category]['succeeded'] += 1
                global_stats['by_model'][model]['succeeded'] += 1
            else:
                global_stats['total_failed'] += 1
                global_stats['by_category'][category]['failed'] += 1
                global_stats['by_model'][model]['failed'] += 1
                
                # 分类错误
                main_cat, sub_cat = classify_error_detailed(error_msg)
                full_cat = f"{main_cat}::{sub_cat}"
                
                error_stats['main_categories'][main_cat] += 1
                error_stats['sub_categories'][sub_cat] += 1
                error_stats['full_categories'][(main_cat, sub_cat)] += 1
                
                # 保存错误示例
                context = extract_error_context(error_msg)
                if len(error_stats['contexts'][full_cat]) < 5:
                    error_stats['contexts'][full_cat].append({
                        'file': json_file.name,
                        'sample_id': f"{sample_id}/{file_name}",
                        'context': context,
                        'source': 'pass@5'
                    })


def analyze_all_verification_results(base_directory: Path) -> Dict:
    """分析所有目录下的验证结果（pass@1 和 pass@5）"""
    
    # 全局统计
    global_stats = {
        'total_files': 0,
        'total_samples': 0,
        'total_succeeded': 0,
        'total_failed': 0,
        'by_category': defaultdict(lambda: {'total': 0, 'succeeded': 0, 'failed': 0}),
        'by_model': defaultdict(lambda: {'total': 0, 'succeeded': 0, 'failed': 0}),
        'by_pass_type': defaultdict(lambda: {'total': 0, 'succeeded': 0, 'failed': 0}),
    }
    
    # 错误统计
    error_stats = {
        'main_categories': Counter(),
        'sub_categories': Counter(),
        'full_categories': Counter(),
        'contexts': defaultdict(list)
    }
    
    # 定义要扫描的目录
    directories = [
        ('verify_pass1', 'pass@1', '*.jsonl'),
        ('verify_pass5_fewshot', 'pass@5', '*.json'),
        ('verify_pass5_fewshot_COT', 'pass@5', '*.json'),
        ('verify_pass5_fewshot_LTM', 'pass@5', '*.json'),
        ('verify_pass5_zeroshot', 'pass@5', '*.json'),
    ]
    
    for dir_name, pass_type, pattern in directories:
        dir_path = base_directory / dir_name
        
        if not dir_path.exists():
            print(f"跳过不存在的目录: {dir_path}")
            continue
        
        print(f"\n扫描目录: {dir_name} ({pass_type})")
        
        # 获取文件列表
        if pass_type == 'pass@1':
            files = list(dir_path.glob(pattern))
            for file in files:
                analyze_pass1_results(file, global_stats, error_stats)
                global_stats['total_files'] += 1
                global_stats['by_pass_type']['pass@1']['total'] += 1
        else:  # pass@5
            files = [f for f in dir_path.glob(pattern) if not f.stem.endswith('_summary')]
            for file in files:
                analyze_pass5_results(file, global_stats, error_stats)
                global_stats['total_files'] += 1
                global_stats['by_pass_type']['pass@5']['total'] += 1
    
    return {
        'global_stats': global_stats,
        'error_main_categories': error_stats['main_categories'],
        'error_sub_categories': error_stats['sub_categories'],
        'error_full_categories': error_stats['full_categories'],
        'error_contexts': error_stats['contexts']
    }


def print_statistics(analysis: Dict, top_n: int = 50):
    """打印统计结果"""
    stats = analysis['global_stats']
    
    print("\n" + "="*80)
    print("全局统计")
    print("="*80)
    print(f"处理文件数: {stats['total_files']}")
    print(f"总样本数:   {stats['total_samples']}")
    print(f"成功样本:   {stats['total_succeeded']} ({stats['total_succeeded']/stats['total_samples']*100:.2f}%)")
    print(f"失败样本:   {stats['total_failed']} ({stats['total_failed']/stats['total_samples']*100:.2f}%)")
    
    # 按 pass 类型统计
    print("\n" + "="*80)
    print("按验证类型统计")
    print("="*80)
    for pass_type, counts in sorted(stats['by_pass_type'].items()):
        print(f"{pass_type:20s} | 文件数: {counts['total']:4d}")
    
    # 按类别统计
    print("\n" + "="*80)
    print("按实验类别统计")
    print("="*80)
    for category, counts in sorted(stats['by_category'].items()):
        success_rate = counts['succeeded'] / counts['total'] * 100 if counts['total'] > 0 else 0
        print(f"{category:40s} | 总数: {counts['total']:6d} | 成功: {counts['succeeded']:6d} ({success_rate:5.2f}%)")
    
    # 按模型统计
    print("\n" + "="*80)
    print("按模型统计")
    print("="*80)
    for model, counts in sorted(stats['by_model'].items()):
        success_rate = counts['succeeded'] / counts['total'] * 100 if counts['total'] > 0 else 0
        print(f"{model:30s} | 总数: {counts['total']:6d} | 成功: {counts['succeeded']:6d} ({success_rate:5.2f}%)")
    
    # 主分类统计
    print("\n" + "="*80)
    print("错误主分类 Top 20")
    print("="*80)
    for category, count in analysis['error_main_categories'].most_common(20):
        percentage = count / stats['total_failed'] * 100
        print(f"{category:30s} : {count:6d} ({percentage:5.2f}%)")
    
    # 详细分类统计
    print("\n" + "="*80)
    print(f"错误详细分类 Top {top_n}")
    print("="*80)
    print(f"{'主分类':<20s} {'子分类':<30s} {'数量':<9s} {'占比'}")
    print("-"*80)
    
    for (main_cat, sub_cat), count in analysis['error_full_categories'].most_common(top_n):
        percentage = count / stats['total_failed'] * 100
        print(f"{main_cat:<20s} {sub_cat:<30s} {count:<9d} {percentage:5.2f}%")
    
    # 显示每种错误的示例
    print("\n" + "="*80)
    print(f"Top 10 错误类型示例")
    print("="*80)
    
    for i, ((main_cat, sub_cat), count) in enumerate(analysis['error_full_categories'].most_common(10), 1):
        full_cat = f"{main_cat}::{sub_cat}"
        print(f"\n{i}. {full_cat} ({count} 次)")
        print("-"*80)
        
        examples = analysis['error_contexts'][full_cat][:3]
        for j, example in enumerate(examples, 1):
            print(f"  示例 {j} [{example['source']}]: {example['file']} - {example['sample_id']}")
            print(f"         {example['context']}")


def save_detailed_report(analysis: Dict, output_file: Path):
    """保存详细报告为 JSON"""
    
    # 转换 Counter 为普通字典以便序列化
    report = {
        'global_stats': dict(analysis['global_stats']),
        'error_main_categories': dict(analysis['error_main_categories']),
        'error_sub_categories': dict(analysis['error_sub_categories']),
        'error_full_categories': {
            f"{main}::{sub}": count 
            for (main, sub), count in analysis['error_full_categories'].items()
        },
        'error_contexts': dict(analysis['error_contexts'])
    }
    
    # 转换嵌套的 defaultdict
    report['global_stats']['by_category'] = dict(report['global_stats']['by_category'])
    report['global_stats']['by_model'] = dict(report['global_stats']['by_model'])
    report['global_stats']['by_pass_type'] = dict(report['global_stats']['by_pass_type'])
    
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(report, f, indent=2, ensure_ascii=False)
    
    print(f"\n✓ 详细报告已保存到: {output_file}")


def main():
    parser = argparse.ArgumentParser(description='批量分析所有验证结果的错误类型（包括 pass@1 和 pass@5）')
    parser.add_argument('--base-directory', type=Path, default=Path('.'), 
                       help='验证结果的基础目录（默认当前目录，应指向 error_types_distribution 文件夹）')
    parser.add_argument('--top-n', type=int, default=50,
                       help='显示前 N 个错误类型（默认50）')
    parser.add_argument('--output', type=Path, 
                       help='输出详细报告的 JSON 文件')
    
    args = parser.parse_args()
    
    if not args.base_directory.exists():
        print(f"错误: 目录 {args.base_directory} 不存在")
        return 1
    
    print(f"分析基础目录: {args.base_directory}")
    print(f"将扫描以下目录:")
    print(f"  - verify_pass1/ (pass@1 结果)")
    print(f"  - verify_pass5_fewshot/ (pass@5 结果)")
    print(f"  - verify_pass5_fewshot_COT/ (pass@5 结果)")
    print(f"  - verify_pass5_fewshot_LTM/ (pass@5 结果)")
    print(f"  - verify_pass5_zeroshot/ (pass@5 结果)")
    
    # 执行分析
    analysis = analyze_all_verification_results(args.base_directory)
    
    # 打印统计
    print_statistics(analysis, args.top_n)
    
    # 保存报告
    if args.output:
        save_detailed_report(analysis, args.output)
    
    return 0


if __name__ == '__main__':
    exit(main())
