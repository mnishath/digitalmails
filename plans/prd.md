# Product Requirements Document (PRD)
## Modernized Realtime Email Service & App

### 1. Vision & Overview
A privacy-first, Web3-powered asynchronous and synchronous communication protocol designed to replace legacy SMTP. The system guarantees zero spam, absolute user privacy, and real-time delivery without relying on centralized data-harvesting servers.

### 2. Core Architecture
*   **Network Topology (Encrypted Relays):** Replaces centralized servers with decentralized, stateless Relays that temporarily hold E2EE data blobs until fetch.
*   **Identity & Blockchain (Chain-Agnostic PKI):** Uses L2 blockchains strictly as a Public Key Infrastructure (PKI). The system is designed to be chain-agnostic, allowing handles to be registered and resolved across any supported L2 (Base, Polygon, etc.) via a Registry Aggregator.
*   **Sovereign Evolution:** Long-term roadmap includes a dedicated **DigitalMails App-Chain** to ensure native zero-gas and absolute infrastructure sovereignty (See `plans/appendix_app_chain.md`).
*   **Gasless Onboarding:** All on-chain costs for handle registration and namespace anchoring are covered by the developer via a Paymaster. Users never manage gas or tokens.

### 3. Trust & Anti-Spam (Web of Trust)
*   **Invite-Only & Reputation:** Users join via a cryptographic invite tree. Inviting a user requires staking reputation/tokens. If an invitee acts maliciously (spamming), the inviter's stake is slashed.
*   **Closed Loop Mode (Child Safety/Compliance):** Natively supports COPPA/GDPR compliance via Cryptographic Age Proofs. Minors' accounts are cryptographically tied to a Guardian's NID and physically cannot receive messages from outside their approved local address book.

### 4. Real-Time Delivery & First Contact
*   **The "Half-Handshake" Protocol:** To prevent spam, initial contact with a stranger requires a "Handshake Request." 
    *   The sender's app sends a small packet (Identity + Reason + Priority Stamp).
    *   The full encrypted message is only transmitted *after* the recipient accepts.
*   **Registry Aggregator:** A global indexing layer that resolves handles regardless of their anchor chain.
*   **Real-Time Experience:** WebSockets for millisecond delivery and Privacy-Preserving "Blind Push" for mobile.

### 5. UI/UX Paradigm
*   **Context Pivot Interface:** A single unified UI that serves both casual and power users.
    *   *Chat Mode:* Default view resembling modern messengers organized by people and recent activity.
    *   *Triage Mode:* A quick toggle shifts the UI to a high-density, keyboard-driven inbox for bulk management.
*   **Privacy-Preserving Discovery (PSI):** Uses Private Set Intersection to match local contacts with NIDs on the L2. The user's address book never leaves their device.
*   **Context Circles (Private Namespaces):** Users can create groups (e.g., `@family`, `@poker-club`) where all members are auto-acknowledged.

### 6. Account Recovery & Data State
*   **On-Chain Vault:** The user's root cryptographic key is heavily encrypted using a simple 4-word "Secret Sentence" (Recovery Passphrase) and stored on the smart contract.
*   **State Anchor:** The human-readable Address Book (mapping hashes to names) is encrypted locally, stored on decentralized storage (IPFS/Relays), and its URI pointer is anchored to the user's on-chain NID.
*   **Continuous Delta Sync:** Devices share a private, E2EE sync stream on the Relay to synchronize live state.
*   **Zero-Data-Loss Backup:** Users can securely back up their encrypted SQLite message vault to the cloud. Devices only command the Relay to prune the sync stream *after* a successful cloud backup, ensuring the Relay holds the perfect "Delta" for disaster recovery.
*   **Recovery Flow:** A user entering their Handle and 4-word Passphrase on a new device decrypts their root key, fetches the last Cloud Backup, and applies the Relay's "Delta" stream to achieve 100% data recovery without interim loss.

### 7. Commercialization & Infrastructure Economics
*   **Velocity & API (Primary):** Free basic human-to-human tier. DAOs, Creators, and Bots pay subscription fees ($49-$199/mo) for broadcast rights and high-volume API access.
*   **Namespace & Handle Registration Fees:** One-time fees for claiming premium handles (e.g., `@alice`) or organizational namespaces (e.g., `@acme.corp`).
*   **Validated Namespace Subscriptions:** Organizations pay recurring fees for administrative controls, employee lifecycle management (revocation), and "Auto-Trust" within their domain.
*   **The Stamp Economy:** A fraction of every "Priority Stamp" serves as a protocol fee or is burned to maintain token value.
*   **DePIN Subsidy (Experimental):** Exploring Filecoin/Arweave to provide a massive 20GB+ free tier by leveraging Web3 storage ecosystem grants and block reward multipliers.

### 8. GTM & User Acquisition (Discovery Phase)
*   **The "Influencer Shield" Strategy:** Target high-profile individuals (CEOs, Founders, Creators) who are overwhelmed by legacy email spam. Use the "Priority Stamp" as a hook to provide them with a "Clean Inbox" experience.
*   **Enterprise Seedling:** Offer free Validated Namespaces to small startups/DAO communities to establish the network effect of "Verified Work Identity."
*   **The Viral Trust Loop:** Leverage the "Sponsor/Invite" model to turn every user into a quality-filter for the network, incentivizing growth through high-reputation circles.
*   **Privacy-Native Marketing:** Focus on the "Zero Metadata" and "Zero Spam" USP (Unique Selling Proposition) for privacy-conscious communities.