use crate::domain::*;
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InvariantId {
    I01ChunkIntegrity,
    I02VerifiableManifest,
    I03AfirReceipt,
    I04MmrAppendOnly,
    I05OfflineAudit,
    I06DeviceBinding,
    I07HumanGovernance,
    I08RewardByProof,
    I09ChainAgnosticEvidence,
    I10DisjointRoles,
}

impl InvariantId {
    pub fn code(&self) -> &'static str {
        match self {
            InvariantId::I01ChunkIntegrity => "I-01",
            InvariantId::I02VerifiableManifest => "I-02",
            InvariantId::I03AfirReceipt => "I-03",
            InvariantId::I04MmrAppendOnly => "I-04",
            InvariantId::I05OfflineAudit => "I-05",
            InvariantId::I06DeviceBinding => "I-06",
            InvariantId::I07HumanGovernance => "I-07",
            InvariantId::I08RewardByProof => "I-08",
            InvariantId::I09ChainAgnosticEvidence => "I-09",
            InvariantId::I10DisjointRoles => "I-10",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            InvariantId::I01ChunkIntegrity => "Chunk íntegro (alteração de 1 byte resulta em rejeição imediata)",
            InvariantId::I02VerifiableManifest => "Manifesto verificável (troca de CID ou chunk faltante invalida manifesto)",
            InvariantId::I03AfirReceipt => "Recibo AFiR (assinatura ML-DSA-65 válida e proteção contra replay de nonce)",
            InvariantId::I04MmrAppendOnly => "MMR append-only (reordenamento ou remoção de folhas invalida a raiz)",
            InvariantId::I05OfflineAudit => "Auditoria offline (bundle offline detecta reescrita sem conexão de rede)",
            InvariantId::I06DeviceBinding => "Vínculo ao dispositivo (chave e atestação Titan/StrongBox obrigatórias)",
            InvariantId::I07HumanGovernance => "Governança humana (proposta autônoma não altera constituição sem aprovação)",
            InvariantId::I08RewardByProof => "Reward por prova (pagamento bloqueado sem recibo e MMR válidos)",
            InvariantId::I09ChainAgnosticEvidence => "Evidência chain-agnostic (verificação de bundle independente de blockchain)",
            InvariantId::I10DisjointRoles => "Papéis disjuntos (worker não pode validar no mesmo epoch)",
        }
    }
}

/// I-01: Valida integridade do chunk OmniNode.
pub fn verify_i01_chunk_integrity(chunk: &OmniChunk) -> Result<(), &'static str> {
    if !chunk.verify() {
        return Err("I-01 VIOLATION: Chunk digest mismatch against declared CIDv1");
    }
    Ok(())
}

/// I-02: Valida integridade e consistência do manifesto.
pub fn verify_i02_verifiable_manifest(manifest: &ModelManifest) -> Result<(), &'static str> {
    if !manifest.verify() {
        return Err("I-02 VIOLATION: ModelManifest root CID mismatch or chunk count discrepancy");
    }
    Ok(())
}

/// I-03: Valida recibo AFiR e detecção de repetição de nonce.
pub fn verify_i03_afir_receipt(
    receipt: &AfirReceipt,
    worker_pubkey: &[u8],
    seen_nonces: &mut HashSet<u64>,
) -> Result<(), &'static str> {
    if seen_nonces.contains(&receipt.nonce) {
        return Err("I-03 VIOLATION: Nonce replay detected");
    }
    if !receipt.verify(worker_pubkey) {
        return Err("I-03 VIOLATION: Invalid ML-DSA-65 signature on AFiR receipt");
    }
    seen_nonces.insert(receipt.nonce);
    Ok(())
}

/// I-04: Valida integridade append-only da MMR comparada à anterior.
pub fn verify_i04_mmr_append_only(
    previous: &MerkleMountainRange,
    current: &MerkleMountainRange,
) -> Result<(), &'static str> {
    if current.leaves.len() < previous.leaves.len() {
        return Err("I-04 VIOLATION: MMR length decreased (deletion detected)");
    }
    for (i, prev_leaf) in previous.leaves.iter().enumerate() {
        if &current.leaves[i] != prev_leaf {
            return Err("I-04 VIOLATION: Past MMR leaf rewritten or reordered");
        }
    }
    // Raiz deve ser consistente
    let mut expected_current = current.clone();
    expected_current.recompute_root();
    if expected_current.root != current.root {
        return Err("I-04 VIOLATION: Current MMR root is corrupt");
    }
    Ok(())
}

/// I-05: Auditoria offline estrita do bundle.
pub fn verify_i05_offline_audit(
    bundle: &OfflineVerificationBundle,
    strongbox_pqc_pubkey: &[u8],
    tee_ed25519_pubkey: &[u8; 32],
) -> Result<(), &'static str> {
    bundle
        .verify_offline(strongbox_pqc_pubkey, tee_ed25519_pubkey)
        .map_err(|_| "I-05 VIOLATION: Offline bundle verification failed or evidence corrupted")
}

/// I-06: Vínculo estrito de chave e hardware Titan M2/StrongBox.
pub fn verify_i06_device_binding(
    bundle: &OfflineVerificationBundle,
    known_app_measurement: &Digest32,
) -> Result<(), &'static str> {
    if &bundle.app_measurement != known_app_measurement {
        return Err("I-06 VIOLATION: App measurement does not match authorized runtime measurement");
    }
    let expected_hash = MobileWER::compute_device_attestation_hash(
        &bundle.titan_attestation_blob,
        &bundle.first_wer_nonce,
        &bundle.app_measurement,
    );
    if bundle.wer.device_attestation_hash != expected_hash {
        return Err("I-06 VIOLATION: Hardware attestation blob not bound to MobileWER");
    }
    Ok(())
}

/// I-07: Governança humana - propostas autônomas não podem alterar a constituição sem aprovação canônica.
pub fn verify_i07_human_governance(proposal: &GovernanceProposal) -> Result<(), &'static str> {
    if proposal.targets_constitution && !proposal.human_canonical_approval {
        return Err("I-07 VIOLATION: Constitutional mutation requires explicit human canonical approval");
    }
    Ok(())
}

/// I-08: Reward por prova - incentivo/pagamento exige recibo AFiR válido e inclusão na MMR.
pub fn verify_i08_reward_by_proof(
    receipt: &AfirReceipt,
    worker_pubkey: &[u8],
    mmr: &MerkleMountainRange,
    receipt_leaf_index: usize,
) -> Result<(), &'static str> {
    if !receipt.verify(worker_pubkey) {
        return Err("I-08 VIOLATION: Payment rejected due to invalid AFiR receipt");
    }
    let leaf_hash = hash_sha256(&receipt.signing_bytes());
    if receipt_leaf_index >= mmr.leaves.len() || mmr.leaves[receipt_leaf_index] != leaf_hash {
        return Err("I-08 VIOLATION: Payment rejected because proof is not anchored in MMR");
    }
    Ok(())
}

/// I-09: Evidência chain-agnostic - a validação deve ser realizada com sucesso de modo puramente local/offline sem nós RPC blockchain.
pub fn verify_i09_chain_agnostic_evidence(
    bundle: &OfflineVerificationBundle,
    strongbox_pqc_pubkey: &[u8],
    tee_ed25519_pubkey: &[u8; 32],
) -> Result<(), &'static str> {
    // Executa verificação autônoma sem requisições remotas
    verify_i05_offline_audit(bundle, strongbox_pqc_pubkey, tee_ed25519_pubkey)
        .map_err(|_| "I-09 VIOLATION: Chain-agnostic evidence could not be verified locally")
}

/// I-10: Papéis disjuntos - um mesmo nó não pode exercer funções de worker e validador no mesmo epoch.
pub fn verify_i10_disjoint_roles(
    registry: &EpochRoleRegistry,
    node_id: &Digest32,
    requested_role: &str,
) -> Result<(), &'static str> {
    match requested_role {
        "worker" => {
            if registry.validators.contains(node_id) {
                return Err("I-10 VIOLATION: Node is already validator in epoch; cannot act as worker");
            }
        }
        "validator" => {
            if registry.workers.contains(node_id) {
                return Err("I-10 VIOLATION: Node is already worker in epoch; cannot act as validator");
            }
        }
        _ => return Err("Invalid role requested"),
    }
    Ok(())
}
