# Best Practices: Building DigitalMails with AI Agents

Transitioning from a comprehensive Product Requirements Document (PRD) to a fully functional application using AI agents requires a shift from "Architectural Brainstorming" to "Orchestrated Execution."

To ensure the "Invisible Math" elegance of DigitalMails is maintained in the codebase, follow this strict agentic development workflow.

---

## Phase 1: Atomic Decomposition (The "Spike" Strategy)
Agents excel at narrowly defined, highly specific tasks but can hallucinate if given monolithic directives (e.g., "Build the auth system").

1.  **Define the Core Primitive:** Start with the absolute foundation. For DigitalMails, this is **Local Key Generation & NID Hashing**.
2.  **Create Micro-Plans:** Break the PRD into isolated, verifiable "Spikes."
    *   *Bad Prompt:* "Implement the Identity Layer."
    *   *Good Prompt:* "Implement an Ed25519 key pair generator in Rust, hash the public key using SHA-256 to create the NID, and store it in an encrypted SQLite database."
3.  **Strict Boundaries:** Ensure every delegated task has a clear "Definition of Done."

## Phase 2: Scaffolding & CI/CD First
Before any agent writes business logic, establish the infrastructure to keep their code disciplined.

1.  **Initialize the Workspace:** Setup the monorepo (e.g., a Cargo workspace for Rust core, a Turborepo for the React web app and Relay backend) to provide strict boundaries.
2.  **Strict Linters & Formatters:** AI agents generate code rapidly. Enforce strict linting (e.g., Clippy for Rust, ESLint for TypeScript) to force idiomatic code generation from Day 1.
3.  **Test-Driven Development (TDD):** This is the **most critical best practice** for agentic coding.
    *   *Step A:* Command the agent to write a failing test based on the PRD requirement (e.g., `test_nid_generation_from_handle`).
    *   *Step B:* Command the agent to implement the exact code required to make that specific test pass. The test acts as an objective anchor, preventing scope creep or hallucination.

## Phase 3: Strategic Agent Delegation
Operating as the "Orchestrator," utilize specialized sub-agents to manage complexity and context limits.

1.  **The `codebase_investigator`:** Use this agent when starting a new module to research existing patterns, map architectural dependencies, or plan how a new component (like the Relay Sync Stream) integrates with the existing E2EE core.
2.  **The `generalist`:** Delegate large, repetitive, or structurally complex implementations to this agent. This keeps the main conversational context window lean and focused. (e.g., *"Invoke generalist to implement the WebSocket connection manager for the Relay based on the PRD."*)
3.  **The Main Agent (Gemini CLI):** Utilize the primary agent for high-level architectural decisions, single-file surgical edits, real-time problem-solving, and continuous alignment with the PRD.

## Phase 4: The DigitalMails Build Sequence
Based on the finalized PRD, execute the implementation in the following ordered milestones:

*   **Milestone 1: The Cryptographic Core (Rust):** Implement the E2EE primitives (X3DH, Double Ratchet) and the Private Set Intersection (PSI) hashing logic. This foundational math must be rock-solid and heavily tested before any network code is written.
*   **Milestone 2: The "Dumb" Relay (Rust/Go):** Build the stateless WebSocket server capable of accepting, holding, and routing E2EE blobs based purely on NIDs, incorporating the "Continuous Delta" sync logic.
*   **Milestone 3: The Aggregator API:** Build the middleware service that resolves handles across the L2 testnet (e.g., Base Sepolia), returning the correct Public Key and Relay Address.
*   **Milestone 4: The Client UI (React/Mobile):** Finally, build the "Context Pivot" interface (Chat vs. Triage), wrapping the Rust cryptographic core via WASM/UniFFI.

## The Golden Rule: The Human as the Architect
Never allow an agent to design code structure or architectural patterns on the fly. You must dictate the patterns. If the PRD mandates "explicit composition over inheritance" or "dumb stateless routing," enforce it strictly during code generation. Review the agent's proposed changes *before* they are committed, forcing refactors if the code becomes overly complex or deviates from the elegant simplicity required by the system.