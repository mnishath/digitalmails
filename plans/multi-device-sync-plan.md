# Feature Plan: Multi-Device Sync & Zero-Data-Loss Recovery

## Problem Statement
The decentralized Relay architecture is designed to be stateless, holding E2EE blobs only until fetched. However, if a user loses their device between periodic backups, they lose all messages sent or received in the interim. Furthermore, users need the ability to seamlessly use multiple devices (Desktop + Phone) with a synchronized inbox state.

## Proposed Architecture: The "Continuous Delta" Model

We solve this by creating an interplay between a "Master Snapshot" (Cloud Backup) and an "Interim Delta" (Relay Sync Stream).

### 1. Secure Remote Backup (The Master Snapshot)
*   **The Vault:** The user's historical messages, contacts, and preferences are stored in an encrypted local SQLite database.
*   **Encrypted Cloud Storage:** Devices periodically (e.g., daily) push an encrypted copy of this vault to a secure remote location (e.g., iCloud, Google Drive, IPFS).
*   **Zero Knowledge:** The vault is encrypted with the user's Root Key. The cloud provider cannot read the contents.

### 2. The Encrypted Sync Stream (The Interim Delta)
To cover the time gap between backups and to sync multiple devices live, we use the Relay as a temporary, append-only log.
*   **The Log:** Every user has a private, E2EE "Sync Stream" on their Relay.
*   **Event Posting:** When a device sends a message, reads an email, or receives a payload, it encrypts the event/message and posts it to the Sync Stream.
*   **Live Synchronization:** All active devices fetch the stream to keep their local state perfectly synchronized.

### 3. The Pruning Trigger (Preventing Interim Loss)
This is the critical mechanism for zero-data-loss:
*   Devices **do not** tell the Relay to delete a message just because it was read.
*   The Relay holds the continuous Sync Stream and Inbound Queue as a "Delta."
*   **The Prune Command:** Only *after* a device successfully uploads a new Secure Remote Backup to the cloud does it issue a "Prune to Timestamp X" command to the Relay. 
*   The Relay then safely deletes the blobs, knowing they are permanently secured in the user's cloud snapshot.

### 4. New Device / Disaster Recovery Flow
If a device is lost or destroyed mid-day, the user loses nothing:
1.  **Identity Recovery:** The user enters their `handle` and 4-word `Secret Sentence` on the new device, reconstructing their Root Key.
2.  **Snapshot Restoration:** The app downloads and decrypts the latest Secure Remote Backup from the cloud, restoring the inbox up to the point of the last backup.
3.  **Delta Application:** The app connects to the Relay. Because no device issued a "Prune Command" for the interim period, the Relay holds the complete Encrypted Sync Stream of all messages sent/received since the backup. The app downloads the stream and perfectly rebuilds the inbox to the exact second the old device was lost.

## Impact on System Architecture
*   **Relays:** Remain privacy-preserving and effectively stateless, acting only as short-term buffers (deltas) rather than permanent databases.
*   **Clients:** Handle the heavy lifting of state management, delta application, and cloud syncing.
*   **User Experience:** Provides legacy-email levels of reliability (zero data loss) while maintaining strict Web3 privacy architectures.