# System Architecture & Technical Stack

## 1. Architectural Overview
The system follows a **Tri-Layer Architecture** that decouples Identity, Routing, and Data to ensure absolute privacy and zero-spam.

```ascii
[ IDENTITY LAYER ]       [ ROUTING LAYER ]        [ DATA LAYER ]
  L2 Blockchain            Decentralized            End-to-End
   (Trust Notary)             Relays                Encrypted (E2EE)
        |                       |                       |
        +--- NID Hash           +--- Message Blobs      +--- Attachments
        +--- Public Keys        +--- Status Pings       +--- Address Book
        +--- Vault Pointers     +--- Blind Push         +--- Media
```

---

## 2. Phase 1 Technical Stack

```ascii
[ CLIENT TIER ]
+-----------------------------------------------------------------------+
|  UI LAYER: React (Web) | Swift (iOS) | Kotlin (Android)               |
|  - "Context Pivot" logic (Chat vs. Triage views)                      |
+-----------------------------------------------------------------------+
|  CORE LOGIC LAYER (Shared via Rust + WASM/UniFFI)                     |
|  - E2EE (X3DH, Double Ratchet)                                        |
|  - PSI (Private Set Intersection) matching                            |
|  - Local Hashing (Handle -> NID)                                      |
+-----------------------------------------------------------------------+
|  LOCAL PERSISTENCE: SQLite (Encrypted)                                |
|  - Local Address Book & Message Cache                                 |
+-----------------------------------------------------------------------+
        |                                       ^
        | (Secure WebSockets / JSON-RPC)        | (Blind Push / Pings)
        v                                       |
[ INFRASTRUCTURE TIER ]                 [ IDENTITY & TRUST TIER ]
+----------------------------+          +-------------------------------+
|  DECENTRALIZED RELAYS      |          |  L2 BLOCKCHAIN (The Notary)   |
|  (Language: Rust or Go)    |          |  (Base, Arbitrum, or Polygon) |
+----------------------------+          +-------------------------------+
| - Stateless Blob Storage   | <------> | - NID -> Public Key Mapping   |
| - WebSocket Gateway        |          | - NameSpace Registry          |
| - Spam Proof Verification  |          | - Reputation/Stake Contracts  |
+----------------------------+          +-------------------------------+
        |                                       |
        v                                       v
[ PERSISTENCE LAYER ]                   [ INDEXING LAYER ]
+----------------------------+          +-------------------------------+
|  Relay Storage (S3/Disk)   |          |  Indexers (The Graph/Envio)   |
|  Encrypted Blobs Only      |          |  Fast Querying of NIDs        |
+----------------------------+          +-------------------------------+
```

---

## 3. Component Justification

### A. Rust Core (Logic & Crypto)
*   **Portability:** Compiled to WASM for Web and UniFFI for Mobile (iOS/Android).
*   **Security:** Memory-safe language for critical E2EE (X3DH, Double Ratchet) and PSI logic.
*   **Consistency:** Ensures the same cryptographic primitives and hashing logic across all platforms.

### B. Decentralized Relays (Routing)
*   **Dumb Storage:** Relays only hold encrypted blobs indexed by NID. They have no visibility into content or social graphs.
*   **Real-time:** Uses WebSockets for millisecond delivery, supporting typing indicators and read receipts.
*   **Statelessness:** Allows for easy horizontal scaling and node operator participation.

### C. L2 Blockchain (Chain-Agnostic Registry)
*   **Abstracted Notary Model:** The blockchain is a pluggable storage layer for the "Global Phone Book." The protocol is natively chain-agnostic, supporting resolution across multiple L2s (Base, Polygon, etc.).
*   **Sovereign App-Chain (Optional Implementation):** As an additional consideration for the implementation phase, the protocol can be anchored to a sovereign "DigitalMails App-Chain" (See `plans/appendix_app_chain.md`). This is an evolutionary path and does not disrupt the core agnostic architecture.
*   **Registry Aggregator:** A middleware indexing layer that resolves handles across all supported chains, providing a unified lookup API for clients.
*   **Developer-Paid Gas (Paymaster):** Uses ERC-4337 Paymasters to fund on-chain registration. Users experience "Zero Gas" onboarding regardless of the underlying chain.

---

## 4. Multi-Chain Resolution Flow

```ascii
[ CLIENT APP ]
      |
      +-- (1. "Find @alice")
      V
[ REGISTRY AGGREGATOR / INDEXER ]
      |
      |-- (Search Base) ----> [ @alice -> Public Key (Base) ]
      |-- (Search Polygon) -> [ @bob   -> Public Key (Polygon) ]
      V
[ RESOLVED ENDPOINT ]
      - Public Key & Relay Address
      - Portability: NID is derived locally, not from the chain.
```

### D. Persistence & Discovery
*   **PSI Discovery:** Private Set Intersection ensures Grandma can find friends via her phone book without uploading it to any server.
*   **Blind Push:** Relays trigger OS-level pings through APNs/FCM without leaking metadata. The app fetches and decrypts the payload locally.
*   **Continuous Delta Sync:** A private E2EE queue on the Relay where devices post state updates to synchronize the inbox across platforms.
*   **Zero-Data-Loss Backup:** Encrypted SQLite vaults are backed up to the cloud. Relays retain the delta of messages until the backup is confirmed, ensuring zero interim loss if a device is destroyed.
*   **DePIN Storage (Cold):** Optional offloading of historical 20GB+ archives to Filecoin/Arweave to enable a highly scalable, free "Infinite Log".