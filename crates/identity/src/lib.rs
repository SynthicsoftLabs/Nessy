// Copyright 2026 Synthicsoft Labs LLC
// Licensed under the Apache License, Version 2.0.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentIdentity {
    pub id: String,
    pub key_fingerprint: String,
    pub created_at: u64,
}

impl AgentIdentity {
    pub fn generate() -> Self {
        let seed = Uuid::new_v4().to_string();
        let fingerprint = Sha256::digest(seed.as_bytes());
        Self {
            id: format!("did:key:{}", hex(&fingerprint)),
            key_fingerprint: hex(&fingerprint),
            created_at: current_unix_time(),
        }
    }

    pub fn verify_fingerprint(&self) -> bool {
        self.id
            .strip_prefix("did:key:")
            .is_some_and(|value| value == self.key_fingerprint)
    }
}

fn current_unix_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs())
}

fn hex(bytes: &[u8]) -> String {
    const TABLE: &[u8; 16] = b"0123456789abcdef";
    bytes
        .iter()
        .flat_map(|b| {
            [
                TABLE[(b >> 4) as usize] as char,
                TABLE[(b & 0x0f) as usize] as char,
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_identity_has_verifiable_fingerprint() {
        let identity = AgentIdentity::generate();
        assert!(identity.verify_fingerprint());
        assert!(identity.id.starts_with("did:key:"));
    }
}
