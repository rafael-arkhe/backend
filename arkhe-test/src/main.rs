use arkhe_test::audit::*;
use arkhe_test::domain::*;
use arkhe_test::flags::*;
use arkhe_test::invariants::*;
use arkhe_test::rollback::*;
use arkhe_test::vectors::*;
use ed25519_dalek::SigningKey;
use std::collections::HashSet;
use std::env;
use std::fs;

fn run_pipeline(stage_filter: &str) -> TestExecutionReport {
    println!("============================================================");
    println!("  Arkhe-Test: Pipeline de Qualidade e Invariantes I-01..I-10");
    println!("  Filtro de Estágio: {}", stage_filter);
    println!("============================================================");

    let mut results = Vec::new();

    // -------------------------------------------------------------
    // Invariante I-01: Chunk Íntegro (OmniNode)
    // -------------------------------------------------------------
    {
        let data = b"Tensor chunk weights float16 raw block payload".to_vec();
        let chunk = OmniChunk::new(0, data.clone());
        let nom_ok = verify_i01_chunk_integrity(&chunk).is_ok();

        // Adversarial: 1 byte alterado
        let mut corrupted_data = data;
        corrupted_data[0] ^= 0xFF;
        let corrupted_chunk = OmniChunk {
            chunk_index: 0,
            data: corrupted_data,
            cid: chunk.cid.clone(),
        };
        let adv_rejection = verify_i01_chunk_integrity(&corrupted_chunk).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I01ChunkIntegrity.code().to_string(),
            description: InvariantId::I01ChunkIntegrity.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-01 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-02: Manifesto Verificável
    // -------------------------------------------------------------
    {
        let cids = vec![
            OmniChunk::compute_cid(b"c1"),
            OmniChunk::compute_cid(b"c2"),
            OmniChunk::compute_cid(b"c3"),
        ];
        let manifest = ModelManifest::new("arkhe-v1".to_string(), cids.clone());
        let nom_ok = verify_i02_verifiable_manifest(&manifest).is_ok();

        // Adversarial: trocar CID ou remover chunk
        let mut bad_cids = cids.clone();
        bad_cids[1] = OmniChunk::compute_cid(b"c2_tampered");
        let corrupted_manifest = ModelManifest {
            model_id: manifest.model_id.clone(),
            total_chunks: manifest.total_chunks,
            chunk_cids: bad_cids,
            root_cid: manifest.root_cid.clone(),
        };
        let adv_rejection = verify_i02_verifiable_manifest(&corrupted_manifest).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I02VerifiableManifest.code().to_string(),
            description: InvariantId::I02VerifiableManifest.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-02 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-03: Recibo AFiR (Assinatura e Nonce Anti-Replay)
    // -------------------------------------------------------------
    {
        let worker_pubkey = vec![0x12u8; 32];
        let wrong_pubkey = vec![0x99u8; 32];
        let mut seen_nonces = HashSet::new();

        let mut receipt = AfirReceipt {
            inference_root: [1u8; 32],
            chunk_index: 0,
            worker_id: [2u8; 32],
            nonce: 500,
            timestamp: 1730000000,
            ml_dsa_sig: vec![],
        };
        let sig = AfirReceipt::compute_sig(&receipt.signing_bytes(), &worker_pubkey);
        receipt.ml_dsa_sig = sig;

        let nom_ok = verify_i03_afir_receipt(&receipt, &worker_pubkey, &mut seen_nonces).is_ok();

        // Adversarial 1: replay de nonce
        let replay_rejection = verify_i03_afir_receipt(&receipt, &worker_pubkey, &mut seen_nonces).is_err();

        // Adversarial 2: assinatura com chave errada
        let mut fresh_nonces = HashSet::new();
        let wrong_key_rejection = verify_i03_afir_receipt(&receipt, &wrong_pubkey, &mut fresh_nonces).is_err();

        let adv_rejection = replay_rejection && wrong_key_rejection;

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I03AfirReceipt.code().to_string(),
            description: InvariantId::I03AfirReceipt.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-03 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-04: MMR Append-Only
    // -------------------------------------------------------------
    {
        let mut mmr_prev = MerkleMountainRange::new();
        mmr_prev.append([1u8; 32]);
        mmr_prev.append([2u8; 32]);

        let mut mmr_curr = mmr_prev.clone();
        mmr_curr.append([3u8; 32]);

        let nom_ok = verify_i04_mmr_append_only(&mmr_prev, &mmr_curr).is_ok();

        // Adversarial: reordenação / alteração de folha passada
        let mut mmr_tampered = mmr_curr.clone();
        mmr_tampered.leaves[0] = [0xFFu8; 32];
        mmr_tampered.recompute_root();
        let adv_rejection = verify_i04_mmr_append_only(&mmr_prev, &mmr_tampered).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I04MmrAppendOnly.code().to_string(),
            description: InvariantId::I04MmrAppendOnly.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-04 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Setup comum para I-05, I-06, I-09 (Offline Bundle)
    // -------------------------------------------------------------
    let titan_blob = b"TITAN_M2_HARDWARE_ATTESTATION_KEY_CERT_CHAIN".to_vec();
    let nonce_wer = [0x55u8; 32];
    let app_measurement = [0xAAu8; 32];
    let device_hash = MobileWER::compute_device_attestation_hash(&titan_blob, &nonce_wer, &app_measurement);

    let wer = MobileWER {
        version: 1,
        device_attestation_hash: device_hash,
        app_id: [0x11u8; 32],
        action: MobileAction::InferenceFragment { chunk_index: 0, total_chunks: 1 },
        input_digest: [0x22u8; 32],
        output_digest: [0x33u8; 32],
        policy_digest: [0x44u8; 32],
        performed_at: 1730001000,
        evidence_refs: vec![],
    };

    let mut mmr = MerkleMountainRange::new();
    mmr.append(wer.canonical_hash());
    let proof = mmr.get_inclusion_proof(0);

    let strongbox_key = vec![0x88u8; 32];
    let ed_seed = [0x42u8; 32];
    let ed_sk = SigningKey::from_bytes(&ed_seed);
    let ed_vk = ed_sk.verifying_key();

    let mut cp = HybridCheckpointEnvelope {
        mmr_size: mmr.leaves.len() as u64,
        mmr_root: mmr.root,
        timestamp: 1730001010,
        strongbox_pqc_sig: vec![],
        tee_ed25519_sig: vec![],
    };
    let cp_bytes = cp.signing_bytes();
    cp.strongbox_pqc_sig = HybridCheckpointEnvelope::compute_strongbox_sig(&cp_bytes, &strongbox_key);
    use ed25519_dalek::Signer;
    let ed_sig = ed_sk.sign(&cp_bytes);
    cp.tee_ed25519_sig = ed_sig.to_bytes().to_vec();

    let bundle = OfflineVerificationBundle {
        wer: wer.clone(),
        wer_index: 0,
        mmr_inclusion_proof: proof,
        checkpoint: cp.clone(),
        titan_attestation_blob: titan_blob.clone(),
        first_wer_nonce: nonce_wer,
        app_measurement,
    };

    // -------------------------------------------------------------
    // Invariante I-05: Auditoria Offline
    // -------------------------------------------------------------
    {
        let nom_ok = verify_i05_offline_audit(&bundle, &strongbox_key, ed_vk.as_bytes()).is_ok();

        // Adversarial: corromper output_digest na evidência
        let mut tampered_bundle = bundle.clone();
        tampered_bundle.wer.output_digest[0] ^= 0xFF;
        let adv_rejection = verify_i05_offline_audit(&tampered_bundle, &strongbox_key, ed_vk.as_bytes()).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I05OfflineAudit.code().to_string(),
            description: InvariantId::I05OfflineAudit.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-05 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-06: Vínculo ao Dispositivo (Titan M2 Hardware Binding)
    // -------------------------------------------------------------
    {
        let nom_ok = verify_i06_device_binding(&bundle, &app_measurement).is_ok();

        // Adversarial: binário com app_measurement alterado
        let rogue_app_measurement = [0xFEu8; 32];
        let adv_rejection = verify_i06_device_binding(&bundle, &rogue_app_measurement).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I06DeviceBinding.code().to_string(),
            description: InvariantId::I06DeviceBinding.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-06 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-07: Governança Humana
    // -------------------------------------------------------------
    {
        let canonical_proposal = GovernanceProposal {
            proposal_id: 1,
            proposer: "human_council".to_string(),
            is_autonomous_agent: false,
            targets_constitution: true,
            action_payload: "UPGRADE_CONSTITUTION_PARAMS".to_string(),
            human_canonical_approval: true,
        };
        let nom_ok = verify_i07_human_governance(&canonical_proposal).is_ok();

        // Adversarial: agente autônomo tentando mutar constituição sem aprovação humana
        let rogue_agent_proposal = GovernanceProposal {
            proposal_id: 2,
            proposer: "mcp_autonomous_agent_007".to_string(),
            is_autonomous_agent: true,
            targets_constitution: true,
            action_payload: "ALTER_IMMUTABLE_RULES".to_string(),
            human_canonical_approval: false,
        };
        let adv_rejection = verify_i07_human_governance(&rogue_agent_proposal).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I07HumanGovernance.code().to_string(),
            description: InvariantId::I07HumanGovernance.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-07 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-08: Reward por Prova
    // -------------------------------------------------------------
    {
        let worker_key = vec![0x11u8; 32];
        let mut receipt = AfirReceipt {
            inference_root: [0x55u8; 32],
            chunk_index: 0,
            worker_id: [0x66u8; 32],
            nonce: 88,
            timestamp: 1730002000,
            ml_dsa_sig: vec![],
        };
        let sig = AfirReceipt::compute_sig(&receipt.signing_bytes(), &worker_key);
        receipt.ml_dsa_sig = sig;

        let mut reward_mmr = MerkleMountainRange::new();
        reward_mmr.append(hash_sha256(&receipt.signing_bytes()));

        let nom_ok = verify_i08_reward_by_proof(&receipt, &worker_key, &reward_mmr, 0).is_ok();

        // Adversarial: tentar liberar reward sem recibo ancorado
        let empty_mmr = MerkleMountainRange::new();
        let adv_rejection = verify_i08_reward_by_proof(&receipt, &worker_key, &empty_mmr, 0).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I08RewardByProof.code().to_string(),
            description: InvariantId::I08RewardByProof.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-08 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-09: Evidência Chain-Agnostic
    // -------------------------------------------------------------
    {
        let nom_ok = verify_i09_chain_agnostic_evidence(&bundle, &strongbox_key, ed_vk.as_bytes()).is_ok();

        // Adversarial: checkpoint com assinatura adulterada
        let mut bad_bundle = bundle.clone();
        bad_bundle.checkpoint.strongbox_pqc_sig[0] ^= 0xAA;
        let adv_rejection = verify_i09_chain_agnostic_evidence(&bad_bundle, &strongbox_key, ed_vk.as_bytes()).is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I09ChainAgnosticEvidence.code().to_string(),
            description: InvariantId::I09ChainAgnosticEvidence.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-09 failed".to_string()) },
        });
    }

    // -------------------------------------------------------------
    // Invariante I-10: Papéis Disjuntos
    // -------------------------------------------------------------
    {
        let mut registry = EpochRoleRegistry::new(100);
        let node_a = [0x11u8; 32];
        let node_b = [0x22u8; 32];

        assert!(registry.register_worker(node_a).is_ok());
        assert!(registry.register_validator(node_b).is_ok());

        let nom_ok = verify_i10_disjoint_roles(&registry, &node_a, "worker").is_ok()
            && verify_i10_disjoint_roles(&registry, &node_b, "validator").is_ok();

        // Adversarial: node_a (worker) tentar validar no mesmo epoch
        let adv_rejection = registry.register_validator(node_a).is_err()
            && verify_i10_disjoint_roles(&registry, &node_a, "validator").is_err();

        results.push(InvariantAuditResult {
            invariant_code: InvariantId::I10DisjointRoles.code().to_string(),
            description: InvariantId::I10DisjointRoles.description().to_string(),
            passed: nom_ok && adv_rejection,
            nominal_test_passed: nom_ok,
            adversarial_rejection_passed: adv_rejection,
            error_message: if nom_ok && adv_rejection { None } else { Some("I-10 failed".to_string()) },
        });
    }

    TestExecutionReport::new(stage_filter, "HEAD", results)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("run");

    match command {
        "run" => {
            let stage = args.get(3).map(|s| s.as_str()).unwrap_or("all");
            let report = run_pipeline(stage);

            for res in &report.results {
                let status_icon = if res.passed { " [PASS] " } else { " [FAIL] " };
                println!("{}{} - {}", status_icon, res.invariant_code, res.description);
                println!("       Nominal: {} | Adversarial Rejection: {}",
                    if res.nominal_test_passed { "OK" } else { "FAIL" },
                    if res.adversarial_rejection_passed { "OK" } else { "BYPASSED!" }
                );
            }

            println!("------------------------------------------------------------");
            println!("Total Invariantes Testados: {}/{}", report.total_passed, report.total_invariants_tested);
            println!("Evidência Auditável (Content-Addressed Digest): {}", report.report_content_digest);
            println!("------------------------------------------------------------");

            // Save report
            fs::create_dir_all("reports/security").ok();
            let json = serde_json::to_string_pretty(&report).unwrap();
            fs::write("reports/security/latest_audit.json", json).expect("Failed to write audit report");

            if !report.all_invariants_satisfied {
                eprintln!("Pipeline falhou: nem todos os invariantes satisfizeram a política fail-closed!");
                std::process::exit(1);
            } else {
                println!("Arkhe-test: Todos os invariantes I-01..I-10 validados com sucesso!");
            }
        }
        "verify-flags" => {
            println!("Verificando política de Feature Flags (I-T06)...");
            let toml_str = fs::read_to_string("policies/flags.toml").expect("policies/flags.toml not found");
            let policy = FlagsPolicy::parse(&toml_str).expect("Failed to parse policies/flags.toml");
            match policy.validate_safety() {
                Ok(_) => {
                    println!("[PASS] Nenhuma flag desativa verificações obrigatórias de segurança.");
                }
                Err(errs) => {
                    eprintln!("[FAIL] Violações de segurança encontradas:");
                    for e in errs {
                        eprintln!("  - {}", e);
                    }
                    std::process::exit(1);
                }
            }
        }
        "verify-rollback" => {
            println!("Verificando harness de Rollback Seguro (I-T05)...");
            let mut mmr = MerkleMountainRange::new();
            mmr.append([1u8; 32]);
            let state = NodeState {
                software_version: "0.2.0".to_string(),
                config_version: "v2".to_string(),
                mmr,
                receipts: vec![],
                checkpoints: vec![],
            };
            let rolled = RollbackHarness::execute_rollback(&state, "0.1.9".to_string(), "v1".to_string()).unwrap();
            assert_eq!(rolled.software_version, "0.1.9");
            assert_eq!(rolled.mmr.root, state.mmr.root);
            println!("[PASS] Rollback concluído com preservação estrita de integridade de MMR e evidências.");
        }
        "generate-vectors" => {
            let vectors = generate_golden_vectors();
            fs::create_dir_all("vectors/afir").ok();
            fs::create_dir_all("vectors/mmr").ok();
            fs::create_dir_all("vectors/checkpoint").ok();
            fs::create_dir_all("vectors/fraud").ok();

            fs::write("vectors/afir/afir_golden.json", serde_json::to_string_pretty(&vectors.afir_vector).unwrap()).unwrap();
            fs::write("vectors/checkpoint/checkpoint_golden.json", serde_json::to_string_pretty(&vectors.checkpoint_vector).unwrap()).unwrap();
            fs::write("vectors/mmr/chunk_manifest_golden.json", serde_json::to_string_pretty(&vectors.manifest_vector).unwrap()).unwrap();
            println!("[OK] Vetores dourados gerados.");
        }
        _ => {
            eprintln!("Comando desconhecido. Use: run | verify-flags | verify-rollback | generate-vectors");
            std::process::exit(1);
        }
    }
}
