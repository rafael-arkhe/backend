use crate::domain::*;
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GoldenVectorSet {
    pub chunk_vector: ChunkGoldenVector,
    pub manifest_vector: ManifestGoldenVector,
    pub afir_vector: AfirGoldenVector,
    pub checkpoint_vector: CheckpointGoldenVector,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkGoldenVector {
    pub chunk_index: u32,
    pub data_hex: String,
    pub expected_cid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ManifestGoldenVector {
    pub model_id: String,
    pub chunk_cids: Vec<String>,
    pub expected_root_cid: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AfirGoldenVector {
    pub inference_root_hex: String,
    pub chunk_index: u32,
    pub worker_id_hex: String,
    pub nonce: u64,
    pub timestamp: u64,
    pub worker_pubkey_hex: String,
    pub signature_hex: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CheckpointGoldenVector {
    pub mmr_size: u64,
    pub mmr_root_hex: String,
    pub timestamp: u64,
    pub strongbox_pqc_pubkey_hex: String,
    pub tee_ed25519_pubkey_hex: String,
    pub strongbox_pqc_sig_hex: String,
    pub tee_ed25519_sig_hex: String,
}

pub fn generate_golden_vectors() -> GoldenVectorSet {
    // 1. Chunk
    let chunk_data = b"ARKHE_OMNINODE_TENSOR_WEIGHT_TEST_VECTOR_CHUNK_001".to_vec();
    let expected_cid = OmniChunk::compute_cid(&chunk_data);
    let chunk_vector = ChunkGoldenVector {
        chunk_index: 0,
        data_hex: hex::encode(&chunk_data),
        expected_cid: expected_cid.clone(),
    };

    // 2. Manifest
    let chunk_cids = vec![
        expected_cid.clone(),
        OmniChunk::compute_cid(b"CHUNK_002"),
        OmniChunk::compute_cid(b"CHUNK_003"),
    ];
    let expected_root_cid = ModelManifest::compute_root_cid(&chunk_cids);
    let manifest_vector = ManifestGoldenVector {
        model_id: "arkhe-llm-70b-v1".to_string(),
        chunk_cids,
        expected_root_cid,
    };

    // 3. AFiR
    let inference_root = [0x11u8; 32];
    let worker_id = [0x22u8; 32];
    let worker_pubkey = vec![0x33u8; 32];
    let nonce = 1001u64;
    let timestamp = 1730500000u64;
    let afir = AfirReceipt {
        inference_root,
        chunk_index: 0,
        worker_id,
        nonce,
        timestamp,
        ml_dsa_sig: vec![],
    };
    let sig = AfirReceipt::compute_sig(&afir.signing_bytes(), &worker_pubkey);
    let afir_vector = AfirGoldenVector {
        inference_root_hex: hex::encode(inference_root),
        chunk_index: 0,
        worker_id_hex: hex::encode(worker_id),
        nonce,
        timestamp,
        worker_pubkey_hex: hex::encode(&worker_pubkey),
        signature_hex: hex::encode(&sig),
    };

    // 4. Checkpoint
    let mmr_root = [0x44u8; 32];
    let strongbox_pqc_key = vec![0x55u8; 32];
    let ed25519_seed = [0x77u8; 32];
    let signing_key = SigningKey::from_bytes(&ed25519_seed);
    let verifying_key = signing_key.verifying_key();

    let checkpoint = HybridCheckpointEnvelope {
        mmr_size: 10,
        mmr_root,
        timestamp: 1730500100u64,
        strongbox_pqc_sig: vec![],
        tee_ed25519_sig: vec![],
    };
    let sb = checkpoint.signing_bytes();
    let pqc_sig = HybridCheckpointEnvelope::compute_strongbox_sig(&sb, &strongbox_pqc_key);

    use ed25519_dalek::Signer;
    let ed_sig = signing_key.sign(&sb);

    let checkpoint_vector = CheckpointGoldenVector {
        mmr_size: 10,
        mmr_root_hex: hex::encode(mmr_root),
        timestamp: 1730500100u64,
        strongbox_pqc_pubkey_hex: hex::encode(&strongbox_pqc_key),
        tee_ed25519_pubkey_hex: hex::encode(verifying_key.as_bytes()),
        strongbox_pqc_sig_hex: hex::encode(&pqc_sig),
        tee_ed25519_sig_hex: hex::encode(ed_sig.to_bytes()),
    };

    GoldenVectorSet {
        chunk_vector,
        manifest_vector,
        afir_vector,
        checkpoint_vector,
    }
}
