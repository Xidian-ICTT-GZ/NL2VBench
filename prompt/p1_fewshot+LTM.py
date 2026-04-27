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


USER_PROMPT = """
Please generate a Verus program for the provided problem.

Before writing the final Verus program, consider the following:

1. What are the necessary preconditions (`requires`) for the function,
   including any constraints on inputs and arithmetic bounds?
2. What are the desired postconditions (`ensures`) that the function
   must satisfy?
3. What specifications are required to prove the postconditions?
   This may include loop invariant, decreases clauses, assertions,
   and any necessary proof obligations.

After reasoning about these aspects, generate the final Verus program
that satisfies the specifications and is accepted by the Verus verifier.

### problem:
{description}

### function signature:
{function_signature}
"""