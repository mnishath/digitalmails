# Appendix: DigitalMails Sovereign App-Chain Strategy

## 1. Vision: Sovereign Infrastructure
While Phase 1 utilizes hosted L2s (like Base), Phase 2 envisions a sovereign **DigitalMails App-Chain**. This provides absolute control over the Handle Registry, removes third-party fee volatility, and enables native gasless onboarding.

## 2. Technical Implementation (The "Rollup" Stack)
The App-Chain will be implemented as a **Layer 2 Rollup** using modern modular frameworks:

*   **Framework:** OP Stack or Arbitrum Orbit (EVM-compatible for seamless migration of Registry contracts).
*   **Sequencer:** Managed via a **Rollup-as-a-Service (RaaS)** provider (e.g., Conduit, Caldera) for 99.9% uptime and managed devops.
*   **Data Availability (DA):** Uses an Alt-DA layer (e.g., **Celestia** or EigenDA) to minimize data-posting costs to <$100/month.
*   **Gas Logic:** Natively configured for **Zero Gas Fees**. The sequencer is programmed to accept transactions from the DigitalMails client without requiring tokens.

## 3. Economic Model (Operational Costs)
Operating a sovereign registry transitions the protocol from "Variable Gas Costs" to "Fixed Infrastructure Costs":

| Component | Provider Type | Estimated Monthly Cost |
| :--- | :--- | :--- |
| **Sequencer & RPC** | RaaS (Managed) | ~$3,000 |
| **Alt-DA Posting** | Celestia/EigenDA | ~$100 |
| **Block Explorer** | Blockscout (Hosted) | Included in RaaS |
| **Indexer API** | Envio / The Graph | ~$200 |
| **TOTAL** | | **~$3,300/mo** |

## 4. Migration Path: "The Registry Pivot"
Thanks to the **Registry Aggregator** architecture, the migration is invisible to users:
1.  **Snapshot:** A Merkle root of the Phase 1 (Base/Polygon) registry is taken.
2.  **Anchor:** This root is initialized as the "Genesis State" of the App-Chain.
3.  **Pivot:** The Aggregator service is updated to resolve new handle claims on the App-Chain while continuing to verify legacy handles via the anchored snapshot.

## 5. Decision Triggers
The move to a sovereign App-Chain should be triggered when:
*   **Volume:** Registration volume exceeds 50,000 users/month (where RaaS costs < Variable Gas costs).
*   **Sovereignty:** A major L2 makes a breaking change to their gas model or censorship policies.
*   **Enterprise Demand:** A large organization requires a private, high-performance sub-registry (L3) linked to our root.
