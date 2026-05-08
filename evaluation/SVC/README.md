# SVC

This folder contains expert review annotations for the verification results.

- `expert_review_pass5.csv` records manual labels for pass@5 outputs.
- Columns: `sample_id`, `version`, `pipeline`, `model`, `prompt_strategy`, `svc_label`, `issue_type`.
- `svc_label` uses `1` for accepted/valid cases and `0` for cases with issues.