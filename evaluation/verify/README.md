# Verification

Unified verification pipeline for generation outputs (pass@1 and pass@5).

## Directory Structure

- `verify_pass1/` — Pass@1 results (extracted from pass@5 4th generation)
- `verify_pass5_zeroshot/` — Pass@5 zeroshot experiments
- `verify_pass5_fewshot/` — Pass@5 fewshot experiments
- `verify_pass5_fewshot_COT/` — Pass@5 fewshot+COT experiments
- `verify_pass5_fewshot_LTM/` — Pass@5 fewshot+LTM experiments
- `summary_tables/` — CSV summaries with pass rates

## Scripts

### `verify_generation.py`

Main verification script. Scans `generation/` directory and verifies all generated Rust code using Verus.

**Usage:**

```bash
python3 verify_generation.py [options]
```

**Key Options:**
- `--config CONFIG` — Path to `config/config.yaml` (must have `verus.verus_path` set)
- `--dataset DATASET` — Path to `dataset/NL2VBench.jsonl` (default: auto-detected)
- `--generation-root DIR` — Generation output root (default: `generation/`)
- `--pipelines {p1,p2,p3}` — Which pipelines to verify (default: all)
- `--variants {zeroshot,fewshot,fewshot+COT,fewshot+LTM}` — Which variants (default: all)
- `--models MODEL [MODEL ...]` — Filter by model names (optional)
- `--pass1-generation N` — Which pass@5 generation to use for pass@1 (1-5, default: 4 = 4th)
- `--timeout SECS` — Verus verification timeout (default: 120s)
- `--dry-run` — Print planned tasks without running
- `--clean-output` — Delete previous results before verifying

**Example:**

```bash
# Verify all pass@5 experiments
python3 verify_generation.py

# Verify only p1_zeroshot, specific models
python3 verify_generation.py --pipelines p1 --variants zeroshot --models gpt-5.2

# Dry-run to see what will be verified
python3 verify_generation.py --dry-run
```

**Output Files:**
- Each pass@5 variant saves JSON results: `verify_pass5_*/p{1,2,3}_variant_model.json`
- Each pass@1 saves JSONL: `verify_pass1/p{1,2,3}_variant_model.jsonl`
- Summary CSV: `summary_tables/pass1_pass5_verification_rates.csv`
- Batch metadata: `verify_pass1/_batch_summary.json`

## Configuration

Ensure `config/config.yaml` includes:

```yaml
verus:
  verus_path: /path/to/verus
```

## Notes

- Pass@1 is derived from the 4th generation of pass@5 by default (configurable)
- Results organized per experiment (pipeline + variant + model)
- All timeout and error outputs preserved in verification details
