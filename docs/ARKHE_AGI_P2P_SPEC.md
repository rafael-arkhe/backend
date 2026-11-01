# Arkhe AGI-P2P & ArkheOS Mobile Verification Specification

## 1. Visão Geral e Fundamentos Teóricos

O framework **AGI-P2P** resolve a convergência entre capacidades cognitivas distribuídas e arquiteturas estritamente descentralizadas (sem autoridade central de agregação de pesos ou checkpoints).

A arquitetura organiza-se em uma **pilha de verificação em profundidade de 4 camadas (L1–L4)**, onde cada camada mitiga vetores específicos de risco e delega a verificação criptográfica pontual ou histórica para a camada adequada.

```text
┌──────────────────────────────────────────────────────────────────────┐
│ L4 — GOVERNANÇA E INCENTIVOS                                         │
│ OpenCLAW-P2P (BFT + reputação) · A.N.N. (sem autoridade central)    │
│ Risco residual: Sybil, colusão tácita, limite de escala BFT          │
├──────────────────────────────────────────────────────────────────────┤
│ L3 — EVIDÊNCIA LOCAL E CONSISTÊNCIA HISTÓRICA                        │
│ ArkheOS Mobile: MobileWER · MMR selado pelo StrongBox                │
│ Binding: attestation_blob ↔ MobileWER · OfflineVerificationBundle    │
│ Risco residual: comprometimento do Titan M2 / Hardware Root          │
├──────────────────────────────────────────────────────────────────────┤
│ L2 — PROVENIÊNCIA DA INFERÊNCIA                                      │
│ IETF SPICE (inference_root) · AFiR (ML-DSA-65, TEE-free)            │
│ ZKML (opcional para alta certeza) · TEE (apenas aceleração)          │
│ Risco residual: AFiR não prova correção semântica; TEE é proprietário│
├──────────────────────────────────────────────────────────────────────┤
│ L1 — DISTRIBUIÇÃO E CONSENSO                                         │
│ Petals (camadas em enxame) · Hyperspace (Mysticeti DAG)             │
│ OmniNode (BLAKE3 → CIDv1, SUM Chain)                                 │
│ Risco residual: nós maliciosos com output adulterado                 │
└──────────────────────────────────────────────────────────────────────┘
```

---

## 2. Camada L1 — Distribuição e Consenso

* **OmniNode**: Pesos GGUF e tensores são particionados em blocos de transformers (*transformer block chunking*). O digest criptográfico BLAKE3 de cada chunk é codificado no formato IPFS CIDv1 (`bafk...`). Qualquer nó da rede valida deterministicamente se o chunk recebido corresponde ao CID declarado.
* **Hyperspace**: Consenso Mysticeti DAG de alta vazão para canais de liquidação e micro-pagamentos de streaming de inferência entre nós.
* **Petals Relay**: Mecanismo de fallback para sharding de camadas quando modelos extensos (ex.: 70B+) não cabem na VRAM de um único executor.

---

## 3. Camada L2 — Proveniência da Inferência (AFiR & SPICE)

* **AFiR (Atomic Fragment Inference Receipt)**: Perfil TEE-free compatível com a árvore append-only do draft **IETF SPICE**.
  * Algoritmo de Assinatura: **ML-DSA-65** (FIPS 204).
  * Tamanho do Recibo: exatamente **384 bytes**.
  * Overhead Computacional: **~0.785 ms** por fragmento de inferência.
  * O recibo AFiR garante a proveniência e que a instância assinante processou o fragmento associado ao `inference_root`, dispensando enclaves TEE ou a sobrecarga severa de ZKML (~1000×).

---

## 4. Camada L3 — Evidência Local (ArkheOS Mobile)

O ArkheOS Mobile ancora a execução móvel em um ambiente de hardware seguro (GrapheneOS / Titan M2 StrongBox), garantindo que as ações e inferências de agentes inteligentes locais não possam ser reescritas retroativamente.

### 4.1 Estrutura do MobileWER (Work Evidence Record)

Domínio canónico: `AKG:MOBILE:ACT:v1\0`.

```rust
use serde::{Deserialize, Serialize};

pub type Digest32 = [u8; 32];

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
    /// H(attestation_blob || first_wer_nonce || app_measurement)
    pub device_attestation_hash: Digest32,
    pub app_id: Digest32,
    pub action: MobileAction,
    pub input_digest: Digest32,
    pub output_digest: Digest32,
    pub policy_digest: Digest32,
    pub performed_at: u64,
    pub evidence_refs: Vec<Digest32>,
}
```

### 4.2 MMR Local Selado e Assinatura Híbrida Distribuída

O StrongBox (ex.: Titan M2 no Android 17) suporta nativamente chaves **ML-DSA** (ML-DSA-65 e ML-DSA-87), mas não oferece suporte a **Ed25519** (restringido ao TEE/software).

A assinatura do checkpoint da MMR (*Merkle Mountain Range*) local adota um **envelope híbrido distribuído**:
1. Assinatura PQC **ML-DSA-65** efetuada no hardware isolado pelo **StrongBox**.
2. Assinatura clássica **Ed25519** efetuada no ambiente **TEE/Software**.

```rust
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HybridCheckpointEnvelope {
    pub mmr_size: u64,
    pub mmr_root: Digest32,
    pub timestamp: u64,
    /// Assinatura pós-quântica emitida dentro do StrongBox (Titan M2)
    pub strongbox_pqc_sig: Vec<u8>,
    /// Assinatura clássica Ed25519 gerada em TEE/Software
    pub tee_ed25519_sig: [u8; 64],
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct OfflineVerificationBundle {
    pub wer: MobileWER,
    pub mmr_inclusion_proof: Vec<Digest32>,
    pub checkpoint: HybridCheckpointEnvelope,
    pub titan_attestation_blob: Vec<u8>,
    pub first_wer_nonce: [u8; 32],
    pub app_measurement: Digest32,
}
```

### 4.3 Algoritmo de Verificação Offline

O verificador de terceiros (`arkhe-verify.wasm` ou verificador offline local) executa a validação sem comunicação de rede:

1. **Validação do Binding de Hardware**:
   $$\text{expected\_hash} = \text{BLAKE3}(\text{titan\_attestation\_blob} \mathbin{\Vert} \text{first\_wer\_nonce} \mathbin{\Vert} \text{app\_measurement})$$
   Verifica-se se $\text{wer.device\_attestation\_hash} == \text{expected\_hash}$.
2. **Inclusão na MMR**:
   Verifica-se a prova de inclusão do `BLAKE3(wer)` em relação ao `mmr_root` presente no `checkpoint`.
3. **Validação das Assinaturas do Checkpoint**:
   Valida-se a assinatura `strongbox_pqc_sig` com a chave pública ML-DSA-65 e a assinatura `tee_ed25519_sig` com a chave pública Ed25519 do dispositivo.
4. **Verificação de Consistência Histórica**:
   Ao comparar dois checkpoints subsequentes via prova de consistência de MMR ($O(\log N)$), qualquer tentativa de reescrever registros passados resulta em falha imediata.

---

## 5. Camada L4 — Governança e Limites Declarados

* **Consenso OpenCLAW-P2P**: BFT com quórum qualificado para propostas de auto-melhoria de agentes e calibração de modelos locais. Finalização de 95% das propostas em 60s sob até 20% de nós bizantinos (em redes de escala até 20–30 nós).
* **Fronteira Explícita de Sybil**: A resistência contra ataques Sybil não pode ser resolvida exclusivamente por regras de saldo ou voto quadrático em ambientes permissionless; exige mecanismos externos como *proof-of-personhood* ou chaves de identidade ancoradas em governança institucional.
