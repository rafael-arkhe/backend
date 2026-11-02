use arkhe_test::domain::*;
use arkhe_test::invariants::*;
use arkhe_test::rollback::*;

#[test]
fn test_stage2_end_to_end_integration() {
    // 1. Chunk downloaded and verified
    let data = b"Layer_0_Weights_GGUF".to_vec();
    let chunk = OmniChunk::new(0, data.clone());
    assert!(verify_i01_chunk_integrity(&chunk).is_ok());

    // 2. Receipt signed and appended to MMR
    let worker_key = vec![0x99; 32];
    let mut receipt = AfirReceipt {
        inference_root: hash_sha256(&data),
        chunk_index: 0,
        worker_id: [0x50; 32],
        nonce: 10,
        timestamp: 1730000000,
        ml_dsa_sig: vec![],
    };
    receipt.ml_dsa_sig = AfirReceipt::compute_sig(&receipt.signing_bytes(), &worker_key);

    let mut mmr = MerkleMountainRange::new();
    let receipt_digest = hash_sha256(&receipt.signing_bytes());
    mmr.append(receipt_digest);

    assert!(verify_i08_reward_by_proof(&receipt, &worker_key, &mmr, 0).is_ok());

    // 3. Rollback preserves evidence
    let state = NodeState {
        software_version: "v2.0".to_string(),
        config_version: "cfg2".to_string(),
        mmr: mmr.clone(),
        receipts: vec![receipt],
        checkpoints: vec![],
    };
    let rolled = RollbackHarness::execute_rollback(&state, "v1.0".to_string(), "cfg1".to_string()).unwrap();
    assert_eq!(rolled.mmr.root, mmr.root);
    assert_eq!(rolled.receipts.len(), 1);
}
