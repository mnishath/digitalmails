use crate::{Identity, PreKeyBundle, PreKeyStore};
use hkdf::Hkdf;
use rand::rngs::OsRng;
use sha2::Sha256;
use x25519_dalek::{PublicKey, StaticSecret};
use zeroize::ZeroizeOnDrop;

const INFO: &[u8] = b"DigitalMails X3DH v1";

/// The 32-byte shared secret produced by X3DH — seeds the Double Ratchet root key.
#[derive(ZeroizeOnDrop)]
pub struct SharedSecret([u8; 32]);

impl SharedSecret {
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

/// What Alice transmits to Bob alongside the first encrypted message so he can
/// reproduce the same DH operations on his side.
pub struct InitiationPacket {
    pub alice_ik_dh_pub: [u8; 32],
    pub alice_ek_pub: [u8; 32],
    pub spk_id: u32,
    pub opk_id: Option<u32>,
}

/// Alice (initiator): produce an `InitiationPacket` and the shared secret.
///
/// Uses the first OPK in the bundle if one is present; omits DH4 otherwise.
pub fn x3dh_initiate(
    alice: &Identity,
    bob_bundle: &PreKeyBundle,
) -> (InitiationPacket, SharedSecret) {
    let alice_ek = StaticSecret::random_from_rng(OsRng);
    let alice_ek_pub = PublicKey::from(&alice_ek).to_bytes();

    let opk = bob_bundle.opks.first();

    // DH1 = DH(IK_A_dh, SPK_B)
    let dh1 = alice.dh_diffie_hellman(&PublicKey::from(bob_bundle.spk_pub));
    // DH2 = DH(EK_A, IK_B_dh)
    let dh2 = alice_ek.diffie_hellman(&PublicKey::from(bob_bundle.ik_dh_pub));
    // DH3 = DH(EK_A, SPK_B)
    let dh3 = alice_ek.diffie_hellman(&PublicKey::from(bob_bundle.spk_pub));

    let mut ikm = Vec::with_capacity(128);
    ikm.extend_from_slice(dh1.as_bytes());
    ikm.extend_from_slice(dh2.as_bytes());
    ikm.extend_from_slice(dh3.as_bytes());

    let opk_id = opk.map(|o| {
        // DH4 = DH(EK_A, OPK_B)
        let dh4 = alice_ek.diffie_hellman(&PublicKey::from(o.pub_key));
        ikm.extend_from_slice(dh4.as_bytes());
        o.key_id
    });

    let packet = InitiationPacket {
        alice_ik_dh_pub: alice.dh_public_key(),
        alice_ek_pub,
        spk_id: bob_bundle.spk_id,
        opk_id,
    };

    (packet, kdf(&ikm))
}

/// Bob (receiver): reproduce the shared secret from Alice's `InitiationPacket`.
pub fn x3dh_receive(
    bob: &Identity,
    bob_store: &PreKeyStore,
    packet: &InitiationPacket,
) -> SharedSecret {
    let alice_ik = PublicKey::from(packet.alice_ik_dh_pub);
    let alice_ek = PublicKey::from(packet.alice_ek_pub);

    // DH1 = DH(SPK_B, IK_A_dh) — commutative mirror of Alice's DH1
    let dh1 = bob_store.spk_diffie_hellman(&alice_ik);
    // DH2 = DH(IK_B_dh, EK_A) — commutative mirror of Alice's DH2
    let dh2 = bob.dh_diffie_hellman(&alice_ek);
    // DH3 = DH(SPK_B, EK_A) — commutative mirror of Alice's DH3
    let dh3 = bob_store.spk_diffie_hellman(&alice_ek);

    let mut ikm = Vec::with_capacity(128);
    ikm.extend_from_slice(dh1.as_bytes());
    ikm.extend_from_slice(dh2.as_bytes());
    ikm.extend_from_slice(dh3.as_bytes());

    if let Some(opk_id) = packet.opk_id {
        // DH4 = DH(OPK_B, EK_A) — commutative mirror of Alice's DH4
        let dh4 = bob_store
            .opk_diffie_hellman(opk_id, &alice_ek)
            .expect("OPK must exist for the id Alice specified");
        ikm.extend_from_slice(dh4.as_bytes());
    }

    kdf(&ikm)
}

fn kdf(ikm: &[u8]) -> SharedSecret {
    let hk = Hkdf::<Sha256>::new(None, ikm);
    let mut okm = [0u8; 32];
    hk.expand(INFO, &mut okm)
        .expect("32 bytes is a valid HKDF output length");
    SharedSecret(okm)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Identity, PreKeyStore};

    #[test]
    fn test_x3dh_both_parties_derive_same_secret() {
        let alice = Identity::generate();
        let bob = Identity::generate();
        let bob_store = PreKeyStore::generate(&bob, 1);
        let bob_bundle = bob_store.bundle(&bob);

        let (packet, alice_secret) = x3dh_initiate(&alice, &bob_bundle);
        let bob_secret = x3dh_receive(&bob, &bob_store, &packet);

        assert_eq!(alice_secret.as_bytes(), bob_secret.as_bytes());
    }

    #[test]
    fn test_x3dh_shared_secret_is_32_bytes() {
        let alice = Identity::generate();
        let bob = Identity::generate();
        let bob_store = PreKeyStore::generate(&bob, 1);
        let bob_bundle = bob_store.bundle(&bob);

        let (_, secret) = x3dh_initiate(&alice, &bob_bundle);
        assert_eq!(secret.as_bytes().len(), 32);
    }

    #[test]
    fn test_x3dh_different_sessions_produce_different_secrets() {
        let alice = Identity::generate();
        let bob = Identity::generate();
        let bob_store = PreKeyStore::generate(&bob, 2);
        let bob_bundle = bob_store.bundle(&bob);

        let (_, s1) = x3dh_initiate(&alice, &bob_bundle);
        let (_, s2) = x3dh_initiate(&alice, &bob_bundle);

        assert_ne!(s1.as_bytes(), s2.as_bytes());
    }

    #[test]
    fn test_x3dh_shared_secret_is_not_all_zeros() {
        let alice = Identity::generate();
        let bob = Identity::generate();
        let bob_store = PreKeyStore::generate(&bob, 1);
        let bob_bundle = bob_store.bundle(&bob);

        let (_, secret) = x3dh_initiate(&alice, &bob_bundle);
        assert_ne!(secret.as_bytes(), &[0u8; 32]);
    }

    #[test]
    fn test_x3dh_works_without_opk() {
        let alice = Identity::generate();
        let bob = Identity::generate();
        let bob_store = PreKeyStore::generate(&bob, 0);
        let bob_bundle = bob_store.bundle(&bob);

        let (packet, alice_secret) = x3dh_initiate(&alice, &bob_bundle);
        let bob_secret = x3dh_receive(&bob, &bob_store, &packet);

        assert_eq!(alice_secret.as_bytes(), bob_secret.as_bytes());
    }
}
