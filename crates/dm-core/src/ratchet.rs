use crate::x3dh::SharedSecret;
use hkdf::Hkdf;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use zeroize::ZeroizeOnDrop;

type HmacSha256 = Hmac<Sha256>;

/// Per-message symmetric key produced by a single ratchet step.
#[derive(ZeroizeOnDrop)]
pub struct MessageKey {
    pub key: [u8; 32],
    pub index: u32,
}

/// Symmetric KDF chain seeded from the X3DH shared secret.
///
/// Both parties initialize from the same `SharedSecret`; stepping in sync with
/// `next_message_key` produces identical keys on each side, giving forward
/// secrecy — a compromised key reveals nothing about prior or future keys.
#[derive(ZeroizeOnDrop)]
pub struct RatchetSession {
    chain_key: [u8; 32],
    count: u32,
}

impl RatchetSession {
    /// Derive the initial chain key from the X3DH shared secret via HKDF.
    pub fn from_shared_secret(secret: &SharedSecret) -> Self {
        let hk = Hkdf::<Sha256>::new(None, secret.as_bytes());
        let mut chain_key = [0u8; 32];
        hk.expand(b"DigitalMails DR chain v1", &mut chain_key)
            .expect("32 bytes is a valid HKDF output length");
        Self {
            chain_key,
            count: 0,
        }
    }

    /// Advance the chain by one step and return the message key for this slot.
    ///
    /// `msg_key  = HMAC-SHA256(chain_key, 0x01)`
    /// `chain_key = HMAC-SHA256(chain_key, 0x02)`
    pub fn next_message_key(&mut self) -> MessageKey {
        let msg_key = hmac_sha256(&self.chain_key, &[0x01]);
        self.chain_key = hmac_sha256(&self.chain_key, &[0x02]);
        let index = self.count;
        self.count += 1;
        MessageKey {
            key: msg_key,
            index,
        }
    }

    pub fn count(&self) -> u32 {
        self.count
    }
}

fn hmac_sha256(key: &[u8; 32], data: &[u8]) -> [u8; 32] {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC accepts any key length");
    mac.update(data);
    mac.finalize().into_bytes().into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::x3dh::x3dh_initiate;
    use crate::{Identity, PreKeyStore};

    fn make_shared_secret() -> SharedSecret {
        let alice = Identity::generate();
        let bob = Identity::generate();
        let bob_store = PreKeyStore::generate(&bob, 1);
        let bob_bundle = bob_store.bundle(&bob);
        let (_, secret) = x3dh_initiate(&alice, &bob_bundle);
        secret
    }

    #[test]
    fn test_sender_and_receiver_derive_same_message_keys() {
        let secret = make_shared_secret();
        let mut sender = RatchetSession::from_shared_secret(&secret);
        let mut receiver = RatchetSession::from_shared_secret(&secret);

        for _ in 0..3 {
            assert_eq!(
                sender.next_message_key().key,
                receiver.next_message_key().key
            );
        }
    }

    #[test]
    fn test_message_keys_advance_each_step() {
        let secret = make_shared_secret();
        let mut session = RatchetSession::from_shared_secret(&secret);
        let k1 = session.next_message_key().key;
        let k2 = session.next_message_key().key;
        assert_ne!(k1, k2);
    }

    #[test]
    fn test_message_key_is_32_bytes() {
        let secret = make_shared_secret();
        let mut session = RatchetSession::from_shared_secret(&secret);
        assert_eq!(session.next_message_key().key.len(), 32);
    }

    #[test]
    fn test_message_key_indices_are_sequential() {
        let secret = make_shared_secret();
        let mut session = RatchetSession::from_shared_secret(&secret);
        for expected in 0u32..5 {
            assert_eq!(session.next_message_key().index, expected);
        }
    }

    #[test]
    fn test_chain_produces_all_unique_keys() {
        let secret = make_shared_secret();
        let mut session = RatchetSession::from_shared_secret(&secret);
        let keys: std::collections::HashSet<[u8; 32]> =
            (0..20).map(|_| session.next_message_key().key).collect();
        assert_eq!(
            keys.len(),
            20,
            "every message key in the chain must be unique"
        );
    }
}
