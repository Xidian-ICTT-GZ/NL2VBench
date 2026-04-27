import os
import json
import time
import asyncio
import argparse
import logging
from typing import List, Optional, Tuple, Dict, Any
from pathlib import Path

import yaml
from pydantic import BaseModel, Field
from langchain_openai import ChatOpenAI
from langchain_core.messages import SystemMessage, HumanMessage
from tqdm.asyncio import tqdm_asyncio

# Configure logging
logging.basicConfig(level=logging.INFO, format='%(asctime)s - %(levelname)s - %(message)s')
logger = logging.getLogger(__name__)

# -----------------------------
# Models for structured outputs
# -----------------------------
class MetaInfo(BaseModel):
    category: str = Field(
        ..., 
        description="Classify code as 'Simple', 'Loops', or 'Complex'",
        enum=["Simple", "Loops", "Complex"]
    )
    keywords: List[str] = Field(
        ..., 
        description="List of 3-5 keywords describing the algorithm (e.g., ['Binary Search', 'Pointers'])"
    )

class VeriCBenchEntry(BaseModel):
    sample_id: str = Field(description="Keep this as PLACEHOLDER in generation, will be overwritten by script")
    description: str = Field(..., description="Ambiguous-free natural language description of the problem")
    function_signature: str = Field(..., description="The function signature(s) extracted from code")
    test_cases: str = Field(..., description="Self-contained main() function with assertions")
    meta: MetaInfo = Field(..., description="Metadata about the problem")

# -----------------------------
# Config & Prompt loading
# -----------------------------
SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = Path(__file__).resolve().parents[2]
CONFIG_PATH = REPO_ROOT / "config" / "config.yaml"

# Prompt template lives in the same folder
from reverse import REVERSE_ENGINEERING_SYSTEM_PROMPT, REVERSE_ENGINEERING_USER_PROMPT


def load_config(config_path: Path) -> Dict[str, Any]:
    if not config_path.exists():
        raise FileNotFoundError(f"Config file not found: {config_path}")
    with config_path.open("r", encoding="utf-8") as f:
        cfg = yaml.safe_load(f)
    return cfg or {}


class SampleCreator:
    def __init__(self, cfg: Dict[str, Any], model_key: str = "gpt-5.2", override: Dict[str, Any] | None = None):
        models_cfg = cfg.get("models", {})
        gen_cfg = cfg.get("generation", {})
        model_cfg = models_cfg.get(model_key, {})
        if not model_cfg:
            raise ValueError(f"Model config '{model_key}' not found in config.yaml")

        api_key = (override or {}).get("api_key") or model_cfg.get("api_key", os.environ.get("OPENAI_API_KEY", ""))
        base_url = (override or {}).get("base_url") or model_cfg.get("base_url", os.environ.get("OPENAI_BASE_URL", None))
        # Normalize base_url: some configs may include "/chat/completions" which
        # is not expected by the OpenAI-compatible client. We strip it to the root.
        if isinstance(base_url, str) and base_url.endswith("/chat/completions"):
            base_url = base_url.rsplit("/chat/completions", 1)[0]
        model_name = (override or {}).get("model_name") or model_cfg.get("model_name", model_key)

        # Defaults
        timeout = float(gen_cfg.get("timeout", 120.0))
        if override and override.get("timeout") is not None:
            try:
                timeout = float(override.get("timeout"))
            except Exception:
                pass
        max_retries = int(gen_cfg.get("max_retries", 3))
        temperature = 0.1
        chat_max_tokens = int((override or {}).get("max_tokens", gen_cfg.get("chat_max_tokens", 2048)))

        # Create LangChain OpenAI-compatible client
        self.llm = ChatOpenAI(
            model=model_name,
            api_key=api_key,
            base_url=base_url,
            timeout=timeout,
            temperature=temperature,
            max_retries=max_retries,
            max_tokens=chat_max_tokens,
        ).with_structured_output(VeriCBenchEntry, method="json_schema", strict=True, include_raw=True)

        self.system_prompt = [SystemMessage(content=REVERSE_ENGINEERING_SYSTEM_PROMPT)]

    async def acreate_sample(self, file_content: str) -> Tuple[Optional[VeriCBenchEntry], Dict[str, Any], float]:
        formatted_user_prompt = REVERSE_ENGINEERING_USER_PROMPT.format(file_content=file_content)
        messages = self.system_prompt + [HumanMessage(content=formatted_user_prompt)]

        start_time = time.time()
        try:
            response = await self.llm.ainvoke(messages)
            duration = time.time() - start_time

            parsed_result = response["parsed"]
            raw_response = response["raw"]
            token_usage = raw_response.response_metadata.get("token_usage", {}) if hasattr(raw_response, "response_metadata") else {}

            return parsed_result, token_usage, duration
        except Exception as e:
            logger.error(f"Error generating sample: {e}")
            return None, {}, time.time() - start_time


async def process_single_sample(creator: SampleCreator, line: str, semaphore: asyncio.Semaphore) -> Optional[Dict[str, Any]]:
    async with semaphore:
        try:
            input_sample = json.loads(line)

            file_content = input_sample.get("file_content", "")
            if not file_content:
                return None

            result, token_usage, duration = await creator.acreate_sample(file_content)
            if not result:
                return None

            final_sample = {
                "sample_id": input_sample.get("sample_id"),
                # Merge description with function_signature for a concise problem+signature field
                "description": f"{result.description}\n{result.function_signature}",
                "meta": {
                    "description": result.description,
                    "function_signature": result.function_signature,
                    "test_cases": result.test_cases,
                    "category": result.meta.category,
                    "keywords": result.meta.keywords,
                    "file_content": file_content,
                }
            }

            return {
                "final_sample": final_sample,
                "token_usage": token_usage,
                "duration": duration
            }
        except Exception as e:
            logger.error(f"Error processing sample: {e}")
            return None


async def main():
    parser = argparse.ArgumentParser(description="Generate Verus-Bench samples from Rust/Verus code")
    parser.add_argument("--limit", type=int, default=1, help="Number of samples to process")
    parser.add_argument("--concurrency", type=int, default=8, help="Number of concurrent requests")
    parser.add_argument("--input_file", type=str, default=str(SCRIPT_DIR / "input.jsonl"), help="Input JSONL file with file_content")
    parser.add_argument("--output_file", type=str, default=str(SCRIPT_DIR / "verus_bench.jsonl"), help="Output JSONL file")
    parser.add_argument("--model", type=str, default="gpt-5.2", help="Model key in config/config.yaml")
    # Optional overrides for API routing
    parser.add_argument("--base_url", type=str, default=None, help="Override base URL for OpenAI-compatible API")
    parser.add_argument("--api_key", type=str, default=None, help="Override API key (env OPENAI_API_KEY if omitted)")
    parser.add_argument("--model_name", type=str, default=None, help="Override actual remote model name")
    parser.add_argument("--timeout", type=float, default=None, help="Override request timeout in seconds")
    parser.add_argument("--max_tokens", type=int, default=None, help="Override max output tokens")

    args = parser.parse_args()

    cfg = load_config(CONFIG_PATH)
    override = {
        k: v for k, v in {
            "base_url": args.base_url,
            "api_key": args.api_key,
            "model_name": args.model_name,
            "timeout": args.timeout,
            "max_tokens": args.max_tokens,
        }.items() if v is not None
    }
    creator = SampleCreator(cfg, model_key=args.model, override=override if override else None)
    semaphore = asyncio.Semaphore(args.concurrency)

    if not os.path.exists(args.input_file):
        logger.error(f"Input file not found: {args.input_file}")
        return

    with open(args.input_file, "r", encoding="utf-8") as f:
        lines = f.readlines()

    lines_to_process = lines[:args.limit]
    logger.info(f"Starting processing of {len(lines_to_process)} samples with concurrency {args.concurrency}")

    global_start_time = time.time()
    tasks = [process_single_sample(creator, line, semaphore) for line in lines_to_process]
    results = await tqdm_asyncio.gather(*tasks)
    total_wall_time = time.time() - global_start_time

    valid_results = [r for r in results if r is not None]

    total_input_tokens = 0
    total_output_tokens = 0
    total_compute_time = 0.0

    output_samples = []
    for r in valid_results:
        output_samples.append(r["final_sample"])
        usage = r["token_usage"] or {}
        total_input_tokens += usage.get("prompt_tokens", 0) or usage.get("input_tokens", 0)
        total_output_tokens += usage.get("completion_tokens", 0) or usage.get("output_tokens", 0)
        total_compute_time += r["duration"]

    # Write output
    output_dir = os.path.dirname(args.output_file)
    if output_dir and not os.path.exists(output_dir):
        os.makedirs(output_dir)

    with open(args.output_file, "w", encoding="utf-8") as f:
        for sample in output_samples:
            f.write(json.dumps(sample, ensure_ascii=False) + "\n")

    # Print Stats
    avg_time_per_sample = total_compute_time / len(valid_results) if valid_results else 0
    parallel_efficiency = (total_compute_time / total_wall_time) if total_wall_time > 0 else 0

    print("\n" + "="*50)
    print("PROCESSING SUMMARY")
    print("="*50)
    print(f"Total Samples Processed: {len(valid_results)}/{len(lines_to_process)}")
    print(f"Total Input Tokens:      {total_input_tokens}")
    print(f"Total Output Tokens:     {total_output_tokens}")
    print("-" * 30)
    print(f"Total Wall Time:         {total_wall_time:.2f} s")
    print(f"Total Compute Time:      {total_compute_time:.2f} s")
    print(f"Avg Time per Sample:     {avg_time_per_sample:.2f} s")
    print(f"Parallel Efficiency:     {parallel_efficiency:.2f}x")
    print("="*50)
    print(f"Output saved to: {args.output_file}")


if __name__ == "__main__":
    asyncio.run(main())
