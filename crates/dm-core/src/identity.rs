use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use zeroize::ZeroizeOnDrop;

#[derive(ZeroizeOnDrop)]
pub struct Identity {
    signing_key: SigningKey,
    nid: [u8; 32],
}

impl Identity {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let nid = Self::derive_nid(signing_key.verifying_key());
        Self { signing_key, nid }
    }

    pub fn verifying_key(&self) -> VerifyingKey {
        self.signing_key.verifying_key()
    }

    pub fn nid(&self) -> &[u8; 32] {
        &self.nid
    }

    pub fn nid_hex(&self) -> String {
        self.nid.iter().map(|b| format!("{b:02x}")).collect()
    }

    fn derive_nid(vk: VerifyingKey) -> [u8; 32] {
        Sha256::digest(vk.as_bytes()).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    #[test]
    fn test_nid_is_32_bytes() {
        let id = Identity::generate();
        assert_eq!(id.nid().len(), 32);
    }

    #[test]
    fn test_nid_hex_is_64_lowercase_chars() {
        let id = Identity::generate();
        let hex = id.nid_hex();
        assert_eq!(hex.len(), 64);
        assert!(hex
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
    }

    #[test]
    fn test_nid_is_sha256_of_verifying_key() {
        let id = Identity::generate();
        let vk = id.verifying_key();
        let expected: [u8; 32] = Sha256::digest(vk.as_bytes()).into();
        assert_eq!(id.nid(), &expected);
    }

    #[test]
    fn test_different_identities_have_different_nids() {
        let a = Identity::generate();
        let b = Identity::generate();
        assert_ne!(a.nid(), b.nid());
    }

    #[test]
    fn test_nid_hex_decodes_to_nid_bytes() {
        let id = Identity::generate();
        let decoded = hex::decode(id.nid_hex()).expect("hex should decode");
        assert_eq!(decoded.as_slice(), id.nid().as_slice());
    }
}
