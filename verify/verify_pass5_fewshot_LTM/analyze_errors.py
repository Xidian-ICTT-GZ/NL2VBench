#!/usr/bin/env python3
"""
分析所有pass@5验证结果中的错误类型排行
"""

import json
import re
from pathlib import Path
from collections import Counter
from typing import Dict, List, Tuple

def extract_error_type(output: str) -> str:
    """
    从Verus错误输出中提取错误类型
    """
    if not output:
        return "unknown"
    
    # 提取第一个error关键字之后的错误类型
    # 常见模式: "error: xxx" 或 "error[Exxx]: xxx"
    
    # 尝试匹配 error[Exxx] 格式
    match = re.search(r'error\[([^\]]+)\]:', output)
    if match:
        return match.group(1)
    
    # 尝试匹配 error: 格式，提取冒号后的描述（截断到第一个换行）
    match = re.search(r'error:\s*([^\n]+)', output)
    if match:
        error_msg = match.group(1).strip()
        # 限制长度
        if len(error_msg) > 80:
            error_msg = error_msg[:80]
        return error_msg
    
    return "unknown"

def process_results_file(file_path: Path) -> List[Tuple[str, str]]:
    """
    处理单个结果文件，返回(错误类型, 详情)列表
    """
    errors = []
    
    try:
        with open(file_path, 'r') as f:
            data = json.load(f)
            
        # data可能是列表或字典
        if isinstance(data, dict) and 'results' in data:
            results = data['results']
        elif isinstance(data, list):
            results = data
        else:
            return errors
        
        for result in results:
            # 检查是否有验证详情
            if 'verification_details' not in result:
                continue
                
            for detail in result['verification_details']:
                if not detail.get('success', False) and 'output' in detail:
                    error_type = extract_error_type(detail['output'])
                    errors.append((error_type, detail['output']))
    
    except Exception as e:
        print(f"Warning: 处理文件 {file_path} 时出错: {e}")
    
    return errors

def main():
    """主函数"""
    results_dir = Path(__file__).parent
    
    # 收集所有JSON文件
    json_files = sorted(results_dir.glob('verification_results_*.json'))
    json_files = [f for f in json_files if '_summary' not in f.name]
    
    print(f"找到 {len(json_files)} 个结果文件")
    
    # 统计错误类型
    error_counter = Counter()
    total_errors = 0
    error_samples = {}  # 保存每个错误类型的样本
    
    for json_file in json_files:
        print(f"处理: {json_file.name}")
        errors = process_results_file(json_file)
        
        for error_type, output in errors:
            total_errors += 1
            error_counter[error_type] += 1
            
            # 保存第一个样本
            if error_type not in error_samples:
                error_samples[error_type] = output[:200]  # 截断前200字符
    
    # 输出结果
    print("\n" + "="*80)
    print("错误类型排行榜 (前30名)")
    print("="*80)
    print(f"总错误数: {total_errors}")
    print(f"不同错误类型数: {len(error_counter)}\n")
    
    # 排序并输出前30名
    top_30 = error_counter.most_common(30)
    
    for rank, (error_type, count) in enumerate(top_30, 1):
        percentage = (count / total_errors * 100) if total_errors > 0 else 0
        print(f"{rank}. {error_type}")
        print(f"   发生次数: {count} ({percentage:.2f}%)")
        print(f"   示例: {error_samples[error_type]}")
        print()
    
    # 保存详细报告
    report_file = results_dir / "error_analysis_report.json"
    report = {
        "total_errors": total_errors,
        "unique_error_types": len(error_counter),
        "top_30": [
            {
                "rank": rank,
                "error_type": error_type,
                "count": count,
                "percentage": count / total_errors * 100 if total_errors > 0 else 0,
                "sample": error_samples[error_type]
            }
            for rank, (error_type, count) in enumerate(top_30, 1)
        ],
        "all_errors": dict(error_counter.most_common(100))  # 保存前100名
    }
    
    with open(report_file, 'w') as f:
        json.dump(report, f, indent=2, ensure_ascii=False)
    
    print(f"\n详细报告已保存到: {report_file}")
    
    # 生成CSV供分析
    csv_file = results_dir / "error_analysis_top50.csv"
    with open(csv_file, 'w') as f:
        f.write("rank,error_type,count,percentage\n")
        for rank, (error_type, count) in enumerate(error_counter.most_common(100), 1):
            percentage = count / total_errors * 100 if total_errors > 0 else 0
            # 转义CSV中的特殊字符
            error_type_escaped = error_type.replace('"', '""')
            f.write(f'{rank},"{error_type_escaped}",{count},{percentage:.2f}\n')
    
    print(f"CSV报告已保存到: {csv_file}")

if __name__ == '__main__':
    main()
