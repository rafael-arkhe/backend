use arkhe_test::domain::*;
use arkhe_test::invariants::*;
use std::collections::HashSet;

#[test]
fn test_stage1_i01_chunk_integrity_nominal() {
    let chunk = OmniChunk::new(0, b"Tensor payload".to_vec());
    assert!(verify_i01_chunk_integrity(&chunk).is_ok());
}

#[test]
fn test_stage1_i02_manifest_nominal() {
    let cids = vec![
        OmniChunk::compute_cid(b"chunk1"),
        OmniChunk::compute_cid(b"chunk2"),
    ];
    let manifest = ModelManifest::new("model_a".to_string(), cids);
    assert!(verify_i02_verifiable_manifest(&manifest).is_ok());
}

#[test]
fn test_stage1_i03_afir_nominal() {
    let key = vec![0x10; 32];
    let mut seen = HashSet::new();
    let mut afir = AfirReceipt {
        inference_root: [0; 32],
        chunk_index: 0,
        worker_id: [1; 32],
        nonce: 1,
        timestamp: 100,
        ml_dsa_sig: vec![],
    };
    afir.ml_dsa_sig = AfirReceipt::compute_sig(&afir.signing_bytes(), &key);
    assert!(verify_i03_afir_receipt(&afir, &key, &mut seen).is_ok());
}

#[test]
fn test_stage1_i04_mmr_append_only_nominal() {
    let mut prev = MerkleMountainRange::new();
    prev.append([1; 32]);
    let mut curr = prev.clone();
    curr.append([2; 32]);
    assert!(verify_i04_mmr_append_only(&prev, &curr).is_ok());
}
