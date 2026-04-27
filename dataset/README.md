# NL2VBench Dataset

This directory contains the NL2VBench benchmark dataset, organized in both consolidated and per-instance formats for convenient use.

## Directory Structure

- `NL2VBench.jsonl`: Main dataset file in JSONL format (one sample per line)
- `NL2VBench/`: Individual sample directories organized by `sample_id`
- `fewshot/`: Few-shot exemplar files used to replace placeholders in prompt templates (`exp1*`, `exp2*`)
- `statistics.csv`: Summary statistics of sample counts by `category`

## Sample Format

Each sample is stored under `NL2VBench/<sample_id>/` and includes:

- `description.txt`: Natural-language task description
- `verified_verus.rs`: Reference verified Verus implementation

## Statistics

`statistics.csv` contains two columns:

- `category`
- `count`

This file provides a quick overview of the category distribution of the dataset.

## Usage Scenarios

This dataset format supports:

- Instance-level browsing and retrieval
- Data preprocessing for training or evaluation
- Paired analysis of natural-language specifications and verified code
- Category-based sampling and statistical analysis