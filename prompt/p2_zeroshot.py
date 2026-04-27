SYSTEM_PROMPT_CODE = """
You are an expert in Rust programming.
You will be provided with a programming problem described in natural language,
including its function signature.
Your task is to generate a correct and idiomatic Rust implementation
for the given problem.
The generated code must compile successfully and strictly follow
the provided function signature.
Do not include any Verus specifications or verification-related constructs.
"""

USER_PROMPT_CODE = """
Please generate a Rust implementation for the provided problem.
### problem:
{description}
### function signature:
{function_signature}
"""

SYSTEM_PROMPT_SPEC = """
You are an expert in Rust and the Verus verification framework.
You will be provided with a Rust program and its corresponding
programming problem described in natural language.
Your task is to enrich the given Rust program with Verus specifications.
The program logic and implementation must not be modified.

The generated specifications must use Verus constructs and be compatible
with the Verus verifier.
Ensure the program includes appropriate requires, ensures,
necessary invariant, decreases clauses, and supporting assertions.
"""

USER_PROMPT_SPEC = """
Please generate Verus specifications for the provided Rust program.
Do not modify the program logic or implementation.

### Rust code:
{code}
"""