# DigitalMails — Claude Code Project Brief

## What This Is

DigitalMails is a privacy-first, Web3-powered asynchronous and synchronous communication protocol designed to replace legacy SMTP. It guarantees zero spam and absolute user privacy by combining decentralized stateless Relays (routing), an L2 blockchain as a chain-agnostic PKI (identity), and a Rust cryptographic core shared across all clients via WASM and UniFFI.

Full requirements: `plans/prd.md`  
System architecture: `plans/architecture.md`  
Agentic dev workflow: `plans/llm-agent-best-practices.md`

---

## Repository Layout

```
/
├── Cargo.toml              ← Cargo workspace root (no [package])
├── crates/
│   └── dm-core/            ← Milestone 1: cryptographic primitives (Rust)
│       └── src/
│           ├── lib.rs      ← public API; #![deny(warnings)]
│           └── identity.rs ← Identity struct: Ed25519 keypair + SHA-256 NID
└── plans/                  ← architecture & process docs (read-only reference)
```

Future crates drop into `crates/` and get added to the workspace `members` list. Future apps (Relay server, React web, mobile) go under `apps/`.

---

## Build Sequence (Milestones)

| Milestone | Scope | Status |
|---|---|---|
| **1 — Cryptographic Core** | Ed25519 keygen, NID hashing (SHA-256), X3DH, Double Ratchet, PSI | Spike 1 ✅ |
| **2 — Dumb Relay** | Stateless WebSocket server; blob routing by NID; Continuous Delta sync | Not started |
| **3 — Aggregator API** | Cross-chain handle resolver (Base Sepolia, etc.) | Not started |
| **4 — Client UI** | React (WASM) + mobile (UniFFI); Context Pivot: Chat ↔ Triage | Not started |

Do not start a later milestone until the prior one is fully tested and committed.

---

## Core Concepts

**NID (Network Identifier):** `NID = SHA-256(ed25519_public_key_bytes)` — a 32-byte value, displayed as 64-char lowercase hex. This is the user's canonical address on the network. It is derived locally and never requires a server.

**Identity Layer:** L2 blockchain (Base, Polygon, etc.) is used *only* as a PKI — mapping `handle → NID → public key`. It is never a message store.

**Relay:** Stateless blob storage indexed by NID. Holds E2EE blobs until fetched. Has zero visibility into content or social graph.

**E2EE Protocol:** X3DH for initial key agreement, Double Ratchet for session keys (future spikes).

**PSI:** Private Set Intersection — contact discovery without uploading the address book (future spike).

---

## Invariants — Enforce in Every Session

- `#![deny(warnings)]` in every Rust crate. No exceptions.
- **TDD**: write failing test → implement minimal code to pass → refactor. Never skip the failing-test step.
- **One Spike per session**. Commit after every Green phase before starting the next spike.
- No `unsafe` blocks without explicit human architect approval.
- Explicit composition — no inheritance or trait-object chains where a plain struct suffices.
- No `unwrap()` in library code (only in tests). Use `?` or explicit error types.
- Relays must remain stateless. No social-graph metadata stored server-side.

---

## Commands

```bash
# Test a specific crate
cargo test -p dm-core

# Lint (warnings = errors)
cargo clippy -p dm-core -- -D warnings

# Format check (run before committing)
cargo fmt --check

# Auto-format
cargo fmt

# Test everything
cargo test --workspace
```

---

## Current Spike State (Milestone 1, Spike 1)

**Crate:** `crates/dm-core`  
**Done:** `Identity::generate()` — produces an Ed25519 signing key and derives its NID via SHA-256. Five tests cover shape, format, cryptographic correctness, uniqueness, and round-trip hex decoding.

**Next spike candidates:**
- Spike 2: Serialize/deserialize `Identity` to/from an encrypted local store (SQLite + SQLCipher)
- Spike 3: X3DH pre-key bundle generation and key agreement
- Spike 4: Double Ratchet session initialization
