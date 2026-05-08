# Failure Modes Analysis

Analyze pass@5 verification failures from `evaluation/verify` and classify errors into error families.

## Scripts

### `analyze_error_families.py`

Scans pass@5 results and groups errors into six categories:
- **F1_Syntax**: Syntax errors
- **F2_Environment**: Import errors, undefined symbols, missing main
- **F3_Type**: Type mismatches, ownership/borrow issues
- **F4_Spec_Construction**: Verus spec language errors (ghost, spec, proof, exec)
- **F5_Verification_Logic**: Failed postconditions, invariants, assertions
- **F6_Other**: Unclassified errors

**Usage:**

```bash
python3 analyze_error_families.py [--base-directory PATH] [--output-directory PATH]
```

**Arguments:**
- `--base-directory`: Path to `evaluation/verify` (default: `../verify`)
- `--output-directory`: Output directory (default: `outputs/pass5_error_analysis`)

**Outputs:**
- `pass5_complete_error_analysis.json` — Detailed error counts by category
- `pass5_error_families_analysis.json` — Family-level statistics
- `pass5_full_category_counts.csv` — All error types ranked
- `pass5_error_family_counts.csv` — Error families ranked
- `pass5_experiment_family_distribution.csv` — Per-experiment error family breakdown

### `batch_classify_errors.py`

Utility module with error classification logic:
- `classify_error_detailed(error_msg: str) → (main_category, sub_category)`
- `extract_error_context(error_msg: str) → str`

Used internally by `analyze_error_families.py`.

## Example

```bash
cd evaluation/failure_modes
python3 analyze_error_families.py --output-directory ./results
```

Check `./results/pass5_experiment_family_distribution.csv` for per-model error breakdown.
