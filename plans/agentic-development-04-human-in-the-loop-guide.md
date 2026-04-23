# Human in the Loop (HITL): The Architect's Guide

## Your Role: The Orchestrator and Gatekeeper
When building software with AI agents, you are no longer the primary coder. You are the **Chief Architect**, the **Product Manager**, and the **QA Gatekeeper**. The agents are your highly capable, but literal-minded, engineering team.

Your primary job is to **steer intent** and **enforce simplicity**.

## 1. Defining the Boundaries
Agents will solve the problem presented to them, often introducing unnecessary complexity if left unchecked.
*   **Enforce "Invisible Math":** If an agent proposes a complex, multi-step user flow to solve a cryptographic problem, reject it. Remind the agent of the core mandate: *The complexity must be hidden from the user.*
*   **Scope the Spikes:** Provide narrow, unambiguous instructions. Do not say "build the inbox UI." Say: "Build a React component that fetches the SQLite 'Inbound Queue' and displays it as a flat list. Do not implement the Chat UI yet."

## 2. The Art of Reviewing Agent PRs
When the agent completes an implementation step, do not just accept "It works."
*   **Check the Architecture:** Did the agent use explicit composition, or did it hack together a quick, fragile inheritance chain? 
*   **Check the Tests:** Did the agent write meaningful tests, or just tautological ones (e.g., `assert(true == true)`) to bypass the TDD rule?
*   **Rejecting Code:** It is cheaper to tell the agent to `git restore .` and try again with better instructions than it is to manually fix bad agentic code. Be ruthless. Say: *"The logic works, but it violates our stateless relay rule. Revert and rewrite this without storing metadata."*

## 3. Managing the Context
You must act as the memory manager for the AI.
*   **Course Corrections:** Use "User hints:" to redirect the agent immediately if you see it going down a rabbit hole in the terminal output.
*   **End of Session Summaries:** When a major milestone is complete, instruct the agent to write a summary of the implementation details to a local `.md` documentation file, and then instruct it to clear its temporary memory/context for the next sprint.

## 4. The Workflow Checklist
For every major feature implementation, ensure you follow this checklist:
- [ ] Is the requirement clearly mapped in the PRD?
- [ ] Have I broken this down into an atomic "Spike"?
- [ ] Did the agent write a failing test first?
- [ ] Does the implementation pass the test?
- [ ] Did I review the code for architectural drift and simplicity?
- [ ] Is the feature documented and the context ready for the next Spike?