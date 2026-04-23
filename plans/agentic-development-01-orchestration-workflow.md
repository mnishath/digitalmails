# Orchestrated Execution Workflow

## Overview
Building complex software with AI agents requires a deterministic, iterative loop. Agents cannot hold entire monolithic systems in their working memory reliably. The Orchestrated Execution Workflow breaks down the PRD into isolated, manageable units of work.

## The Execution Loop
Follow this sequence for every feature derived from the PRD:

### 1. Define the Epic
Extract a major capability from the PRD (e.g., "The Cryptographic Core").

### 2. Isolate into "Spikes"
Break the Epic down into atomic "Spikes" (Proof of Concept tasks). A Spike must have a singular, unambiguous goal.
*   *Example:* "Generate an Ed25519 keypair and derive a SHA-256 NID."

### 3. Scaffold the Environment
Before writing business logic, ensure the scaffolding is present:
*   Initialize the module/package.
*   Configure strict linters (e.g., Clippy for Rust).
*   Setup the test harness.

### 4. Test-Driven Implementation (TDD)
1.  **Red:** Command the agent to write a failing test that asserts the exact requirements of the Spike.
2.  **Green:** Command the agent to write the minimal code required to make the test pass.
3.  **Refactor:** Command the agent to clean up the code, enforcing architectural rules (e.g., "Apply explicit composition").

### 5. Context Pruning
Once a Spike is complete, merged, and verified, clear the conversational context if possible, or summarize the output into a markdown file. This prevents the agent from dragging legacy thoughts into the next Spike.

## Rules of Engagement
*   **Never skip the test phase.** Tests are the objective anchor that prevents AI hallucination.
*   **One Spike per session/context.** Do not ask the agent to implement the UI and the Backend database in the same conversational sweep.
*   **Commit frequently.** After every successful Green/Refactor phase, ensure the code is committed so you can easily revert if a subsequent agentic step breaks the system.