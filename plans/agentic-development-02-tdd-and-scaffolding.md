# TDD & Scaffolding Guidelines for Agents

## The "Constraint First" Philosophy
AI agents are highly capable but tend to choose the path of least resistance. If boundaries are not explicitly defined, they may generate code that is functional but unmaintainable or architecturally misaligned.

## 1. Workspace Scaffolding
Before any business logic is written, the human orchestrator must instruct the agent to build a rigid environment.
*   **Monorepos:** If using multiple languages (Rust + TypeScript), initialize a clear monorepo structure (e.g., Turborepo or Cargo Workspace) immediately.
*   **Type Safety:** For TypeScript, enforce `strict: true` in `tsconfig.json`. For Rust, treat warnings as errors (`#![deny(warnings)]`).
*   **Linting as a Guardrail:** Have the agent configure linters immediately. When the agent writes code, it must automatically run the linter and fix errors before presenting the solution to the human.

## 2. Test-Driven Development (TDD) Protocol
TDD is non-negotiable in agentic development. Tests act as the contract between the Human's intent and the Agent's execution.

### The TDD Sequence
1.  **Prompt for the Test:** Provide the agent with the specific requirement from the PRD. Instruct it to write a comprehensive test suite covering standard and edge cases.
    *   *Prompt Example:* "Based on PRD Section 3, write a failing Rust unit test that validates a Handshake Request requires a Priority Stamp."
2.  **Verify the Test:** The Human must briefly review the test to ensure it accurately captures the PRD requirement.
3.  **Prompt for Implementation:** Instruct the agent to write the implementation logic to satisfy the test.
    *   *Prompt Example:* "Now write the minimal Rust implementation to make `test_handshake_requires_stamp` pass. Do not add undocumented features."
4.  **Validate:** The agent runs the test. If it fails, the agent self-corrects until it passes.

## 3. Handling Hallucinations
If an agent hallucinates an API, invents a library, or writes complex "just-in-case" code:
*   Do not manually rewrite the code for the agent.
*   Instead, point out the specific error: *"You assumed the `tokio-tungstenite` crate was installed. It is not. Please add it to `Cargo.toml` and rewrite the connection logic."*
*   Alternatively, use the test suite to prove the hallucination fails the architectural constraints.