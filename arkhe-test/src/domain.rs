use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub type Digest32 = [u8; 32];

pub fn hash_sha256(data: &[u8]) -> Digest32 {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

pub fn hash_concat(parts: &[&[u8]]) -> Digest32 {
    let mut hasher = Sha256::new();
    for part in parts {
        hasher.update(part);
    }
    let result = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(&result);
    out
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OmniChunk {
    pub chunk_index: u32,
    pub data: Vec<u8>,
    pub cid: String, // e.g., "bafk..."
}

impl OmniChunk {
    pub fn compute_cid(data: &[u8]) -> String {
        let hash = hash_sha256(data);
        format!("bafk{}", hex::encode(hash))
    }

    pub fn new(chunk_index: u32, data: Vec<u8>) -> Self {
        let cid = Self::compute_cid(&data);
        Self {
            chunk_index,
            data,
            cid,
        }
    }

    pub fn verify(&self) -> bool {
        let expected_cid = Self::compute_cid(&self.data);
        self.cid == expected_cid
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModelManifest {
    pub model_id: String,
    pub total_chunks: u32,
    pub chunk_cids: Vec<String>,
    pub root_cid: String,
}

impl ModelManifest {
    pub fn compute_root_cid(cids: &[String]) -> String {
        let joined = cids.join(":");
        let h = hash_sha256(joined.as_bytes());
        format!("bafkroot{}", hex::encode(h))
    }

    pub fn new(model_id: String, chunk_cids: Vec<String>) -> Self {
        let root_cid = Self::compute_root_cid(&chunk_cids);
        Self {
            model_id,
            total_chunks: chunk_cids.len() as u32,
            chunk_cids,
            root_cid,
        }
    }

    pub fn verify(&self) -> bool {
        if self.chunk_cids.len() as u32 != self.total_chunks {
            return false;
        }
        let expected_root = Self::compute_root_cid(&self.chunk_cids);
        self.root_cid == expected_root
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct AfirReceipt {
    pub inference_root: Digest32,
    pub chunk_index: u32,
    pub worker_id: Digest32,
    pub nonce: u64,
    pub timestamp: u64,
    pub ml_dsa_sig: Vec<u8>, // Mocked/simulated ML-DSA-65 signature
}

impl AfirReceipt {
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"ARKHE:AFIR:v1\0");
        b.extend_from_slice(&self.inference_root);
        b.extend_from_slice(&self.chunk_index.to_be_bytes());
        b.extend_from_slice(&self.worker_id);
        b.extend_from_slice(&self.nonce.to_be_bytes());
        b.extend_from_slice(&self.timestamp.to_be_bytes());
        b
    }

    pub fn compute_sig(signing_bytes: &[u8], secret_key: &[u8]) -> Vec<u8> {
        let h = hash_concat(&[b"ML-DSA-65:", signing_bytes, secret_key]);
        let mut sig = Vec::with_capacity(384);
        for i in 0..12 {
            let mut round_hasher = Sha256::new();
            round_hasher.update(&h);
            round_hasher.update(&[i as u8]);
            sig.extend_from_slice(&round_hasher.finalize());
        }
        sig
    }

    pub fn verify(&self, expected_worker_pubkey: &[u8]) -> bool {
        if self.ml_dsa_sig.len() != 384 {
            return false;
        }
        let sb = self.signing_bytes();
        let expected_sig = Self::compute_sig(&sb, expected_worker_pubkey);
        self.ml_dsa_sig == expected_sig
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum MobileAction {
    InferenceFragment { chunk_index: u32, total_chunks: u32 },
    AgentDecision { decision_type: u16, confidence_score: u16 },
    StateTransition { from_state: Digest32, to_state: Digest32 },
    PolicyAttestation { rule_id: u32 },
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MobileWER {
    pub version: u8,
    pub device_attestation_hash: Digest32,
    pub app_id: Digest32,
    pub action: MobileAction,
    pub input_digest: Digest32,
    pub output_digest: Digest32,
    pub policy_digest: Digest32,
    pub performed_at: u64,
    pub evidence_refs: Vec<Digest32>,
}

impl MobileWER {
    pub const DOMAIN: &'static [u8] = b"AKG:MOBILE:ACT:v1\0";

    pub fn compute_device_attestation_hash(
        titan_attestation_blob: &[u8],
        first_wer_nonce: &[u8; 32],
        app_measurement: &Digest32,
    ) -> Digest32 {
        hash_concat(&[titan_attestation_blob, first_wer_nonce, app_measurement])
    }

    pub fn canonical_hash(&self) -> Digest32 {
        let json = serde_json::to_vec(self).unwrap_or_default();
        hash_concat(&[Self::DOMAIN, &json])
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MerkleMountainRange {
    pub leaves: Vec<Digest32>,
    pub root: Digest32,
}

impl MerkleMountainRange {
    pub fn new() -> Self {
        Self {
            leaves: Vec::new(),
            root: [0u8; 32],
        }
    }

    pub fn append(&mut self, leaf: Digest32) {
        self.leaves.push(leaf);
        self.recompute_root();
    }

    pub fn recompute_root(&mut self) {
        if self.leaves.is_empty() {
            self.root = [0u8; 32];
            return;
        }
        let mut cur = self.leaves.clone();
        while cur.len() > 1 {
            let mut next = Vec::new();
            for chunk in cur.chunks(2) {
                if chunk.len() == 2 {
                    next.push(hash_concat(&[&chunk[0], &chunk[1]]));
                } else {
                    next.push(hash_concat(&[&chunk[0], &chunk[0]]));
                }
            }
            cur = next;
        }
        self.root = cur[0];
    }

    pub fn get_inclusion_proof(&self, index: usize) -> Vec<Digest32> {
        let mut proof = Vec::new();
        if index >= self.leaves.len() {
            return proof;
        }
        let mut cur = self.leaves.clone();
        let mut cur_idx = index;
        while cur.len() > 1 {
            let sibling_idx = if cur_idx % 2 == 0 {
                if cur_idx + 1 < cur.len() {
                    cur_idx + 1
                } else {
                    cur_idx
                }
            } else {
                cur_idx - 1
            };
            proof.push(cur[sibling_idx]);

            let mut next = Vec::new();
            for chunk in cur.chunks(2) {
                if chunk.len() == 2 {
                    next.push(hash_concat(&[&chunk[0], &chunk[1]]));
                } else {
                    next.push(hash_concat(&[&chunk[0], &chunk[0]]));
                }
            }
            cur = next;
            cur_idx /= 2;
        }
        proof
    }

    pub fn verify_inclusion(leaf: &Digest32, proof: &[Digest32], leaf_index: usize, expected_root: &Digest32) -> bool {
        let mut current = *leaf;
        let mut idx = leaf_index;
        for sibling in proof {
            current = if idx % 2 == 0 {
                hash_concat(&[&current, sibling])
            } else {
                hash_concat(&[sibling, &current])
            };
            idx /= 2;
        }
        current == *expected_root
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridCheckpointEnvelope {
    pub mmr_size: u64,
    pub mmr_root: Digest32,
    pub timestamp: u64,
    pub strongbox_pqc_sig: Vec<u8>,
    pub tee_ed25519_sig: Vec<u8>,
}

impl HybridCheckpointEnvelope {
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut b = Vec::new();
        b.extend_from_slice(b"ARKHE:MMR:CHECKPOINT:v1\0");
        b.extend_from_slice(&self.mmr_size.to_be_bytes());
        b.extend_from_slice(&self.mmr_root);
        b.extend_from_slice(&self.timestamp.to_be_bytes());
        b
    }

    pub fn compute_strongbox_sig(signing_bytes: &[u8], pqc_key: &[u8]) -> Vec<u8> {
        let h = hash_concat(&[b"STRONGBOX:ML-DSA-65:", signing_bytes, pqc_key]);
        let mut sig = Vec::with_capacity(384);
        for i in 0..12 {
            let mut round = Sha256::new();
            round.update(&h);
            round.update(&[i as u8]);
            sig.extend_from_slice(&round.finalize());
        }
        sig
    }

    pub fn verify(
        &self,
        strongbox_pqc_pubkey: &[u8],
        tee_ed25519_pubkey: &[u8; 32],
    ) -> bool {
        if self.strongbox_pqc_sig.len() != 384 {
            return false;
        }
        let sb = self.signing_bytes();
        let expected_strongbox_sig = Self::compute_strongbox_sig(&sb, strongbox_pqc_pubkey);
        if self.strongbox_pqc_sig != expected_strongbox_sig {
            return false;
        }

        // Verify ed25519
        use ed25519_dalek::{Signature, VerifyingKey, Verifier};
        let vk = match VerifyingKey::from_bytes(tee_ed25519_pubkey) {
            Ok(k) => k,
            Err(_) => return false,
        };
        if self.tee_ed25519_sig.len() != 64 {
            return false;
        }
        let mut sig_arr = [0u8; 64];
        sig_arr.copy_from_slice(&self.tee_ed25519_sig);
        let sig = Signature::from_bytes(&sig_arr);
        vk.verify(&sb, &sig).is_ok()
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineVerificationBundle {
    pub wer: MobileWER,
    pub wer_index: usize,
    pub mmr_inclusion_proof: Vec<Digest32>,
    pub checkpoint: HybridCheckpointEnvelope,
    pub titan_attestation_blob: Vec<u8>,
    pub first_wer_nonce: [u8; 32],
    pub app_measurement: Digest32,
}

impl OfflineVerificationBundle {
    pub fn verify_offline(
        &self,
        strongbox_pqc_pubkey: &[u8],
        tee_ed25519_pubkey: &[u8; 32],
    ) -> Result<(), &'static str> {
        // 1. Hardware Binding
        let expected_device_hash = MobileWER::compute_device_attestation_hash(
            &self.titan_attestation_blob,
            &self.first_wer_nonce,
            &self.app_measurement,
        );
        if self.wer.device_attestation_hash != expected_device_hash {
            return Err("Hardware attestation binding mismatch");
        }

        // 2. MMR Inclusion
        let leaf_hash = self.wer.canonical_hash();
        if !MerkleMountainRange::verify_inclusion(
            &leaf_hash,
            &self.mmr_inclusion_proof,
            self.wer_index,
            &self.checkpoint.mmr_root,
        ) {
            return Err("MMR inclusion proof failed");
        }

        // 3. Checkpoint Dual Signatures
        if !self.checkpoint.verify(strongbox_pqc_pubkey, tee_ed25519_pubkey) {
            return Err("Hybrid checkpoint signature invalid");
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct GovernanceProposal {
    pub proposal_id: u64,
    pub proposer: String,
    pub is_autonomous_agent: bool,
    pub targets_constitution: bool,
    pub action_payload: String,
    pub human_canonical_approval: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct EpochRoleRegistry {
    pub epoch: u64,
    pub workers: Vec<Digest32>,
    pub validators: Vec<Digest32>,
}

impl EpochRoleRegistry {
    pub fn new(epoch: u64) -> Self {
        Self {
            epoch,
            workers: Vec::new(),
            validators: Vec::new(),
        }
    }

    pub fn register_worker(&mut self, id: Digest32) -> Result<(), &'static str> {
        if self.validators.contains(&id) {
            return Err("Disjoint roles invariant violated: cannot be worker and validator in same epoch");
        }
        self.workers.push(id);
        Ok(())
    }

    pub fn register_validator(&mut self, id: Digest32) -> Result<(), &'static str> {
        if self.workers.contains(&id) {
            return Err("Disjoint roles invariant violated: cannot be worker and validator in same epoch");
        }
        self.validators.push(id);
        Ok(())
    }
}
