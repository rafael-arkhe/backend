use arkhe_test::domain::*;
use arkhe_test::invariants::*;
use std::collections::HashSet;

#[test]
fn test_stage3_adversarial_rejections() {
    // 1. Corrupt chunk
    let mut chunk = OmniChunk::new(0, b"Hello".to_vec());
    chunk.data[0] ^= 0x01;
    assert!(verify_i01_chunk_integrity(&chunk).is_err());

    // 2. Corrupted manifest
    let manifest = ModelManifest {
        model_id: "arkhe".to_string(),
        total_chunks: 2,
        chunk_cids: vec!["cid1".to_string()],
        root_cid: "root".to_string(),
    };
    assert!(verify_i02_verifiable_manifest(&manifest).is_err());

    // 3. Replay nonce
    let key = vec![0x10; 32];
    let mut seen = HashSet::new();
    let mut afir = AfirReceipt {
        inference_root: [0; 32],
        chunk_index: 0,
        worker_id: [1; 32],
        nonce: 42,
        timestamp: 100,
        ml_dsa_sig: vec![],
    };
    afir.ml_dsa_sig = AfirReceipt::compute_sig(&afir.signing_bytes(), &key);
    assert!(verify_i03_afir_receipt(&afir, &key, &mut seen).is_ok());
    assert!(verify_i03_afir_receipt(&afir, &key, &mut seen).is_err());

    // 4. Constitution breach by autonomous agent
    let rogue_proposal = GovernanceProposal {
        proposal_id: 99,
        proposer: "agent_x".to_string(),
        is_autonomous_agent: true,
        targets_constitution: true,
        action_payload: "CHANGE_RULES".to_string(),
        human_canonical_approval: false,
    };
    assert!(verify_i07_human_governance(&rogue_proposal).is_err());

    // 5. Worker acting as validator in same epoch
    let mut reg = EpochRoleRegistry::new(1);
    let node_id = [0x77; 32];
    assert!(reg.register_worker(node_id).is_ok());
    assert!(verify_i10_disjoint_roles(&reg, &node_id, "validator").is_err());
}
