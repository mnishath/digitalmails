# Sub-Agent Delegation Guide

## Context is your Most Precious Resource
In conversational AI interfaces, every turn adds to the context window. If a single agent performs exhaustive research, writes 1,000 lines of code, and fixes 5 bugs in one session, the context becomes bloated, leading to degraded logic and "forgetfulness."

To prevent this, the Primary Agent (Gemini CLI) must act as an **Orchestrator**, delegating tasks to specialized sub-agents. When a sub-agent completes a task, only a concise summary is returned to the main context.

## Sub-Agent Roster & Usage

### 1. `generalist` (The Heavy Lifter)
*   **Role:** The workhorse for repetitive, high-volume, or structurally complex implementations.
*   **When to use:** 
    *   Implementing a large module based on a strict spec.
    *   Refactoring a module to match a new linter rule across multiple files.
    *   Executing bash commands that output massive amounts of text (e.g., verbose builds) so the output doesn't pollute the main session.
*   **Delegation Strategy:** Give the `generalist` a highly detailed prompt containing the specific PRD section, the exact files to touch, and the "Definition of Done."

### 2. `codebase_investigator` (The Scout)
*   **Role:** The specialized tool for mapping architecture and analyzing dependencies without modifying files.
*   **When to use:**
    *   Before starting a new Spike, to understand how it impacts existing systems.
    *   To debug a complex, multi-file error.
    *   To audit the codebase for architectural drift against the PRD.
*   **Delegation Strategy:** Ask broad, analytical questions. "Trace the data flow of a received message from the WebSocket listener to the SQLite storage."

### 3. Primary Agent (The Orchestrator)
*   **Role:** High-level strategy, single-file surgical edits, answering human queries, and delegating to sub-agents.
*   **When to use:**
    *   Reviewing the PRD.
    *   Writing the initial tests (TDD step 1).
    *   Making targeted fixes to configuration files.

## The Concurrency Rule
*   **Never run multiple sub-agents in parallel if they mutate the same files.** This causes race conditions and corrupted file states.
*   You *may* run multiple `codebase_investigator` tasks in parallel to speed up research.