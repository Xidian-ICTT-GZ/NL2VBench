SYSTEM_PROMPT_SPEC_ONLY = """
You are an expert in Rust and the Verus verification framework.
You will be provided with a programming problem described in natural language,
including its function signature.
Your task is to generate Verus specifications for the given problem.
The specifications must be written using Verus constructs
and must be compatible with the Verus verifier.
Ensure the specifications include appropriate requires, ensures,
necessary invariant, decreases clauses, and supporting assertions.
Do not generate an executable implementation.

Below are some examples illustrating the expected input-output behavior.

### Example 1

#### Problem
<exp1description>

#### Function Signature
<exp1function_signature>

#### Verus Specifications
<exp1spec>

### Example 2

#### Problem
<exp2description>

#### Function Signature
<exp2function_signature>

#### Verus Specifications
<exp2spec>
"""

USER_PROMPT_SPEC_ONLY = """
Please generate Verus specifications for the provided problem.

Before writing the final specifications, consider the following:
1. What are the weakest preconditions required for the function to be safe and well-defined
   (e.g., input bounds, arithmetic safety)?
2. What are the strongest postconditions that precisely capture the intended behavior?
3. What additional specifications are necessary to prove the postconditions,
   such as loop invariants, decreases clauses, assertions, or helper lemmas?

After reasoning about these aspects, provide the final Verus specifications.

### problem:
{description}

### function signature:
{function_signature}
"""

SYSTEM_PROMPT_CODE_FROM_SPEC = """
You are an expert in Rust and the Verus verification framework.
You will be provided with Verus specifications and a function signature.
Your task is to generate a Rust implementation that satisfies
the given Verus specifications.

The generated code must strictly follow the provided function signature
and must not modify the given specifications.
The resulting program must be compatible with the Verus verifier.

Below are some examples illustrating the expected input-output behavior.

### Example 1

#### Function Signature
<exp1function_signature>

#### Verus Specifications
<exp1spec>

#### Verus Program
<exp1verus>

### Example 2

#### Function Signature
<exp2function_signature>

#### Verus Specifications
<exp2spec>

#### Verus Program
<exp2verus>
"""

USER_PROMPT_CODE_FROM_SPEC = """
Please generate a Rust implementation that satisfies
the provided Verus specifications.

Before writing the final code, consider the following:
1. How should the control flow and data structures be organized to satisfy the given specifications?
2. Where are assertions or helper proof constructs required to make the verifier succeed?
3. How can the implementation ensure termination and compliance with decreases clauses?

After reasoning about these aspects, provide the final Verus program.

### function signature:
{function_signature}

### Verus specifications:
{spec}
"""