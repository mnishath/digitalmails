# Feature Plan: Monetization & Infrastructure Economics

## Problem Statement
As an unknown startup, DigitalMails cannot rely solely on B2B enterprise sales to displace Google Workspace. We require **Network-Native Monetization**—revenue streams that emerge naturally from the protocol's unique mechanics—while maintaining a generous free tier (20GB+) without incurring unsustainable centralized cloud (AWS) costs.

## Primary Monetization: "Velocity & API" (The Community Hub)
Instead of charging for storage or basic communication, the protocol monetizes the *scale* and *automation* of communication.

### 1. The "Human Tier" (Free Forever)
*   **Target:** Individuals and small, personal Context Circles.
*   **Features:** Generous local/hot storage, E2EE sync, standard UI.
*   **Limits:** Capped outbound velocity (e.g., max 500 msgs/day, max 50 recipients/msg). Cannot be used for mass-mailing.

### 2. The "Community & Creator Tier" (SaaS Subscription)
*   **Target:** Creators, local businesses, and Web3 DAOs transitioning audiences away from Discord/Substack.
*   **Features:** 
    *   **Broadcast Rights:** Ability to message 10,000+ opted-in users simultaneously within their Context Circle.
    *   **Triage Analytics:** Dashboard for engagement metrics (opt-in read receipts).
    *   **Role Management:** Admin/Moderator keys for namespace management.

### 3. The "Automated API & Bot Tier" (High-Velocity / Pay-As-You-Go)
*   **Target:** E-commerce (receipts), SaaS (alerts), and trading bots requiring programmatic access.
*   **Features:**
    *   **Headless Access:** Programmatic API keys tied to the entity's NID for WebSocket delivery.
    *   **Verified Bot Badge:** UI distinction to ensure users know they are interacting with an automated system.
    *   **High Volume:** Replaces legacy transactional email (SendGrid) with secure, spam-free delivery.

## Experimental Infrastructure: The "DePIN Subsidy" (Storage Economics)
To support a massive free tier (e.g., 20GB+ "Infinite Log") without bankrupting the startup on cloud storage, we will experiment with Decentralized Physical Infrastructure Networks (DePIN).

### 1. The "Hot/Warm/Cold" Tiering
*   **Hot (Relays/NVMe):** Last 30 days of text/small media for instant multi-device sync (Funded by Startup/API revenue).
*   **Warm (Cloudflare R2):** Low-cost, zero-egress object storage for fast historical retrieval.
*   **Cold (Filecoin/Arweave):** The permanent "Infinite Log" of encrypted SQLite vaults and large attachments.

### 2. The DePIN Grant Model (R&D)
*   **The Strategy:** Leverage networks like Filecoin (via FIL+ multipliers) or Arweave ecosystem grants to subsidize the Cold Storage layer.
*   **The Mechanism:** Because DigitalMails provides verified, highly-valuable "real-world data" (cryptographically signed by human NIDs), Storage Providers are economically incentivized (via block rewards) to store our encrypted user vaults at near-zero cost to the startup.
*   **Status:** This remains an experimental pathway to aggressively lower COGS (Cost of Goods Sold) during the hyper-growth phase.
