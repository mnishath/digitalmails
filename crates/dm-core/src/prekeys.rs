use crate::Identity;
use ed25519_dalek::Signature;
use rand::rngs::OsRng;
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::ZeroizeOnDrop;

/// Private X25519 pre-key material held by the local device.
#[derive(ZeroizeOnDrop)]
pub struct PreKeyStore {
    spk_id: u32,
    spk_secret: StaticSecret,
    spk_pub: [u8; 32],
    spk_sig: [u8; 64],
    opks: Vec<OtpkEntry>,
}

#[derive(ZeroizeOnDrop)]
struct OtpkEntry {
    key_id: u32,
    secret: StaticSecret,
    pub_key: [u8; 32],
}

/// The public bundle published to the Relay/Registry for X3DH session initiation.
pub struct PreKeyBundle {
    pub spk_id: u32,
    pub spk_pub: [u8; 32],
    pub spk_sig: Signature,
    pub opks: Vec<OtpkPublic>,
}

pub struct OtpkPublic {
    pub key_id: u32,
    pub pub_key: [u8; 32],
}

impl PreKeyStore {
    /// Generate a signed pre-key and `n_otpks` one-time pre-keys, all signed/anchored
    /// to the given `Identity`.
    pub fn generate(identity: &Identity, n_otpks: u32) -> Self {
        let spk_secret = StaticSecret::random_from_rng(OsRng);
        let spk_pub: [u8; 32] = PublicKey::from(&spk_secret).to_bytes();
        let spk_sig: [u8; 64] = identity.sign(&spk_pub).to_bytes();

        let opks = (1..=n_otpks)
            .map(|key_id| {
                let secret = StaticSecret::random_from_rng(OsRng);
                let pub_key = PublicKey::from(&secret).to_bytes();
                OtpkEntry {
                    key_id,
                    secret,
                    pub_key,
                }
            })
            .collect();

        Self {
            spk_id: 1,
            spk_secret,
            spk_pub,
            spk_sig,
            opks,
        }
    }

    /// Return the public-only view of this store — safe to transmit to the Relay.
    pub fn bundle(&self) -> PreKeyBundle {
        PreKeyBundle {
            spk_id: self.spk_id,
            spk_pub: self.spk_pub,
            spk_sig: Signature::from_bytes(&self.spk_sig),
            opks: self
                .opks
                .iter()
                .map(|e| OtpkPublic {
                    key_id: e.key_id,
                    pub_key: e.pub_key,
                })
                .collect(),
        }
    }

    pub fn spk_secret(&self) -> &StaticSecret {
        &self.spk_secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Identity;
    use ed25519_dalek::Verifier;
    use std::collections::HashSet;

    #[test]
    fn test_signed_pre_key_signature_is_valid() {
        let identity = Identity::generate();
        let store = PreKeyStore::generate(&identity, 1);
        let bundle = store.bundle();
        identity
            .verifying_key()
            .verify(&bundle.spk_pub, &bundle.spk_sig.into())
            .expect("SPK signature must verify against identity verifying key");
    }

    #[test]
    fn test_pre_key_bundle_key_sizes_are_32_bytes() {
        let identity = Identity::generate();
        let store = PreKeyStore::generate(&identity, 1);
        let bundle = store.bundle();
        assert_eq!(bundle.spk_pub.len(), 32);
        assert_eq!(bundle.opks[0].pub_key.len(), 32);
    }

    #[test]
    fn test_one_time_pre_keys_are_unique() {
        let identity = Identity::generate();
        let store = PreKeyStore::generate(&identity, 10);
        let bundle = store.bundle();
        let unique: HashSet<[u8; 32]> = bundle.opks.iter().map(|k| k.pub_key).collect();
        assert_eq!(unique.len(), 10, "all OPK public keys must be distinct");
    }

    #[test]
    fn test_pre_key_store_generates_correct_opk_count() {
        let identity = Identity::generate();
        let store = PreKeyStore::generate(&identity, 5);
        let bundle = store.bundle();
        assert_eq!(bundle.opks.len(), 5);
    }

    #[test]
    fn test_opk_ids_are_sequential_from_one() {
        let identity = Identity::generate();
        let store = PreKeyStore::generate(&identity, 3);
        let bundle = store.bundle();
        let ids: Vec<u32> = bundle.opks.iter().map(|k| k.key_id).collect();
        assert_eq!(ids, vec![1, 2, 3]);
    }
}
