# NL2VBench

NL2VBench is a research repository for natural-language-to-Verus generation and verification experiments. It includes:

- Dataset construction and preprocessing artifacts  
- Benchmark datasets  
- Prompt templates  
- Experiment and batch execution scripts  
- Generation outputs and verification results  

---

## Repository Overview

- `config/` — Unified configuration files (models, generation settings, Verus paths, etc.)
- `construction/` — Dataset construction pipeline (source data, cleaned data, reverse construction)
- `dataset/` — Benchmark datasets and few-shot exemplars
- `prompt/` — Prompt templates for different experimental pipelines
- `script/` — Experiment runners and repair utilities
- `generation/` — Model generation outputs (organized by sample)
- `evaluation/verify/` — Verification scripts, outputs, and summary results
- `evaluation/SVC/` — Expert review CSVs and SVC-related annotations

---

## Key Directories

### `construction/`

Contains materials related to benchmark construction and data transformation, including:

- `source/` — Source datasets and related materials
- `cleaned/` — Cleaned intermediate data
- `generation/` — Scripts and I/O files for reverse construction from code to JSONL

For additional details, see:

`construction/generation/README.md`

---

### `dataset/`

Contains the benchmark data used in experiments:

- `NL2VBench.jsonl` — Main dataset file
- `NL2VBench/` — Per-sample organized structure
- `fewshot/` — Few-shot exemplars
- `statistics.csv` — Basic dataset statistics

For additional details, see:

`dataset/README.md`

---

## Main Entry Points

- Single-pipeline experiments: `script/run_p*_*.py`
- Batch execution: `script/run_all_experiments.py`
- Missing-output repair: `script/repair_missing_generations.py`
- Unified verification: `evaluation/verify/verify_generation.py`

---

## Configuration

Please complete the settings in `config/config.yaml` before running experiments:

- Model `api_key` / `base_url`
- `verus.verus_path` (for local verification)

## Reproducibility

The verification results in this repository were produced with Verus version `0.2026.01.10.531beb1`.
If you rerun the experiments, using the same Verus version is recommended for consistent results.

Default paths are aligned to:

- Dataset: `dataset/NL2VBench.jsonl`
- Output directory: `generation/`

---

## Notes

This README provides only a repository-level overview.  
Detailed formats, construction procedures, and experimental conventions are documented in subdirectory READMEs and corresponding scripts.