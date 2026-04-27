#!/usr/bin/env python3
# -*- coding: utf-8 -*-

"""
Pass@5 Verification Script for NL2V Project
Verifies all pass@5 experiments across three pipelines (P1, P2, P3) with fewshot+LTM prompting.
Rule: Pass@5 is considered successful if at least one of the five attempts passes verification.

Each pipeline+model combination generates separate output files for easier analysis and repair planning.
"""

import os
import json
import subprocess
import sys
import re
from pathlib import Path
from datetime import datetime
from typing import Dict, List, Tuple, Optional
from concurrent.futures import ThreadPoolExecutor, as_completed
import argparse

# Constants
WORKDIR = Path(__file__).parent.parent.parent.absolute()
EXP_GENERATION_DIR = WORKDIR / 'exp_generation'
VERIFY_OUTPUT_DIR = Path(__file__).parent.absolute()

# Pipelines and models to verify
PIPELINES = ['p1_fewshot+LTM', 'p2_fewshot+LTM', 'p3_fewshot+LTM']
VERUS_CMD = 'verus'


def run_verus_verification(rs_file: Path) -> Tuple[bool, str]:
    """
    Run Verus verification on a single .rs file.
    
    Args:
        rs_file: Path to the .rs file to verify
    
    Returns:
        Tuple of (success: bool, output: str)
    """
    try:
        result = subprocess.run(
            [VERUS_CMD, str(rs_file)],
            capture_output=True,
            text=True,
            timeout=120  # 2 minutes timeout
        )
        
        success = result.returncode == 0
        output = result.stdout + result.stderr
        
        return success, output
    except subprocess.TimeoutExpired:
        return False, "Verification timeout (120s)"
    except FileNotFoundError:
        return False, f"Verus command not found. Please ensure 'verus' is in your PATH."
    except Exception as e:
        return False, f"Error running verus: {str(e)}"


def has_verus_block_and_function(file_path: Path) -> bool:
    """
    Check if a file has verus! { ... } block and a complete function definition.
    
    Args:
        file_path: Path to the .rs file
    
    Returns:
        True if file has verus! block and function definition
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
        
        # Check for verus! block
        has_verus = bool(re.search(r'verus!\s*\{', content))
        
        # Check for function definition (fn or spec fn)
        has_function = bool(re.search(r'(spec\s+)?fn\s+\w+\s*\(', content))
        
        return has_verus and has_function
    except Exception:
        return False


def select_file_for_repair(pass5_files: List[Path]) -> str:
    """
    Select a file to repair from pass@5 attempts.
    Priority:
    1. Files with verus! { ... } block and complete function definition, earliest first (00 > 01 > ...)
    2. If no suitable file found, select 00.rs
    
    Args:
        pass5_files: List of .rs file paths to choose from
    
    Returns:
        Name of the selected file to repair
    """
    # First pass: find files with verus block and function
    candidates = []
    for rs_file in pass5_files:
        if has_verus_block_and_function(rs_file):
            candidates.append(rs_file)
    
    if candidates:
        # Return the earliest file (00 comes before 01, etc.)
        return candidates[0].name
    
    # Fallback: if no good candidate found, return 00
    for rs_file in pass5_files:
        if rs_file.name.endswith('00.rs') or rs_file.name.endswith('complete00.rs'):
            return rs_file.name
    
    # Last resort: return first file
    if pass5_files:
        return pass5_files[0].name
    
    return "00.rs"


def verify_pass5_sample(sample_dir: Path, pipeline: str, model_name: str) -> Dict:
    """
    Verify a single sample's pass@5 attempts.
    
    Args:
        sample_dir: Path to sample directory (e.g., DAFNY2VERUS-COLLECTION_1)
        pipeline: Pipeline name (e.g., p1_fewshot+LTM)
        model_name: Model name (e.g., claude-sonnet-4-5-20250929)
    
    Returns:
        Dictionary with verification results
    """
    sample_id = sample_dir.name
    model_dir = sample_dir / pipeline / model_name
    
    if not model_dir.exists():
        return {
            'sample_id': sample_id,
            'pipeline': pipeline,
            'model': model_name,
            'status': 'not_found',
            'pass5_result': False,
            'passed_files': [],
            'error': f"Model directory not found: {model_dir}"
        }
    
    # Determine file pattern based on pipeline
    # p1: 00.rs to 04.rs
    # p2 and p3: complete00.rs to complete04.rs
    pass5_files = []
    if pipeline == 'p1_fewshot+LTM':
        # p1 uses 00.rs to 04.rs
        for i in range(5):
            rs_file = model_dir / f"{i:02d}.rs"
            if rs_file.exists():
                pass5_files.append(rs_file)
    else:
        # p2 and p3 use complete00.rs to complete04.rs
        for i in range(5):
            rs_file = model_dir / f"complete{i:02d}.rs"
            if rs_file.exists():
                pass5_files.append(rs_file)
    
    if len(pass5_files) == 0:
        return {
            'sample_id': sample_id,
            'pipeline': pipeline,
            'model': model_name,
            'status': 'no_files',
            'pass5_result': False,
            'passed_files': [],
            'error': f"No pass@5 files found in {model_dir}"
        }
    
    # Verify each file
    verification_results = []
    passed_files = []
    
    for rs_file in pass5_files:
        success, output = run_verus_verification(rs_file)
        
        verification_results.append({
            'file': rs_file.name,
            'success': success,
            'output': output  # Full verification output
        })
        
        if success:
            passed_files.append(rs_file.name)
    
    # Pass@5 rule: at least one must pass
    pass5_result = len(passed_files) > 0
    
    result = {
        'sample_id': sample_id,
        'pipeline': pipeline,
        'model': model_name,
        'status': 'completed',
        'pass5_result': pass5_result,
        'passed_files': passed_files,
        'total_files': len(pass5_files),
        'verification_details': verification_results,
        'timestamp': datetime.now().isoformat()
    }
    
    # If failed, select a file to repair
    if not pass5_result:
        file_to_repair = select_file_for_repair(pass5_files)
        result['file_to_repair'] = file_to_repair
    
    return result


def get_all_samples() -> List[Path]:
    """Get all sample directories from exp_generation."""
    samples = []
    if EXP_GENERATION_DIR.exists():
        for item in EXP_GENERATION_DIR.iterdir():
            if item.is_dir():
                samples.append(item)
    return sorted(samples)


def get_models_for_sample(sample_dir: Path, pipeline: str) -> List[str]:
    """Get all models that have results for a given sample and pipeline."""
    pipeline_dir = sample_dir / pipeline
    if not pipeline_dir.exists():
        return []
    
    models = []
    for item in pipeline_dir.iterdir():
        if item.is_dir():
            models.append(item.name)
    return sorted(models)


def save_results(results: List[Dict], output_file: Path):
    """Save verification results to JSON file."""
    with open(output_file, 'w', encoding='utf-8') as f:
        json.dump(results, f, indent=2, ensure_ascii=False)


def generate_summary(results: List[Dict]) -> Dict:
    """Generate summary statistics from verification results."""
    summary = {
        'total_samples': 0,
        'passed_samples': 0,
        'overall_pass5_rate': 0.0,
        'timestamp': datetime.now().isoformat()
    }
    
    # Process results
    passed_count = 0
    
    for result in results:
        if result['status'] != 'completed':
            continue
        
        summary['total_samples'] += 1
        if result['pass5_result']:
            passed_count += 1
    
    if summary['total_samples'] > 0:
        summary['overall_pass5_rate'] = passed_count / summary['total_samples']
    summary['passed_samples'] = passed_count
    
    return summary


def print_summary(summary: Dict, pipeline: str = None, model: str = None):
    """Print summary statistics to console."""
    print("\n" + "-"*80)
    if pipeline and model:
        print(f"Summary: {pipeline} + {model}")
    else:
        print("Pass@5 Verification Summary")
    print("-"*80)
    print(f"Total Samples: {summary['total_samples']}")
    print(f"Passed Samples: {summary['passed_samples']}")
    print(f"Pass@5 Rate: {summary['overall_pass5_rate']:.2%}")
    print("-"*80)


def main():
    parser = argparse.ArgumentParser(
        description='Verify Pass@5 experiments for all pipelines and models'
    )
    parser.add_argument(
        '--samples',
        nargs='*',
        help='Specific sample IDs to verify (e.g., DAFNY2VERUS-COLLECTION_1). If not specified, all samples will be verified.'
    )
    parser.add_argument(
        '--pipelines',
        nargs='*',
        default=PIPELINES,
        choices=PIPELINES,
        help='Pipelines to verify (default: all three pipelines)'
    )
    parser.add_argument(
        '--models',
        nargs='*',
        help='Specific models to verify (e.g., deepseek-v3.2). If not specified, all models will be verified.'
    )
    parser.add_argument(
        '--max-workers',
        type=int,
        default=4,
        help='Maximum number of parallel verification workers (default: 4)'
    )
    
    args = parser.parse_args()
    
    # Check if verus is available
    try:
        subprocess.run([VERUS_CMD, '--version'], capture_output=True, check=True)
        print(f"✓ Verus verification tool found")
    except (subprocess.CalledProcessError, FileNotFoundError):
        print(f"✗ Error: Verus command '{VERUS_CMD}' not found in PATH")
        print("  Please install Verus or ensure it's in your PATH")
        sys.exit(1)
    
    # Get samples to verify
    if args.samples:
        samples = [EXP_GENERATION_DIR / sample_id for sample_id in args.samples]
        samples = [s for s in samples if s.exists()]
    else:
        samples = get_all_samples()
    
    if not samples:
        print("No samples found to verify.")
        sys.exit(1)
    
    print(f"Found {len(samples)} samples to verify")
    print(f"Pipelines: {', '.join(args.pipelines)}")
    
    # Collect all verification tasks
    tasks = []
    for sample_dir in samples:
        for pipeline in args.pipelines:
            models = get_models_for_sample(sample_dir, pipeline)
            # Filter models if specified
            if args.models:
                models = [m for m in models if m in args.models]
            for model in models:
                tasks.append((sample_dir, pipeline, model))
    
    print(f"Total verification tasks: {len(tasks)}")
    
    # Group results by (pipeline, model) for separate output
    results_by_pipeline_model = {}
    
    # Run verifications in parallel
    with ThreadPoolExecutor(max_workers=args.max_workers) as executor:
        futures = {
            executor.submit(verify_pass5_sample, sample_dir, pipeline, model): (sample_dir, pipeline, model)
            for sample_dir, pipeline, model in tasks
        }
        
        for future in as_completed(futures):
            sample_dir, pipeline, model = futures[future]
            try:
                result = future.result()
                
                # Group by (pipeline, model)
                key = (pipeline, model)
                if key not in results_by_pipeline_model:
                    results_by_pipeline_model[key] = []
                results_by_pipeline_model[key].append(result)
                
                # Print progress
                status_symbol = "✓" if result.get('pass5_result', False) else "✗"
                print(f"{status_symbol} {sample_dir.name} | {pipeline} | {model}")
                
            except Exception as e:
                print(f"✗ Error processing {sample_dir.name}/{pipeline}/{model}: {e}")
    
    # Save results separately for each pipeline+model combination
    print("\n" + "="*80)
    print("Saving results by pipeline and model...")
    print("="*80)
    
    for (pipeline, model), results in sorted(results_by_pipeline_model.items()):
        # Create safe filename (simplified)
        pipeline_safe = pipeline.replace('+', 'plus').replace('/', '_')
        output_filename = f"{pipeline_safe}_{model}.json"
        output_path = VERIFY_OUTPUT_DIR / output_filename
        
        # Save detailed results
        save_results(results, output_path)
        print(f"✓ {output_filename}")
        
        # Save summary for this group
        summary = generate_summary(results)
        summary_file = output_path.with_name(output_path.stem + '_summary.json')
        with open(summary_file, 'w', encoding='utf-8') as f:
            json.dump(summary, f, indent=2, ensure_ascii=False)
        print(f"  └─ Summary: {summary_file.name}")
        
        # Print summary
        print_summary(summary, pipeline, model)
    
    print("="*80)
    print("Verification complete!")
    print("="*80 + "\n")


if __name__ == '__main__':
    main()
