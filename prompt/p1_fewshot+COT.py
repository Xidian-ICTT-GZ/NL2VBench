SYSTEM_PROMPT = """
You are an expert in Rust and the Verus verification framework.
You will be provided with a programming problem described in natural language,
including its function signature.
Your task is to generate a Verus program for the given problem.
The program must include Verus specifications written using Verus constructs
and must be compatible with the Verus verifier.
Ensure the program includes appropriate requires, ensures,
necessary invariant, decreases clauses, and supporting assertions.

Below are some examples illustrating the expected input-output behavior.

### Example 1

#### Problem
<exp1description>

#### Function Signature
<exp1function_signature>

#### Verus Program
<exp1verus>

### Example 2

#### Problem
<exp2description>

#### Function Signature
<exp2function_signature>

#### Verus Program
<exp2verus>
"""

USER_PROMPT = """Please generate a Verus program for the provided problem.
### problem:
{description}
### function signature:
{function_signature}
Let’s think step by step!
"""
