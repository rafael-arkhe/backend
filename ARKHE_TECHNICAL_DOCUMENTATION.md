# Arkhe — Documentação Técnica Completa

**Versão:** 1.0 (canônica)
**Data:** 2026-11-06
**Autor:** Rafael Oliveira — ORCID 0009-0005-2697-4668
**Status:** `τ: PROPOSAL` consolidado; núcleo criptográfico `τ: LEMMA`
**Classificação epistêmica:** framework τ(א) — toda afirmação classificada

> **Aviso de escopo:** Este documento é o ponto de entrada canônico para o Arkhe. Substitui múltiplas peças fragmentadas (proposta de hackathon, análises de parceria, análise geopolítica, errata do conceito Arkhe Cloud) por uma narrativa única e auto-contida. Para especificações detalhadas de componentes (WormGraph, OVB, Vault, Recovery, Genesis, Situated Agency, ZK Identity), ver `docs/INDEX.md` para navegação pelo corpus completo.

> **Aviso epistêmico:** Cada afirmação técnica carrega classificação τ: `LEMMA` (segue de fonte primária verificada, P>0.95), `PROOF` (estabelecido por argumento, P>0.90), `PROPOSAL` (decisão normativa, P variável), `CONJECTURE` (hipótese, P 0.50-0.80), `REFUTED` (explicitamente não garantido), `PENDING` (aguardando validação). A regra do corpus: "Ausência de veredito é tratada como recusa, nunca aprovação."

---

## Sumário

0. [TL;DR — uma página](#0-tldr--uma-página)
1. [Tese e fronteira epistêmica](#1-tese-e-fronteira-epistêmica)
2. [Pilhas de verificação L1-L4](#2-pilhas-de-verificação-l1-l4)
3. [Primitivas criptográficas canônicas](#3-primitivas-criptográficas-canônicas)
4. [Catálogo de serviços](#4-catálogo-de-serviços)
5. [Implementação de referência](#5-implementação-de-referência)
6. [Estratégia de produto — Evidence Platform](#6-estratégia-de-produto--evidence-platform)
7. [Parcerias e ecossistema](#7-parcerias-e-ecossistema)
8. [Justificativa geopolítica](#8-justificativa-geopolítica)
9. [Concorrência e diferenciação](#9-concorrência-e-diferenciação)
10. [Roadmap por maturidade](#10-roadmap-por-maturidade)
11. [Errata canônica — o que Arkhe NÃO é](#11-errata-canônica--o-que-arkhe-não-é)
12. [Modelo de negócio e tiers](#12-modelo-de-negócio-e-tiers)
13. [Pitch executivo de 30 segundos](#13-pitch-executivo-de-30-segundos)
14. [Apêndice A — Invariantes canônicas](#apêndice-a--invariantes-canônicas)
15. [Apêndice B — Framework τ](#apêndice-b--framework-τ)
16. [Apêndice C — Fontes primárias](#apêndice-c--fontes-primárias)

---

## 0. TL;DR — uma página

> **Arkhe é uma camada de evidência verificável, pós-quântica, offline. Não move dinheiro, não executa inferência, não governa agentes. Prova o que aconteceu — com criptografia real, verificável por qualquer terceiro, sem depender de provedor, servidor ou chain.**
>
> **Prove, do not promise.**

**Tese central (uma frase):** Arkhe transforma eventos de computação (pagamentos, inferência, deploys) em recibos criptográficos pós-quânticos (ML-DSA-65, FIPS 204), registrados em MMR append-only, ancorados em Bitcoin, verificáveis offline no navegador.

**Fronteira epistêmica canônica:** Arkhe prova **integridade** (saída não foi alterada), **proveniência** (sabe-se qual artefato gerou) e **atestação** (prova-se o ambiente de execução contra uma política). Arkhe **não prova corretude** (que a saída é verdadeira ou livre de alucinação). Esta fronteira é `τ: LEMMA` — consequência direta do que são funções de hash e assinaturas digitais.

**Stack canônica:**
- ML-DSA-65 (FIPS 204, ago 2024) — assinatura pós-quântica (pk 1952 B, sk 4032 B, sig 3309 B)
- BLAKE3 — hash canônico
- CBOR determinístico (RFC 8949 §4.2) — serialização
- MMR (Merkle Mountain Range) — histórico append-only
- Bitcoin OP_RETURN — ancoragem independente verificável
- WASM — verificação no navegador, offline

**Implementação:** 213 testes (161 produção + 52 sandbox), 0 falhas. Crypto real, não mock. WASM compilado. CLI com 15 subcomandos. AFiR slash proof em RISC Zero zkVM verificado na Base Sepolia.

**Posicionamento:** Padrão técnico aberto. A soberania é consequência do design (offline, sem provedor, padrões abertos), não de marketing. Não é projeto de soberania; é projeto de verificação cuja soberania emerge como propriedade.

**Roadmap imediato:** Hack Brasil 2026 (5-7 dias, Track 1 — Evidence Platform com `PaymentEventSource` trait + `SimulatorSource` adapter + Ingestion API + WASM verifier + React UI). Quatro tracks paralelos: ValidaPix, Mt Pelerin + Alchemy ARC Credits, Picnic, GuaraCloud (design-partner).

---

## 1. Tese e fronteira epistêmica

### 1.1 O que Arkhe vende, e apenas uma coisa

Arkhe vende **prova verificável de que uma computação ocorreu como declarada**. Não vende corretude. Não vende verdade. Não vende ausência de alucinação. Esta distinção é a fundação do conceito.

Um recibo criptográfico Arkhe liga cinco campos:

```text
fingerprint_do_modelo  +  hash_da_entrada  +  hash_da_saída  +  assinatura  +  âncora_temporal
```

**O que prova:** este modelo, sobre esta entrada, produziu esta saída, sem adulteração, neste instante, com cadeia de custódia auditável.

**O que não prova:** que a saída é factualmente correta. Alucinação é uma propriedade semântica do conteúdo; a partir de um hash, é indecidível. Um modelo perfeitamente atestado erra com confiança, e o recibo assina o erro com o mesmo rigor com que assinaria o acerto.

> **Classificação τ:** a fronteira integridade ≠ corretude é `τ: LEMMA` — consequência direta do que são funções de hash e assinaturas digitais (P > 0.99). Tudo o que se segue no framework é projetado para ficar do lado provável dessa fronteira.

### 1.2 As três propriedades que Arkhe entrega — e a quarta que não entrega

| Propriedade | Definição | τ | Mecanismo |
|---|---|---|---|
| **Integridade** | A saída não foi alterada entre produção e consumo | LEMMA | BLAKE3 + ML-DSA-65 + MMR append-only |
| **Proveniência** | Sabe-se qual artefato (modelo, dado, binário) gerou a saída, e por qual chave | LEMMA | AFiR receipt com fingerprint binding |
| **Atestação** | Prova-se o ambiente de execução (TEE, GPU em modo confidencial) contra uma política | LEMMA (com TEE) / CONJECTURE (sem TEE) | Token de atestação TEE + recibo |
| **Corretude** | A saída é factualmente verdadeira | `τ: REFUTED` — fora de escopo | Nenhuma primitiva criptográfica a entrega |

### 1.3 Regra do corpus

> **"Ausência de veredito é tratada como recusa, nunca aprovação."** (Whitepaper §1.3)

Esta regra obriga o framework a explicitar o que não prova. Qualquer afirmação técnica sem fonte canônica é `τ: PENDING` até verificação. Qualquer claim retórica sem mecanismo é `τ: REFUTED` como ECHO.

---

## 2. Pilhas de verificação L1-L4

A arquitetura Arkhe organiza-se em quatro pilhas de verificação, cada uma com primitivas canônicas e invariantes explícitas.

### 2.1 L1 — Distribuição e consenso

```text
BLAKE3 → CIDv1 → DHT (Mysticeti DAG) → MMR append-only → Bitcoin OP_RETURN anchor
```

**Primitivas:**
- BLAKE3 — hash canônico (mais rápido que SHA-256, paralelizável, adotado em IETF e RustCrypto)
- CIDv1 — endereçamento por conteúdo (IPLD, IPFS)
- DHT — descoberta P2P (Kademlia-style)
- Mysticeti DAG — consenso por DAG (referência ao Bullshark/Aptos)
- MMR — Merkle Mountain Range append-only (Peter Todd spec)
- Bitcoin OP_RETURN — ancoragem em mainnet (80 bytes úteis, raiz MMR cabe em 32)

**Invariantes canônicas:** I-01 a I-10 (whitepaper §3.5).

**Implementação:** `btc_anchor.rs` (9 testes produção, mainnet headers), `mmr.rs` (3 testes produção, proof round-trip).

**τ:** LEMMA — todas as primitivas são produção em 2026.

### 2.2 L2 — Provenância de inferência

```text
AFiR receipt (perfil 0x01 inference) → ML-DSA-65 signature (3309 B) → MMR leaf
```

**Primitivas:**
- AFiR receipt — recibo de inferência com 174 B payload + 3309 B assinatura
- ML-DSA-65 — FIPS 204, categoria 3 (equivale AES-192 em segurança pós-quântica)
- MMR leaf — inclusão na raiz append-only

**Estrutura do AFiR (perfil 0x01, inference):**

```text
┌─────────────────────────────────────┐
│ AFiR Receipt (perfil 0x01)          │
├─────────────────────────────────────┤
│ modelFingerprint    (BLAKE3)        │
│ inputHash           (BLAKE3)        │
│ outputHash          (BLAKE3)        │
│ timestamp           (Unix seconds)  │
│ teeAttestationToken (optional)      │
│ executionTrace      (optional)      │
│ ─────────────────────────────────── │
│ signature           (ML-DSA-65)     │
│                     3309 bytes      │
└─────────────────────────────────────┘
```

**Invariantes canônicas:** AFiR-related, cobertas por `afir.rs` (4 testes), `afir_anchor.rs` (3), `afir_bundle.rs` (4), `afir_checkpoint.rs` (4), `afir_manifest.rs` (5), `afir_retrieval.rs` (7).

**Implementação:** byte-exact golden vectors, bit-flip rejection, dual-signature ML-DSA-65 + Ed25519 (checkpoint), chunk re-hash, salt blinding (data-fetch), OpenSSL interop.

**τ:** LEMMA — ML-DSA-65 é FIPS 204 (ago 2024), em produção. AFiR é composição engenharia.

### 2.3 L3 — Evidência local

```text
MMR append-only → StrongBox/Titan M2 → OfflineVerificationBundle (OVB)
```

**Primitivas:**
- MMR — histórico append-only (L1)
- StrongBox/Titan M2 — hardware-backed key storage (mobile, futuro)
- OVB — pacote autônomo verificável offline

**Estrutura do OVB:**

```text
demo.ovb (ZIP)
├── manifest.cbor              ← raiz de confiança (I-OVB-08)
├── receipts/
│   ├── 0.cbor
│   ├── 1.cbor
│   └── ...
├── attestations/              ← (vazio para demo)
├── policy.cbor                ← política versionada
├── wasm/
│   └── verify.wasm            ← verificador pinado (I-OVB-11)
└── journal.cbor               ← journal de verificação (I-OVB-13)
```

**Invariantes canônicas:** I-OVB-01 a I-OVB-14. Destaques:
- I-OVB-08: manifesto é a raiz de confiança (não `bundle.cbor`)
- I-OVB-09: `bundleId` é excluído do próprio digest (evita chicken-and-egg)
- I-OVB-10: atestados são incluídos como bytes
- I-OVB-11: digest do WASM é pinado externamente
- I-OVB-12: política é validada antes dos checks condicionais
- I-OVB-13: journal é incluído quando exigido

**Implementação:** `afir_bundle.rs` (4 testes — remoção de receipt detectada).

**τ:** LEMMA para OVB; CONJECTURE para mobile StrongBox (não implementado).

### 2.4 L4 — Governança e incentivos

```text
PoI (Proof-of-Inference) → burn-and-mint → staking/slashing → BFT quorum
```

**Primitivas:**
- PoI — tokenomics baseado em inferência atestada (CONJECTURE)
- Burn-and-mint — modelo econômico (CONJECTURE)
- Staking/slashing — incentivo e punição (parcial via AFiR slash)
- BFT quorum — consenso para canonização (CONJECTURE)

**Implementação existente:** `afir_slash_zk` (Phase 0 completo) — prova de equivocação roda em RISC Zero zkVM, verificada na Base Sepolia (selector 73c457ba, 276k gas, seal Groth16 de 260 bytes).

**τ:** LEMMA para AFiR slash (zkVM em produção); CONJECTURE para PoI tokenomics completo.

### 2.5 WormGraph — grafo causal append-only

```text
Operational Surface (CRDT, convergente) ⟷ Evidential Surface (append-only, fail-closed)
```

O WormGraph é o grafo de evidência que conecta eventos. Tem duas superfícies estruturais separadas por uma fronteira constitucional (I-WG-01):

- **Operacional** — CRDT, permite convergência entre nós sem ordem total. Mutável.
- **Evidencial** — append-only, fail-closed. Imutável. Esta é a superfície ancorada em Bitcoin.

**Invariantes canônicas:** I-WG-01 a I-WG-08. Key rotation: ML-DSA-65 canônico; SLH-DSA fallback por decisão constitucional (I-WG-06).

**EventTypes canônicos:** ExperimentRoot, IndependentEvaluation, Decision, Rejection, Rollback, Revocation, InstitutionalCommitment, AgiStatusProposal, AgiStatusDecision, RecoveryRequested.

**τ:** LEMMA para MMR no crate; CONJECTURE para grafo completo (sem implementação P2P ainda).

### 2.6 Composição confidencial (I-PC1 a I-PC7)

```text
Octra/FHE → composição privada de computações atestadas
```

**τ:** `τ: PENDING` — pesquisa aberta, não implementado. Não prometer a cliente.

---

## 3. Primitivas criptográficas canônicas

### 3.1 ML-DSA-65 (FIPS 204)

**Status:** `τ: LEMMA` — NIST FIPS 204 finalizado em 13 de agosto de 2024.

**Parâmetros canônicos (NIST FIPS 204):**

| Parameter set | pk (bytes) | sk (bytes) | sig (bytes) | Categoria NIST |
|---|---|---|---|---|
| ML-DSA-44 | 1312 | 2560 | 2420 | 2 (equivale AES-128) |
| **ML-DSA-65** | **1952** | **4032** | **3309** | **3 (equivale AES-192)** |
| ML-DSA-87 | 2592 | 4896 | 4627 | 5 (equivale AES-256) |

**Implementação Arkhe:** crate `fips204` v0.5 (Rust). Sandbox `arkhe-verification/src/crypto.rs` usa `ml_dsa_65::KeyPair` real (não-determinístico, exige geração randomizada).

**Comparação com ECDSA:** ML-DSA-65 tem assinatura ~50x maior que ECDSA (3309 B vs 64 B). Para audit records em database, 1M de ações assinadas adiciona ~3.2 GB. Inflação de certificado (pk 1952 B vs 64 B) é desafio real de deployment.

**Por que ML-DSA-65 e não ML-DSA-44 ou 87?**

- ML-DSA-44 (categoria 2) — considerado abaixo do padrão para novos sistemas em 2026
- ML-DSA-65 (categoria 3) — equilíbrio entre segurança e tamanho; alinhado com tendência NIST para sistemas novos
- ML-DSA-87 (categoria 5) — overkill para maioria dos casos; inflação de tamanho proibitiva

### 3.2 BLAKE3

**Status:** `τ: LEMMA` — produção, IETF draft, RustCrypto.

**Propriedades:**
- Hash de 256 bits (32 bytes)
- Paralelizável (até 8x mais rápido que SHA-256 em multi-core)
- Modos: hash, keyed_hash, derive_key
- Árvore Merkle interna (suporta verificação incremental)

**Implementação:** crate `blake3` v1.5. Usado em todo o corpus para: hash canônico, MMR, pseudonymização salted, manifest digests.

### 3.3 CBOR determinístico (RFC 8949 §4.2)

**Status:** `τ: LEMMA` — IETF RFC 8949 (out 2020).

**Regras canônicas:**
- Chaves de mapa ordenadas lexicograficamente (byte-by-byte)
- Inteiros na forma mais compacta
- Strings no menor comprimento
- Sem duplicações de chave

**Implementação:** `arkhe-verification/src/cbor.rs` — CBOR determinístico com chaves ordenadas. Teste `cbor.rs` valida com golden GGUF vectors.

### 3.4 MMR (Merkle Mountain Range)

**Status:** `τ: LEMMA` — Peter Todd spec, em produção em OpenTimestamps desde 2016.

**Propriedades:**
- Append-only (sem mutação de folhas existentes)
- Bagging de picos (não linear hash) — cada pico é uma sub-árvore completa
- Inclusion proof O(log n)
- Consistency proof entre tamanhos

**Implementação:** `mmr.rs` (3 testes produção — proof round-trip). Sandbox tem MMR linear a ser migrado para bagging real (Track 1 build spec §7).

### 3.5 Bitcoin OP_RETURN

**Status:** `τ: LEMMA` — Bitcoin mainnet, em produção desde 2014.

**Formato ARKV1 (Vault):**

```text
OP_RETURN ARKV1 (77 bytes)
├── Magic (4 bytes): "ARK1"
├── Commitment (32 bytes): BLAKE3 hash
├── Round (8 bytes): drand quicknet round number
├── Chain hash (32 bytes): drand chain hash
└── Reserved (1 byte)
```

**Implementação:** `btc_anchor.rs` (9 testes produção — mainnet headers, coinbase, confirmações).

### 3.6 AFiR receipt

**Status:** `τ: LEMMA` — composição das primitivas acima.

**Perfis canônicos:**

| Perfil | Uso | Implementação |
|---|---|---|
| 0x01 | Inference (modelo ML) | `afir.rs` (4 testes), `afir_anchor.rs` (3), `afir_checkpoint.rs` (4) |
| 0x03 | Data-fetch (webhook, API) | `afir_retrieval.rs` (7 testes — salt blinding, OpenSSL interop) |

### 3.7 TEE (Trusted Execution Environment) — para AIR/ACC

**Status:** `τ: LEMMA` para CPU TEE; `τ: LEMMA` para GPU TEE (H100+).

**CPU TEE:**
- AMD SEV-SNP — CVM em produção desde 2020+
- Intel TDX — early access 2022, GA 2023+

**GPU TEE:**
- NVIDIA H100 PCIe/HGX — CC-On mode cifra memória on-card via encrypted bounce buffer
- Requisitos: VBIOS 96.00.5E.00.01+, driver R535.86+, CPU CVM (SEV-SNP ou TDX)
- H200, B200, B300 — suportam CC com NVLink encryption entre GPUs
- GPUs de consumo (RTX, etc.) — não suportam CC em produção

**Atestação independente:**
- Intel Tiber Trust Authority (ex-Project Amber) — multi-cloud, multi-TEE
- Fortanix CCM — workload registration + attestation
- Self-hosted verifier — possível com código open source

### 3.8 zkVM (RISC Zero)

**Status:** `τ: LEMMA` — em produção para slash de equivocação.

**Implementação AFiR slash (Phase 0 completo):**
- zkVM: RISC Zero
- Image ID: e12f6434…
- Ciclos: 19.061.567
- STARK prove: ~1h53 (12 vCPU, 2.7 GB RAM)
- Groth16 compress: 122s, seal de 260 bytes
- On-chain: Base Sepolia, selector 73c457ba, 276k gas
- Verificador: v3.0.0, ativo, não pausado

### 3.9 PoRep/PoSt (Filecoin)

**Status:** `τ: LEMMA` — produção desde 2020.

- PoRep (Proof of Replication) — Filecoin FIP-0064 com zk-SNARK (2023)
- PoSt (Proof of Spacetime) — WindowPoSt (24h) + WinningPoSt (por bloco)
- Auditoria por período, não em tempo real contínuo

---

## 4. Catálogo de serviços

Arkhe oferece cinco serviços, cada um ancorado em primitiva madura. Para cada: o que entrega, o que prova, o que não prova, e de que depende. O que "não prova" é escrito na ficha do produto — é o antídoto contra a venda de corretude.

### 4.1 Recibo AFiR (o produto-chave)

**O que entrega:** Artefato transversal que liga fingerprint do modelo/artefato + hash de entrada + hash de saída + assinatura ML-DSA-65 + âncora temporal. Compacto (ordem de 5-8 KB com ML-DSA-65).

**O que prova:** Cadeia de custódia auditável de ponta a ponta — qual artefato gerou qual saída, sem adulteração, em qual instante.

**O que não prova:** Nada sobre o mérito do que foi computado — só sobre sua integridade e origem.

**Depende de:** ML-DSA-65, BLAKE3, CBOR, MMR, ancoragem Bitcoin.

**τ:** LEMMA — já implementado no corpus.

### 4.2 AIR — Attested Inference Receipt

**O que entrega:** Endpoint de inferência que roda modelo em TEE e devolve saída + recibo assinado (fingerprint do modelo, hash entrada, hash saída, trace assinado, token de atestação TEE).

**O que prova:** Proveniência e integridade da inferência — este modelo gerou esta saída, sem troca silenciosa de pesos, em ambiente atestado.

**O que não prova:** Que a saída é factualmente correta ou livre de alucinação (ver §1). O recibo é a cadeia de custódia, não um selo de verdade.

**Depende de:** GPU TEE (H100+ CC-On); para modelos grandes, TEE + trace assinado (zkML não escala aqui em 2026).

**τ:** PROPOSAL — extensão do AFiR 0x01 com campos `teeAttestationToken`, `executionTrace`, `teeAttestationVerified`. A ser implementado pós-Hack Brasil.

### 4.3 ACC — Arkhe Confidential Compute

**O que entrega:** VM confidencial (CPU TEE + GPU TEE opcional) com token de atestação verificável por terceiro independente.

**O que prova:** Que o workload rodou num ambiente íntegro, não adulterado pelo operador da nuvem, contra uma política declarada.

**O que não prova:** Que o código do cliente está correto ou é seguro; o TEE isola, não audita a lógica.

**Depende de:** SEV-SNP/TDX + (para GPU) H100+ com VBIOS CC; serviço de atestação independente (Intel Tiber Trust Authority ou self-hosted).

**τ:** PROPOSAL — adiar pós-Hack Brasil. Exige parceria com provedor de TEE (Azure CC, GCP Confidential VMs, ou on-prem).

### 4.4 ACS — Arkhe Content-addressed Storage

**O que entrega:** Armazenamento com ID por conteúdo (BLAKE3/CIDv1), shards cifrados, prova periódica de continuidade (PoRep/PoSt).

**O que prova:** Que o arquivo recuperado é bit-a-bit o que foi gravado, e que esteve continuamente disponível no período.

**O que não prova:** Confidencialidade absoluta se a chave vazar; nem disponibilidade instantânea entre janelas de prova.

**Depende de:** Esquema tipo PoRep/PoSt (Filecoin); âncora da raiz num ledger imutável.

**τ:** CONJECTURE — adiar. Arkhe é camada de evidência, não storage. ACS seria produto distinto. Parceria com Filecoin ou IPFS pinning services é alternativa a construir ACS próprio.

### 4.5 AID — Arkhe Identity DID + ML-DSA-65

**O que entrega:** Identificadores descentralizados (DID) + chaves ML-DSA-65, com wrappers para e-CNPJ/ICP-Brasil, passkeys e OAuth.

**O que prova:** Autenticidade de origem de agentes e artefatos, resistente a ataque quântico futuro.

**O que não prova:** Que o detentor da chave é confiável; prova posse da chave, não virtude do agente.

**Depende de:** Gestão de ciclo de vida de chave e da inflação de certificado (chave pública 1952 B).

**τ:** PROPOSAL — extensão da spec ZK identity existente. Definir `did:arkhe:...` method. Wrapper passkey (WebAuthn → ML-DSA-65 delegation) é viável. Wrapper ICP-Brasil exige pesquisa regulatória (Lei 14.063/2020 exige ICP-Brasil para atos formais; ML-DSA-65 não é ICP-Brasil, então wrappers são complementares, não substitutivos).

---

## 5. Implementação de referência

### 5.1 Crate de produção — `arkhe-verify-core`

| Campo | Valor |
|---|---|
| Local | `verification/crates/arkhe-verify-core/` |
| Testes | 161 (0 falhas) |
| Crypto | ML-DSA-65 real (fips204 v0.4.6), Ed25519 (ed25519-dalek), BLAKE3 |
| WASM | ✅ Compilado (wasm32-unknown-unknown) |
| CLI | 15 subcomandos |
| CI | Podman isolado, sem rede, Rust 1.96.0 pinned |

**Suítes de teste (resumo):**

| Suite | Testes | O que verifica |
|---|---|---|
| afir.rs | 4 | Payload byte-exact, bit-flip rejection |
| afir_anchor.rs | 3 | Equivocation como fraud proof |
| afir_bundle.rs | 4 | Remoção de receipt detectada |
| afir_checkpoint.rs | 4 | Dual-signature (ML-DSA-65 + Ed25519) |
| afir_manifest.rs | 5 | Chunk re-hash, pruned node |
| afir_retrieval.rs | 7 | Salt blinding, OpenSSL interop |
| arc_reconciliation.rs | 14 | Swapped invoice, forged decision |
| btc_anchor.rs | 9 | Mainnet headers, coinbase, confirmações |
| c2pa_signature.rs | 9 | Rogue key, identity binding |
| cbor.rs | 1 | GGUF golden CBOR |
| custody.rs | 11 | 100-transfer chain, every-byte-change |
| discovery.rs | 5 | TypeScript vector parity |
| entitlement.rs | 4 | Signed token, test mode |
| gguf_supply_chain.rs | 11 | Tensor validation, RFC 6962 |
| golden_contract.rs | 1 | Committed vectors match |
| gov_lifecycle.rs | 4 | Qualification, replay prevention |
| gov_offline.rs | 2 | Synthetic credentials |
| hash.rs | 1 | BLAKE3 known answers |
| interop.rs | 11 | VAOL, Hive, cosigning, fork detection |
| memo_reconciliation.rs | 11 | Real memo decode, ABI |
| mmr.rs | 3 | Proof round-trip |
| policy_adversarial.rs | 5 | Digest replay, delimiter |
| policy_compatibility.rs | 4 | Migration, expansion |
| rbb_protocol.rs | 7 | Python ABI, keccak match |
| absence.rs | 4 | Window, auditor record |

**CLI subcomandos:** verify-release, verify-custody, verify-absence, inspect-gguf, blake3, verify-gguf, verify-gguf-artifact, verify-c2pa, verify-hive, verify-vaol, cosign-vaol, verify-cosignatures, license, + 2 internos.

### 5.2 Sandbox de referência — `arkhe-verification`

| Campo | Valor |
|---|---|
| Local | `download/arkhe-verification/` |
| Testes | 52 (0 falhas) |
| Crypto | ML-DSA-65 real (fips204 v0.5) |
| Deps | blake3, fips204 |

Implementa RFC-0002 (VerificationRecord, IntegrationAssessment, BranchIntegrationState) e RFC-0003 (NetworkPathAttestation, EnvironmentAttestation, EntityAttestation) com tipos canônicos.

**Suítes:**

| Suite | Testes | O que verifica |
|---|---|---|
| atb.rs | 14 | ATB (Attestation Binding) + ELG (Eligibility) |
| ind.rs | 5 | IND (Independência 4D) |
| con.rs | 8 | CON (Contestação) |
| exp.rs | 9 | EXP (Expiração) |
| bis.rs | 7 | BIS (Branch Integration State) |
| binding.rs + cbor.rs + crypto.rs | 9 | CBOR determinístico, ML-DSA-65 round-trip |

### 5.3 AFiR slash proof (zkVM)

| Campo | Valor |
|---|---|
| zkVM | RISC Zero |
| Image ID | e12f6434… |
| Ciclos | 19.061.567 |
| STARK prove | ~1h53 (12 vCPU, 2.7 GB RAM) |
| Groth16 compress | 122s, seal de 260 bytes |
| On-chain | Base Sepolia, selector 73c457ba, 276k gas |
| Verificador | v3.0.0, ativo, não pausado |

Prova de equivocação roda no zkVM e é verificada on-chain. Par honesto é recusado (`NotEquivocation`).

### 5.4 Estado de implementação por componente

| Componente | Estado | Testes |
|---|---|---|
| ML-DSA-65 (FIPS 204) | ✅ Real (fips204) | sign/verify roundtrip |
| Ed25519 | ✅ Real (ed25519-dalek) | checkpoint dual-sig |
| BLAKE3 | ✅ Real | known answers |
| CBOR determinístico | ✅ Real | GGUF golden |
| VerificationRecord binding | ✅ Real | I-VR-B1..B7 |
| Independência 4D | ✅ Real | I-VR-I1..I5 |
| Contestação | ✅ Real | I-VR-C1..C9 |
| Expiração | ✅ Real | I-VR-E1..E5 |
| Branch state | ✅ Real | I-BIS-01..07 |
| Attestations (3 tipos) | ✅ Real | I-TAP-NB/EB/XB |
| MMR | ✅ Real | proof round-trip |
| AFiR receipt (inference) | ✅ Real | byte-exact, bit-flip |
| AFiR receipt (data-fetch) | ✅ Real | salt blinding, OpenSSL |
| Bitcoin anchor | ✅ Real | mainnet headers |
| RBB protocol | ✅ Real | Python ABI match |
| C2PA signature | ✅ Real | rogue key, identity |
| GGUF supply chain | ✅ Real | tensor validation |
| Custody chain | ✅ Real | 100-transfer |
| VAOL witness | ✅ Real | cosigning, fork |
| Hive receipt | ✅ Real | typed-signer |
| WASM compilation | ✅ Real | binding parity |
| AFiR slash proof (zkVM) | ✅ Phase 0 | Base Sepolia, 276k gas |
| Rede P2P (libp2p) | ❌ TODO | 4-8 semanas |
| Distribuição de pesos | ❌ TODO | 3-6 semanas |
| SDK de interceptação | ❌ TODO | 2-4 semanas |
| CAL adapters (4) | ❌ TODO | 4-8 semanas |
| Mobile evidence (StrongBox) | ❌ TODO | 8-12 semanas |
| Constitution store + BFT | ❌ TODO | 4-6 semanas |
| Account recovery | ❌ TODO | 2-3 semanas |
| Genesis ceremony | ❌ TODO | 3-4 semanas |
| OVB completo | ⚠️ Parcial | 2-3 semanas |
| SAA conformance (T01..T15) | ❌ TODO | 3-4 semanas |
| PaymentEventSource trait | ❌ TODO | 1 dia (Track 1) |
| AIR service (TEE binding) | ❌ TODO | pós-Hack Brasil |
| AID service (DID + ML-DSA-65) | ❌ TODO | pós-Hack Brasil |
| Composição confidencial | ❌ TODO | 6-12 meses |

---

## 6. Estratégia de produto — Evidence Platform

### 6.1 Por que "Evidence Platform" e não "Blockchain para pagamentos"

A conversa inicial considerou posicionar o Arkhe como "camada superior" acima de PSPs. A pesquisa revela que este framing é impreciso:

- **"Superior" implica hierarquia.** Arkhe não orquestra, não autoriza, não intercepta.
- **"Ortogonal" é correto.** Arkhe intercepta eventos de qualquer fonte, gera recibo pós-quântico, torna verificável offline.

**Framing canônico:**

> **Arkhe é uma camada de evidência ortogonal: intercepta eventos de qualquer fonte (ValidaPix, Mt Pelerin, Dfns, Alchemy, Picnic, GuaraCloud, ou um simulador), gera um recibo criptográfico pós-quântico (ML-DSA-65), e o torna verificável offline por qualquer terceiro. Não substitui o gateway; torna o evento verificável de forma portátil.**

### 6.2 Trait `PaymentEventSource` — ponto de unificação

```rust
pub trait PaymentEventSource: Send + Sync {
    fn provider_id(&self) -> &'static str;
    fn provider_jurisdiction(&self) -> &'static str;
    fn kyc_tier(&self) -> KycTier;
    fn verify_authenticity(&self, raw_payload: &[u8], headers: &HashMap<String, String>) -> Result<(), AuthError>;
    fn extract_event(&self, raw_payload: &[u8]) -> Result<PaymentEvent, ExtractError>;
    fn reconcile(&self, event: &PaymentEvent) -> Result<ReconciliationStatus, ReconError>;
}
```

Qualquer PSP, ramp, validador, ou simulador pode implementar. O trait é genérico em `chain`, `currency`, `payment_method`, `processing_partner`. Implementações existentes:

| Adapter | Source type | Caso | Status |
|---|---|---|---|
| `SimulatorSource` | In-process event emitter | Demo hackathon, testes | Track 1 (5-7 dias) |
| `ValidaPixSource` | Webhook HTTP + HMAC | Pix doméstico Brasil | Track 2 (4-6 semanas) |
| `MtPelerinSource` | Webhook HTTP + widget | Ramp cross-border CH | Track 3 (4-6 semanas) |
| `AlchemyWebhookSource` | Webhook HTTP + Alchemy | On-chain settlement | Track 3 (com Mt Pelerin) |
| `DfnsTransferSource` | Dfns Transfer API webhook | Custódia Dfns | Track 3 (com Mt Pelerin) |
| `PicnicSource` | Multi-rail (Pix+ACH+SEPA+on-chain+Visa) | Conta self-custodial BR | Track 4 (4-6 semanas) |
| `GuaraCloudSource` | Build/deploy events + audit logs | PaaS BR proveniência | Track 5 (design-partner) |
| `ArcPaymentEventSource` | On-chain transfers em Arc L1 | Arc mainnet (Elysium) | Após ARC Credits |

### 6.3 Demo de 5 minutos (Hack Brasil 2026)

```text
1. Simular pagamento Pix de R$ 1.000
   → simulator emite webhook com HMAC
   → ingestion API valida HMAC
   → pseudonimiza endToEndId e txid (salted BLAKE3)
   → canonicaliza em CBOR
   → assina com ML-DSA-65 (3.309 bytes)
   → appenda no MMR

2. Mostrar receipt
   → "Pix validado às 14:32; R$ 1.000; hash pseudonimizado"

3. Simular split (10% para afiliado, R$ 100)
   → novo receipt com política versionada

4. Exportar OfflineVerificationBundle (.ovb ZIP)
   → manifest.cbor + receipts/ + policy.cbor + wasm/verify.wasm + manifest.sig

5. Abrir verificador no navegador (localhost:5173)
   → upload do bundle
   → arkhe-verify.wasm verifica 10 fases / 44 passos:
     • ZIP parse → manifest load → manifest signature → bundle ID
     • file integrity → WASM digest pinned → policy → receipts → MMR → verdict
   → resultado: PASS

6. Tentar alterar um valor no histórico (tamper)
   → MMR consistency proof falha
   → "Histórico alterado detectado" (fase 5: file integrity)

7. Closing
   → "Qualquer gateway pode pluguar via trait PaymentEventSource.
      O Arkhe torna cada pagamento verificável. Prove, do not promise."
```

### 6.4 Stack técnica da Evidence Platform

```text
┌──────────────────────────────────────────────────────────────────────────┐
│                         DEMO RUNTIME                                      │
│                                                                          │
│  ┌────────────────┐      ┌──────────────────┐      ┌──────────────────┐ │
│  │  Simulator CLI │ ───▶ │  Ingestion API   │ ───▶ │  Evidence Core   │ │
│  │  (Rust binary) │      │  (Node + Express)│      │  (Rust crate)    │ │
│  │                │      │                  │      │                  │ │
│  │  • Gera event  │      │  • POST /webhook │      │  • Verify HMAC   │ │
│  │  • Assina HMAC │      │  • HMAC verify   │ ───▶ │  • Pseudonym     │ │
│  │  • POST webhook│      │  • Idempotency   │      │  • Canonical CBOR│ │
│  └────────────────┘      │  • Dedup         │      │  • ML-DSA-65 sig │ │
│                          │  • Forward to    │      │  • MMR append    │ │
│                          │    core          │      │  • OVB export    │ │
│                          └──────────────────┘      └────────┬─────────┘ │
│                                                               │           │
│                                                               ▼           │
│                                                    ┌──────────────────┐   │
│                                                    │  Bundle (.ovb)   │   │
│                                                    │  • manifest.cbor │   │
│                                                    │  • receipts/*.cbor│  │
│                                                    │  • wasm/verify.wasm│ │
│                                                    │  • policy.cbor   │   │
│                                                    └────────┬─────────┘   │
│                                                             │             │
│                                                             ▼             │
│  ┌──────────────────────────────────────────────────────────────────┐   │
│  │  Verifier Web (React + Vite + WASM em Web Worker)                │   │
│  │  • Upload → verify → PASS/FAIL + receipt details + MMR proof     │   │
│  │  • Modo "tamper": edita byte, re-verifica, mostra FAIL           │   │
│  └───────────────────────────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 7. Parcerias e ecossistema

### 7.1 Estratégia de cinco tracks paralelos

A estratégia evoluiu de "serializada" → "quatro tracks paralelos" → **cinco tracks paralelos** (com GuaraCloud como quinto track):

| Track | Parceiro | Foco | Custo | τ |
|---|---|---|---|---|
| 1 | (simulador) | Hack Brasil 2026 demo | 5-7 dias | LEMMA |
| 2 | ValidaPix | Chargeback Evidence Pack (Pix BR) | Negociação comercial | PROPOSAL |
| 3 | Mt Pelerin + Alchemy ARC | Cross-border B2B (CH/UE/BR) + $10K credits | Zero (email) | PROPOSAL |
| 4 | Picnic | Conta multi-moeda self-custodial BR | Zero (LinkedIn) | CONJECTURE |
| 5 | GuaraCloud | PaaS BR como design-partner Sovereign Attestation | Co-desenvolvimento | CONJECTURE |

### 7.2 ValidaPix — Track 2

| Campo | Valor |
|---|---|
| Tipo | Validação Pix (BR) |
| Status | Pesquisa completa (454 linhas) |
| Próximo passo | Fase 0: acesso à doc técnica |

**Descobertas materiais:**
1. "Split payment" é termo regulatório ocupado no Brasil (LC 214/2025) — usar "Repasse Auditável"
2. ValidaPix valida via Open Finance Brasil (FAPI Security Profile)
3. STJ tem jurisprudência ativa sobre chargeback — "Dispute Evidence Pack" > "Split Assurance"

**Recomendação:** Começar pela ValidaPix (não ValidaPay). Chargeback Evidence Pack como MVP (não Split Assurance). Adapter gateway-agnóstico desde o início.

### 7.3 Mt Pelerin — Track 3

| Campo | Valor |
|---|---|
| Tipo | Ramp on/off (CH) |
| Status | Pesquisa completa v0.2.0 (650 linhas) com fatos canônicos |
| Próximo passo | Email `hello@mtpelerin.com` em paralelo |

**Descobertas canônicas (v0.2.0):**
1. **Integração é gratuita e go-live é trivial** — zero setup, zero recurring, email + review + key
2. **Mt Pelerin NÃO tem autorização MiCA** — UE apenas por "reverse solicitation"
3. **BRL não suportado** — 18 fiats (AUD/CAD/CHF/CZK/DKK/EUR/GBP/HKD/HUF/JPY/MXN/NOK/NZD/PLN/SEK/SGD/USD/ZAR)
4. **16 chains suportadas** (Bitcoin, Lightning, Ethereum, Arbitrum, Optimism, Base, BNB Chain, Avalanche, Polygon, Gnosis, Tezos, Celo, Rootstock, Sonic, Tempo, zkSync Era)
5. **Regulação:** financial intermediary sob SO-FIT SRO (auditada anualmente), FINMA supervisiona indiretamente
6. **Comissão:** 0.6%-1.3% bank transfer (0% primeiros CHF 500/ano), 2.5%-3.8% card, 0.3% swap
7. **Tração:** 7+ anos, 150k+ users, $1B+ processado, 1539 reviews 5★ Trustpilot

### 7.4 Alchemy ARC Credits — Track 3 (paralelo)

| Campo | Valor |
|---|---|
| Tipo | Programa de credits $10K para times em Arc L1 |
| Status | Análise completa (285 linhas) |
| Próximo passo | Aplicar via formulário web em paralelo |

**Oferta canônica:** até US$ 10.000 em credits Alchemy para times construindo em Arc L1. 90 dias de avaliação, sem lock-in, sem APIs proprietárias. Aplicação web, 5 dias úteis para resposta.

**Encaixe Arkhe:**
1. Cobrir custos de RPC + Webhooks durante Hack Brasil e 6-12 meses seguintes (~$650 estimado para 6 meses; sobra ~$9.350)
2. Sinal de credibilidade — due diligence da Alchemy
3. Adapter Arc-native (`ArcPaymentEventSource`) demonstra portabilidade do trait

### 7.5 Picnic — Track 4

| Campo | Valor |
|---|---|
| Tipo | Conta global self-custodial (BR/BVI) |
| Status | Pesquisa completa (570 linhas) |
| Próximo passo | Validar tração; se material, abordar João Ferreira via LinkedIn |

**Descobertas materiais:**
1. **Self-custody não-negociável** — smart wallet on-chain com chaves exportáveis
2. **Conta multi-moeda com BRL nativo** — único parceiro que cobre BRL (Avenia/BACEN) + USD (Noah/FinCEN) + EUR (SEPA) + crypto + Visa em uma única conta
3. **Cartão Visa sem IOF BR** — via Gnosis Pay/Monavate, 180+ países
4. **Modelo regulatório híbrido** — holding BVI; parceiros regulados por jurisdição
5. **Brasileiros, building in public** — fundada 2021 por João Ferreira e Pedro Brandão
6. **Yield DeFi integrado** — Caixinha (Nexus Mutual/OpenCover), Renda Fácil, Savings
7. **Adapter mais abrangente** — `PicnicSource` cobre 5+ rails em uma única implementação

### 7.6 GuaraCloud — Track 5 (design-partner Sovereign Attestation)

| Campo | Valor |
|---|---|
| Tipo | PaaS brasileiro (design-partner) |
| Status | Análise de encaixe completa |
| Próximo passo | Propor piloto do Vetor 1 (recibo de proveniência de deploy) |

**Encaixe estrutural:** GuaraCloud e Arkhe são complementares, não competitivos. GuaraCloud é PaaS — resolve *onde o código roda*. Arkhe resolve *como provar o que rodou*. São camadas distintas da mesma pilha.

**Ressalva dura:** A documentação da GuaraCloud não menciona TEE, SEV-SNP/TDX nem GPU confidencial. Logo, os serviços Arkhe que dependem de execução confidencial — **ACC (compute atestado)** e **AIR (inferência com recibo em GPU TEE)** — não têm substrato na GuaraCloud hoje. O que encaixa sem TEE é tudo que vive na camada de proveniência: hash, assinatura ML-DSA-65 e ancoragem MMR sobre artefatos e eventos que a GuaraCloud já produz.

**Vetores de parceria priorizados:**

| # | Vetor | Fit | τ |
|---|---|---|---|
| 1 | Provenança de deploy verificável (SLSA/in-toto com ML-DSA-65) | Alto | LEMMA |
| 2 | Trilha de auditoria imutável (Audit/Flow Logs + MMR ancorada) | Alto | LEMMA |
| 3 | Relatório Guara Shield assinado | Alto | LEMMA |
| 4 | Assinatura pós-quântica de artefatos em registries | Médio | LEMMA |
| 5 | Recibo de ação de agente via MCP/Agent Skills | Médio | CONJECTURE |

**Primeiro passo:** Propor um piloto único e barato — o **Vetor 1** (recibo de proveniência de deploy) sobre um serviço real da GuaraCloud, com verificador independente. É a menor prova que demonstra o valor inteiro.

**Alinhamento soberano:** Ambos são "Feito no Brasil", BRL e LGPD. A GuaraCloud é candidata natural a primeiro design-partner do tier *Sovereign Attestation* — transformando conformidade LGPD de alegação em propriedade verificável.

### 7.7 Mapa comparativo de parceiros

| Parceiro | BRL | USD | EUR | Crypto | Cartão | Self-custody | Setup cost | Go-live friction |
|---|---|---|---|---|---|---|---|---|
| ValidaPix | ✅ (valida) | ❌ | ❌ | ❌ | ❌ | n/a | Negociação | Negociação |
| Mt Pelerin | ❌ | ✅ (SWIFT) | ✅ (SEPA) | ✅ (16 chains) | ❌ | ✅ | **Zero** | **Email + key** |
| Picnic | ✅ (Avenia) | ✅ (Noah/ACH) | ✅ (SEPA) | ✅ (BTC/ETH/SOL/USDT) | ✅ Visa | ✅ | Negociação | LinkedIn |
| GuaraCloud | n/a | n/a | n/a | n/a | n/a | n/a | Co-dev | Co-dev |
| Alchemy ARC | n/a | n/a | n/a | ✅ (Arc L1) | n/a | n/a | **Zero** | Form web |

---

## 8. Justificativa geopolítica

### 8.1 Posicionamento canônico

> **Arkhe é um padrão técnico aberto. A soberania é consequência do design (offline, sem provedor, padrões abertos), não de marketing. Não é projeto de soberania; é projeto de verificação cuja soberania emerge como propriedade.**

### 8.2 Tese soberana (pós-análise Arkhe Cloud)

> **"Soberania sem operação é apenas marketing. Hospedar o dado em solo nacional não basta. Enquanto a nuvem soberana resolve jurisdição e localização, Arkhe adiciona prova verificável de controle — atestação de que a operação ocorreu sob a política declarada, com cadeia de custódia auditável. Isso transforma soberania de uma alegação contratual numa propriedade mecanicamente verificável."**

**τ:** PROOF — argumento válido e material. Raciocínio:
1. Soberania de dados (jurisdição + localização) é propriedade contratual/legal
2. Propriedades contratuais são auditáveis apenas via acesso ao contrato e ao provedor
3. Atestação criptográfica (TEE + receipt + MMR + anchor) torna a operação verificável independentemente do provedor
4. Logo: Arkhe transforma soberania de contratual em criptográfica

### 8.3 Sinais regulatórios brasileiros (confirmados canonicamente)

| Sinal | Fonte | Data | Status |
|---|---|---|---|
| TCU determina Serpro/Dataprev rever contratos com provedores estrangeiros (Amazon) | TCU/Poder360 | jun 2026 | `τ: LEMMA` |
| 180 dias para plano de redundância geográfica | TCU | jun 2026 | `τ: LEMMA` |
| Serpro adota modelo de soberania por níveis (dados, operação, tecnologia) | Serpro | 2024+ | `τ: LEMMA` |
| Ministério da Gestão fortalece soberania digital com Serpro/Dataprev | MG | jul 2025 | `τ: LEMMA` |
| Brasil 48º em ranking de soberania digital (BRICS forum) | BRICS | 2024 | `τ: LEMMA` |
| Investimento em nuvem BR ~15% a.a., ~R$ 331,9 bi até 2028 | Brasscom | 2024 | `τ: PENDING` (fonte não aberta) |

### 8.4 Encaixe Arkhe no movimento soberano BR

O argumento de inserção:

- **Nuvem soberana** resolve jurisdição e localização (contrato)
- **Arkhe** adiciona prova verificável de controle (criptografia)
- **Juntos:** soberania de alegação contratual → propriedade mecanicamente verificável

**Diferencial pós-quântico:** ML-DSA-65 por padrão alinha-se a soberania de longo prazo (resiliência a ataque quântico futuro). Matriz energética renovável do Brasil (~83%) é vantagem para compute de baixo carbono.

### 8.5 Tensões geopolíticas

| Dimensão | Posição Arkhe | Tensão |
|---|---|---|
| Padrão técnico | Aberto, não proprietário | n/a |
| Soberania digital | Consequência, não marketing | Brasil 48º em soberania digital |
| ML-DSA-65 | NIST (FIPS 204) | Padrão americano; BRICS pode preferir alternativas |
| Bitcoin | Offshore (mineração concentrada) | Bulletin board, não settlement |
| RISC Zero | zkVM americano | zkVM para slash de equivocação |
| Tor | Financiado por EUA | Transporte opcional, não raiz |
| BRICS alignment | Offline sovereign, PQ antecipado | Oportunidade, não obrigação |

### 8.6 Alinhamentos BRICS

- Verificação offline soberana
- PQ antecipado (ML-DSA-65 antes de migração regulada)
- Pagamentos verificáveis (Evidence Platform)
- IA verificável (AFiR inference receipts)
- Bitcoin como bulletin board (não como settlement)

---

## 9. Concorrência e diferenciação

### 9.1 Concorrentes reais identificados

O espaço de atestação de terceiros já tem incumbentes fortes. Arkhe não finge que não existem; diferencia-se explicitamente.

| Player | O que oferece | Onde Arkhe diferencia |
|---|---|---|
| **Intel Tiber Trust Authority** (ex-Project Amber) | Atestação multi-nuvem/multi-TEE independente do provedor | Pós-quântico por padrão (ML-DSA-65); ancoragem soberana; recibo portátil offline verificável sem acesso ao serviço |
| **Fortanix (CCM + DSM)** | Registro de workload, atestação, liberação de chave para GPU CC | Recibo portátil verificável por terceiro fora do fornecedor; foco em proveniência de IA |
| **Azure / Google attestation** | Atestação integrado ao próprio cloud | Independência de provedor + jurisdição brasileira |
| **Akash, io.net (DePIN)** | Compute P2P 50-85% mais barato | Camada de confiança/atestação sobre a capacidade bruta — Arkhe não precisa ser a nuvem, só a prova |
| **Chainalysis, TRM Labs, Elliptic** | AML/tracing on-chain | Não é evidence portátil; Arkhe é recibo verificável, não análise forense |
| **OpenTimestamps** | Ancoragem Bitcoin OP_RETURN | Arkhe usa OP_RETURN mas adiciona MMR + AFiR + ML-DSA-65 + OVB |

### 9.2 Diferencial canônico Arkhe

| Diferencial | Mecanismo | Concorrente não tem |
|---|---|---|
| Pós-quântico por padrão | ML-DSA-65 (FIPS 204) em toda assinatura | Intel Tiber, Fortanix usam ECDSA/RSA |
| Recibo portátil offline | OVB (ZIP com manifest + receipts + wasm + policy) | Atestação de concorrentes exige acesso ao serviço |
| Cadeia que cobre inferência | AFiR perfil 0x01 liga fingerprint + hash E/S + assinatura | Fortanix foca em enclave, não em IA |
| Ancoragem soberana | Bitcoin OP_RETURN + MMR + (futuro) multi-ledger | Concorrentes não ancoram em ledger público |
| Verificação no navegador | WASM (10 fases / 44 passos, sem servidor) | Concorrentes exigem API call |
| Trait `PaymentEventSource` | Genérico em qualquer PSP/ramp/chain | Concorrentes são vendor lock-in |
| Fronteira integridade ≠ corretude | Errata canônica explícita | Concorrentes não explicitam o que não provam |

---

## 10. Roadmap por maturidade

Reorganização honesta: o que se pode construir hoje com tecnologia de produção vs. o que ainda é pesquisa aberta. **Regra de disciplina:** nada da coluna "Pesquisa aberta" entra em material de cliente ou investidor como capacidade existente.

### 10.1 Primitivas maduras (construir agora)

```text
✅ ML-DSA-65 (FIPS 204) — pk 1952 B, sk 4032 B, sig 3309 B
✅ BLAKE3 + CBOR determinístico (RFC 8949 §4.2)
✅ MMR append-only (bagging de picos)
✅ AFiR receipt (perfis 0x01 inference, 0x03 data-fetch)
✅ Bitcoin OP_RETURN anchor (mainnet)
✅ TEE CPU (AMD SEV-SNP, Intel TDX) — produção 2026
✅ TEE GPU (NVIDIA H100+ CC-On) — produção 2026
✅ Intel Tiber Trust Authority — produção 2026
✅ PoRep/PoSt (Filecoin WindowPoSt) — produção 2026
```

### 10.2 Integração / engenharia (viável, exige trabalho)

```text
🔄 PaymentEventSource trait + adapters (Track 1-5) ← em andamento
🔄 OVB completo (manifest + bundle + verify) ← em andamento (Track 1)
🔄 WASM browser verifier (10 fases / 44 passos) ← em andamento (Track 1)
📋 AFiR + TEE binding (AIR service) ← pós-Hack Brasil
📋 DID + ML-DSA-65 (AID service) ← pós-Hack Brasil
📋 ICP-Brasil wrapper para AID ← pesquisa regulatória
📋 CAL multi-ledger adapters (Bitcoin + Solana + Base) ← em roadmap
📋 Account recovery + genesis ceremony ← em roadmap
📋 Recibo unificado sobre ACC/AIR/ACS/AID ← pós-Hack Brasil
📋 GuaraCloud Vetor 1 (recibo de proveniência de deploy) ← Track 5
```

### 10.3 Pesquisa aberta (NÃO prometer a cliente)

```text
🔬 Prova criptográfica de corretude de inferência em modelos frontier
🔬 zkML escalável para 14T+ params em tempo viável
🔬 Proof-of-Inference econômico sem TEE nem re-execução
🔬 Composição confidencial (Octra/FHE)
🔬 Substituição total de DNS/e-mail/OAuth por DID
🔬 DDoS volumétrico mitigado só por overlay
🔬 PoI tokenomics completo (burn-and-mint, staking, slashing)
🔬 Mobile evidence layer (StrongBox, MobileWER)
```

---

## 11. Errata canônica — o que Arkhe NÃO é

Esta seção registra as 6 refutações adotadas como canônicas após análise crítica do conceito Arkhe Cloud. São `τ: REFUTED` — o framework explicitamente não garante.

### 11.1 Refutações canônicas

| # | Claim refutada | Veredito | Correção |
|---|---|---|---|
| 1 | "Prova matemática de que sua IA não alucinou" | `τ: REFUTED` | Prova integridade e proveniência, não corretude. Alucinação é indecidível por hash. |
| 2 | "Recibo AFiR de 384 bytes" | `τ: REFUTED` | Assinatura ML-DSA-65 sozinha é 3309 B. Recibo é ordem de 5-8 KB. |
| 3 | "DDoS mitigado na DHT Kademlia antes de chegar ao Worker" | `τ: REFUTED` | Filtragem em overlay não para volumetria; pacotes já chegaram ao link físico. Slash pune nós com stake, não botnet anônimo. |
| 4 | "Proof-of-Inference para 14T params como pronto" | `τ: REFUTED` | Em 2026, frontier usa TEE + trace assinado; zkML não escala a frontier. |
| 5 | "HTTP da verificação", "o e-mail morre" | `τ: REFUTED` | ECHO retórico. Analogia não é prova; substituição por decreto não tem mecanismo. |
| 6 | "Agnóstico" como garantia de adoção | `τ: REFUTED` | Non-sequitur. Rodar sobre tudo ≠ alguém migrar. Agnosticismo é propriedade técnica, não estratégia de adoção. |

### 11.2 Erros factuais a corrigir em material derivado

| Claim | Correção | Fonte canônica |
|---|---|---|
| ML-DSA-65 pk ~1312 B | pk = **1952 B** | NIST FIPS 204 (ago 2024) |
| ML-DSA-65 sig ~2420 B | sig = **3309 B** | NIST FIPS 204 (ago 2024) |
| H100 1ª gen não cifra memória on-card | H100 PCIe CC-On cifra via encrypted bounce buffer (VBIOS 96.00.5E.00.01+, driver R535.86+, CPU CVM SEV-SNP/TDX) | NVIDIA Developer Blog |
| "Prova matemática" no pitch | "Assinatura pós-quântica" ou "prova criptográfica" | ML-DSA-65 é hipótese computacional (lattice), não prova formal |

### 11.3 Princípio canônico (fronteira integridade ≠ corretude)

> **Arkhe prova integridade** (saída não foi alterada), **proveniência** (sabe-se qual artefato gerou), e **atestação** (prova-se o ambiente de execução contra uma política). **Arkhe não prova corretude** (que a saída é verdadeira ou livre de alucinação). Esta fronteira é `τ: LEMMA` — consequência direta do que são funções de hash e assinaturas digitais.

### 11.4 O que Arkhe não faz (lista canônica)

| O que Arkhe NÃO faz | Por quê |
|---|---|
| Não move dinheiro | Não é PSP, não é banco, não é gateway |
| Não executa inferência | Não é runtime LLM, não é orchestrator |
| Não governa agentes | Não é ACI, não é constitution engine |
| Não substitui KYC do provedor | Apenas registra o nível de KYC observado |
| Não é "compliance oracle" | Implica obrigação fiduciária que o Arkhe recusa (§17) |
| Não substitui comprovante regulado (Pix) | Apenas adiciona camada de evidência portátil |
| Não é blockchain para pagamentos | Bitcoin é bulletin board; Arkhe não emite token |
| Não prova corretude semântica | Hash é indecidível sobre conteúdo |
| Não mitiga DDoS volumétrico | Overlay não para pacote no link físico |
| Não substitui DNS/e-mail/OAuth por decreto | Substituição exige mecanismo, não analogia |

---

## 12. Modelo de negócio e tiers

### 12.1 O que se vende

**Atestação e proveniência como serviço — a prova, não o hardware nem o dado.** O produto é o recibo verificável e a infraestrutura que o emite e ancora.

### 12.2 Tiers propostos

| Tier | O que entrega | Cliente | τ |
|---|---|---|---|
| **Verifiable API** (entrada) | Endpoint REST que devolve dado/inferência + recibo. Cliente não roda nó; só guarda o recibo. | Fintechs, fundos, devs de agentes com compliance | CONJECTURE |
| **Attested Node** (intermediário) | Software em container que parceiro (provedor de infra, nó DePIN) instala para emitir recibos e prover compute atestado. | Provedores de infra, DePINs | CONJECTURE |
| **Sovereign Attestation** (enterprise) | Camada de atestação ancorada em jurisdição, com DID/ML-DSA, para governo, saúde, bancos. Receita por licenciamento e integração. | Governo, saúde, bancos | CONJECTURE |

### 12.3 Modelo de pricing (alinhado entre parceiros)

```text
1. Evidence-as-a-Service (runtime Arkhe hospedado)
   - R$ 0,05-R$ 0,20 por record (multi-rail tipicamente mais valioso)
   - Para PSPs e ERPs que não querem infra própria
   - $0.01-$0.05 por record (cross-border, menor volume, maior margem)

2. Spec Kit license (para PSP implementar internamente)
   - R$ 120k-400k/ano
   - $30k-$100k/ano (cross-border)
   - Inclui specs, schemas, vetores de teste, conformance suite

3. Compliance Evidence Pack (produto vertical)
   - R$ 10-50 por bundle (chargeback evidence)
   - $5-$25 por bundle (cross-border compliance)
   - Para advogados, auditores, compliance officers

4. Treasury Evidence Pack (produto vertical Picnic)
   - R$ 50-200 por bundle
   - Para tesouraria corporativa: yield DeFi, posições, movimentações

5. Cross-border reconciliation API
   - $0.001-$0.01 por verificação
   - Para ERPs e sistemas de conciliação em lote
```

### 12.4 Critério de sucesso do Hack Brasil 2026

```text
1. Webhook simulado recebido e HMAC validado
2. Evento reconciliado via API simulada
3. VerificationRecord assinado com ML-DSA-65
4. MMR atualizado
5. Bundle offline exportado
6. Verificação no navegador bem-sucedida (10 fases / 44 passos)
7. Verificação via CLI bem-sucedida (fallback)
8. Alteração de histórico detectada (tamper)
9. Zero PII armazenado (apenas hashes pseudonimizados)
10. Demo completa em menos de 5 minutos
```

---

## 13. Pitch executivo de 30 segundos

> **Versão canônica (alinhada com errata):**
>
> "Nuvem soberana garante *onde* seu dado está. Nós garantimos, com assinatura pós-quântica, que a computação ocorreu como declarada — qual modelo, sobre qual entrada, sem adulteração, com custódia auditável. Não provamos que sua IA acertou; provamos que você pode auditar exatamente o que ela fez."
>
> **Arkhe não move dinheiro, não executa inferência, não governa agentes. Arkhe prova o que aconteceu — com criptografia pós-quântica real, verificável offline por qualquer terceiro, sem depender de provedor, servidor ou chain.**
>
> **Prove, do not promise.**

### 13.1 Versão para jurados de hackathon (5 min)

```text
[Minuto 0-1] Contexto
"O problema: lojista que recebe via Pix não tem prova criptográfica de pagamento.
Solução: Arkhe — recibo pós-quântico, MMR append-only, verificação offline."

[Minuto 1-2] Demo pagamento
$ evidence-cli emit --amount 100000 --pix
→ Receipt gerado, MMR atualizado

[Minuto 2-3] Split + export bundle
$ evidence-cli emit --amount 50000 --pix --split percent:10
$ evidence-cli export --out demo.ovb

[Minuto 3-4] Verificação no navegador
[Arrasta demo.ovb para localhost:5173]
→ 10 fases / 44 passos, PASS

[Minuto 4-5] Tamper detection + closing
[Clica "Tamper"]
→ FAIL fase 5 (file integrity)
"Prove, do not promise."
```

### 13.2 Versão para investidor (3 min)

```text
[0-30s] Tese
"Arkhe é a camada de evidência verificável para a economia computacional.
Cada pagamento, cada inferência, cada deploy — um recibo pós-quântico,
verificável offline por qualquer terceiro."

[30s-1min] Fronteira epistêmica
"Não provamos corretude — isso é impossível por hash. Provamos integridade,
proveniência e atestação. Esta fronteira é o que nos diferencia de quem
promete 'IA não alucina'."

[1min-2min] Tração técnica
"213 testes passando. ML-DSA-65 real (FIPS 204). AFiR slash proof em RISC Zero
zkVM verificado na Base Sepolia. 5 parceiros potenciais (ValidaPix, Mt Pelerin,
Picnic, GuaraCloud, Alchemy ARC Credits) — todos em outreach paralelo."

[2min-3min] Mercado
"TCU determinou Serpro/Dataprev rever contratos com provedores estrangeiros.
LGPD exige trilha auditável. MiCA exige atestação. Arkhe transforma soberania
de alegação contratual em propriedade mecanicamente verificável."
```

### 13.3 Versão para governo (5 min)

```text
[0-1min] Soberania como propriedade verificável
"Soberania sem operação é marketing. Nuvem soberana resolve jurisdição.
Arkhe adiciona prova verificável de controle — atestação criptográfica de
que a operação ocorreu sob a política declarada."

[1-2min] Stack pós-quântica
"ML-DSA-65 (FIPS 204) por padrão. Resiliência a ataque quântico futuro.
Brasil 48º em soberania digital — Arkhe é alavanca para subir no ranking."

[2-3min] Compatibilidade regulatória
"LGPD: zero PII no recibo (salted BLAKE3 hashes). Resolução BCB 195/2022:
recibo é complementar ao comprovante, não substituto. MiCA: Arkhe não é
CASP, é camada de evidência sobre CASPs existentes."

[3-4min] Parceiros soberanos
"GuaraCloud (PaaS BR) como design-partner. ValidaPix (validação Pix BR).
Picnic (conta multi-moeda BR). Toda a cadeia em BRL, LGPD, jurisdição BR."

[4-5min] Pitch
"Nuvem soberana garante onde seu dado está. Arkhe garante, com assinatura
pós-quântica, que a computação ocorreu como declarada. Não provamos que
a IA acertou; provamos que você pode auditar exatamente o que ela fez."
```

---

## Apêndice A — Invariantes canônicas

### A.1 Por família

| Família | Range | Documento | Implementado? |
|---|---|---|---|
| I-01..I-10 | Stack L1-L4 | Whitepaper §3.5 | Parcial (crypto + binding + MMR) |
| I-PC1..I-PC7 | Composição confidencial | Whitepaper §16.7 | ❌ |
| I-WG-01..I-08 | WormGraph | wormgraph-spec.md | Parcial (MMR no crate) |
| I-TV-01..I-08 | Vault timelocked | arkhe-vault.md | ❌ (commitment é PLACEHOLDER) |
| I-AR-01..I-09 | Account recovery | arkhe-recovery.md | ❌ |
| I-GB-01..I-05b | Genesis | arkhe-genesis.md | ❌ |
| I-GB-AGI-01..I-12 | AGI evidence | arkhe-situated-agency.md | ❌ |
| I-OVB-01..I-14 | Offline bundle | ovb-spec.md | Parcial (afir_bundle.rs) |
| I-VR-B1..B7 | Binding byte-exact | RFC-0002 §3 | ✅ (52 + 161 testes) |
| I-VR-I1..I5 | Independência | RFC-0002 §4 | ✅ |
| I-VR-C1..C9 | Contestação | RFC-0002 §6 | ✅ |
| I-VR-E1..E5 | Expiração | RFC-0002 §7 | ✅ |
| I-BIS-01..07 | Branch state | RFC-0002 §8 | ✅ |
| I-TAP-01..A23 | Transporte | RFC-0003 | Parcial (attestations no crate) |
| I-TOR-01..10 | Tor | tor-arkhe.md | ❌ |
| I-SAA-01..49 | Agência situada | arkhe-situated-agency.md | ❌ |
| I-ZID-01..10 | ZK identity | arkhe-verify-wasm-zk-identity.md | ❌ |

**Total:** 13 fontes de invariantes, ~150 invariantes individuais. **Cobertura executável:** ~45% dos invariantes têm pelo menos um teste que os exercita.

### A.2 Invariantes canônicos do OVB (I-OVB-01..14)

| Invariante | Descrição | Implementado |
|---|---|---|
| I-OVB-08 | Manifesto é a raiz de confiança (não `bundle.cbor`) | ✅ Track 1 |
| I-OVB-09 | `bundleId` excluído do próprio digest (chicken-and-egg) | ✅ Track 1 |
| I-OVB-10 | Atestados incluídos como bytes | ✅ Track 1 |
| I-OVB-11 | Digest do WASM pinado externamente | ✅ Track 1 |
| I-OVB-12 | Política validada antes dos checks condicionais | ✅ Track 1 |
| I-OVB-13 | Journal incluído quando exigido | ✅ Track 1 |

---

## Apêndice B — Framework τ

### B.1 Classificação epistêmica

| Label | Significado | P (confiança) | Exemplos no corpus |
|---|---|---|---|
| `τ: LEMMA` | Segue de fontes primárias verificadas | >0.95 | BLAKE3 known answers, ML-DSA-65 FIPS 204 (pk 1952, sk 4032, sig 3309), MMR proof |
| `τ: PROOF` | Estabelecido por argumento | >0.90 | I-VR-B1..B7 (binding), I-WG-01 (CRDT vs evidencial), "soberania sem operação é marketing" |
| `τ: PROPOSAL` | Decisão normativa desta especificação | variável | RFCs, invariantes canônicas, tiers de monetização |
| `τ: CONJECTURE` | Hipótese cujo valor depende de validação | 0.50-0.80 | Adoção de PSPs, tamanho de mercado de chargeback, encaixe soberano |
| `τ: REFUTED` | O que o framework explicitamente não garante | n/a | Correção semântica, AGI, "recibo 384 bytes", "DDoS na DHT" |
| `τ: PENDING` | Aguardando plano de validação | n/a | Tokenomics, mobile cost, MiCA equivalência |

### B.2 Regra do corpus

> **"Ausência de veredito é tratada como recusa, nunca aprovação."** (Whitepaper §1.3)

Esta regra obriga o framework a explicitar o que não prova. Qualquer afirmação técnica sem fonte canônica é `τ: PENDING` até verificação. Qualquer claim retórica sem mecanismo é `τ: REFUTED` como ECHO.

### B.3 Mapeamento τ(א) ↔ τ corpus

O conceito Arkhe Cloud usa nomenclatura levemente diferente. Mapeamento:

| Conceito Arkhe Cloud | Corpus Arkhe |
|---|---|
| THEOREM | LEMMA |
| LEMMA | LEMMA |
| CONJECTURE | CONJECTURE |
| PROPOSAL | PROPOSAL |
| ECHO | REFUTED |
| (sem equivalente) | PENDING |

### B.4 Disciplina de aplicação

1. Toda afirmação técnica em material externo deve carregar classificação τ
2. Toda claim `τ: LEMMA` deve citar fonte primária canônica (NIST, IETF, paper peer-reviewed)
3. Toda claim `τ: PROOF` deve apresentar argumento em cadeia
4. Toda claim `τ: CONJECTURE` deve indicar o que a validaria ou refutaria
5. Toda claim `τ: REFUTED` deve ser registrada em errata canônica
6. Toda claim `τ: PENDING` deve ter plano de validação com prazo

---

## Apêndice C — Fontes primárias

### C.1 Criptografia canônica

- **NIST FIPS 204** (ago 13, 2024) — `https://nvlpubs.nist.gov/nistpubs/FIPS/NIST.FIPS.204.pdf` — ML-DSA-65: pk 1952 B, sk 4032 B, sig 3309 B
- **IETF RFC 8949** (out 2020) — CBOR determinístico §4.2
- **BLAKE3** — IETF draft, RustCrypto implementation
- **Peter Todd MMR spec** — OpenTimestamps, em produção desde 2016

### C.2 Hardware TEE

- **NVIDIA Developer Blog — Confidential Computing on H100 GPUs** — `https://developer.nvidia.com/blog/confidential-computing-on-h100-gpus-for-secure-and-trustworthy-ai`
- **NVIDIA B200/B300 CC** (abr 2026) — NVLink encryption
- **AMD SEV-SNP** — produção desde 2020+
- **Intel TDX** — GA 2023+
- **Intel Tiber Trust Authority** (ex-Project Amber, mai 2022) — `https://docs.trustauthority.intel.com/`
- **Fortanix CCM + DSM** — `https://support.fortanix.com/docs/fortanix-ccm-overview-and-definitions`

### C.3 Regulação brasileira

- **TCU — Serpro/Dataprev revisão contratos nuvem** (jun 2026) — `https://www.poder360.com.br/poder-tech/tcu-manda-rever-contrato-de-estatais-com-a-amazon-por-risco-a-dados`
- **TCU — Plano 180 dias redundância geográfica** — confirmado
- **Serpro/Dataprev assumem contratos nuvem governo** (abr 2024) — `https://convergenciadigital.com.br/governo/serpro-e-dataprev-assumem-contratos-da-nuvem-de-governo-com-setor-privado`
- **Ministério da Gestão — Soberania digital** (jul 2025) — parceria Serpro/Dataprev
- **Resolução BCB 195/2022** — SPI (Sistema de Pagamentos Instantâneos)
- **LGPD (Lei 13.709/2018)** — `https://www.gov.br/anpd/pt-br`
- **LC 214/2025** — reforma tributária, split payment tributário (art. 31)
- **Lei 14.478/2022** — Cripto-Lei brasileira
- **IN RFB 1888/2019** — declaração mensal de operações com criptoativos
- **Lei 14.063/2020** — ICP-Brasil para atos formais

### C.4 Regulação internacional

- **MiCA (Regulamento UE 2023/1114)** — `https://eur-lex.europa.eu/eli/reg/2023/1114`
- **ESMA — MiCA Q&A** — `https://www.esma.europa.eu/policy-markets/markets-crypto-assets-regulation-mica`
- **FINMA** — `https://www.finma.ch` (autoridade supervisora suíça)
- **SO-FIT** — SRO sob Swiss AMLA; auditora do Mt Pelerin
- **Swiss AMLA** — threshold CHF 5.000/mês para KYC completo

### C.5 Parceiros

- **ValidaPix** — `https://blog.validapix.com.br/guia-completo-sobre-api-pix`
- **Mt Pelerin — Integrate us** — `https://www.mtpelerin.com/integrate-us`
- **Mt Pelerin — Pricing** — `https://www.mtpelerin.com/pricing`
- **Dfns fiat on/off-ramps** — `https://docs.dfns.co/features/fiat-on-off-ramps`
- **Alchemy docs** — `https://www.alchemy.com/docs`
- **Alchemy ARC Credits** — `https://www.alchemy.com/credits-program/arc`
- **Picnic** — `https://usepicnic.com/en`
- **Picnic — About Us** — `https://usepicnic.com/en/about-us`
- **Gnosis Pay** — `https://gnosispay.com`
- **OpenPix developers** — `https://developers.openpix.com.br`
- **GuaraCloud docs** — `https://guaracloud.com/docs/getting-started/introduction`
- **GuaraCloud Guara Shield** — `https://guaracloud.com/docs/guara-shield/overview`
- **Ramp Network** — `https://ramp.network`
- **Sardine** — `https://sardine.ai`
- **Akash Network** — `https://akash.network`
- **io.net** — `https://io.net`
- **Filecoin** — `https://filecoin.io`
- **Chainalysis** — `https://www.chainalysis.com`
- **TRM Labs** — `https://www.trmlabs.com`

### C.6 Comparativos e contexto

- **ParityDeals — Best Payment Gateways Brazil 2026** — panorama de gateways
- **IMF — Brazil's Pix Payment System** (2023) — design do Pix
- **Stripe — Guide to Pix payments in Brazil** — requisitos para PSPs

### C.7 Não verificadas (τ: PENDING)

- Tração do Picnic (usuários, volume) — validar via App Store/Google Play/Crunchbase
- Financiamento declarado do Picnic — verificar Crunchbase
- Status de autorização Dfns como CASP UE
- Equivalência de evidence records entre jurisdições (CH/UE/BR)
- "Investimento em nuvem BR ~R$ 331,9 bi até 2028" — Brasscom citado mas fonte não aberta
- "14T params" como tamanho de modelo frontier — modelos públicos estão em ~1-2T
- "Akash/io.net 50-85% mais barato" — declaração de marketing das próprias redes

---

## Regra final

> **Arkhe não move dinheiro, não executa inferência, não governa agentes. Arkhe prova o que aconteceu — com criptografia pós-quântica real (ML-DSA-65, FIPS 204), verificável offline por qualquer terceiro, sem depender de provedor, servidor ou chain.**
>
> **Prove, do not promise.**
