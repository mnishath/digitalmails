# Session Handoff Document

## Current Status
We have completed the core architectural brainstorming for a modernized, Web3-based realtime email replacement. We have successfully defined the network topology, trust model, real-time notification mechanics, account recovery, and UI philosophy. We have additionally defined multi-device sync, storage economics, and monetization.

All finalized requirements have been consolidated into `prd.md` and `architecture.md`.

## Key Decisions Made
1.  **Architecture:** Chain-Agnostic Registry Aggregator + Developer-Paid Paymaster + Decentralized Relays.
2.  **Trust Model:** "Half-Handshake" for strangers; "Local Filter" enforcement (no complex on-chain slashing); Universal NIDs (Local Key Management).
3.  **Onboarding:** Gasless experience; Developers cover on-chain costs; Passkey/Secure Enclave as root identity.
4.  **Discovery:** Private Set Intersection (PSI) for local contact matching + Multi-Chain Handle Lookup.
5.  **Multi-Device / Backup:** "Continuous Delta" sync stream + Cloud Vault Backup ensuring zero data loss.
6.  **Monetization:** Velocity & Bot API Subscriptions + Handle Registration Fees + Stamp Fees.
7.  **Storage Economics:** Hot (Relay) / Warm (R2) / Cold (DePIN Filecoin/Arweave) tiering to support massive 20GB+ free tiers.
8.  **Sovereignty Path (Optional):** Roadmap defined for a **DigitalMails App-Chain** for long-term native zero-gas.

## Items Parked for Next Session
1.  **Aggregator API Specification:** Defining the REST/JSON-RPC interface for the cross-chain handle resolver.
2.  **UI/UX Wireframes:** Designing the "Context Pivot" and "Stamp" UX.

## Next Steps
1.  Review final consolidated plans.
2.  Exit Plan Mode and approve the transition to the Implementation Phase.