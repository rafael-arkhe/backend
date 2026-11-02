# Arkhe-Test: Pipeline de Qualidade e Invariantes

Pipeline contínuo de testes de invariantes (I-01 a I-10, I-T01 a I-T08), harness adversarial e motor de evidência auditável para a arquitetura **Arkhe AGI-P2P** e **ArkheOS Mobile**.

## Invariantes Arkhe

| Invariante | Nome | Critério Falsificável |
|---|---|---|
| **I-01** | Chunk Íntegro | Alteração de 1 byte resulta em rejeição imediata do CIDv1 (OmniNode). |
| **I-02** | Manifesto Verificável | Modificação de CID ou omissão de chunk invalida o manifesto do modelo. |
| **I-03** | Recibo AFiR | Assinatura ML-DSA-65 errada ou replay de nonce rejeita o recibo. |
| **I-04** | MMR Append-Only | Tentativa de sobrescrever folha passada ou reduzir MMR invalida a raiz. |
| **I-05** | Auditoria Offline | Verificador offline detecta bundle alterado sem conexão de rede. |
| **I-06** | Vínculo ao Dispositivo | Binding de chave e atestação de hardware Titan M2 / StrongBox validado. |
| **I-07** | Governança Humana | Propostas autônomas não podem alterar a constituição sem aprovação canônica. |
| **I-08** | Reward por Prova | Pagamento/incentivo bloqueado sem recibo AFiR e MMR válidos. |
| **I-09** | Evidência Chain-Agnostic | Bundle verificado independentemente de blockchain. |
| **I-10** | Papéis Disjuntos | Nó que opera como worker não pode validar no mesmo epoch. |

## Invariantes do Pipeline (I-T01 a I-T08)

- **I-T01 — Teste antes de merge**: CI bloqueia merge sem validação dos 10 invariantes.
- **I-T02 — Fail-closed**: qualquer falha em teste de segurança bloqueia o deploy.
- **I-T03 — Rastreabilidade**: todo teste referencia os invariantes formais do Arkhe.
- **I-T04 — Reprodutibilidade**: execução determinística com golden vectors.
- **I-T05 — Rollback seguro**: rollback nunca reescreve nem deleta MMR ou recibos.
- **I-T06 — Feature flags invioláveis**: flags não podem desativar verificações fundamentais de segurança.
- **I-T07 — Evidência auditável**: cada execução gera relatório assinado e content-addressed.
- **I-T08 — Teste adversarial contínuo**: o pipeline testa exaustivamente caminhos adversariais de falha.

## Comandos do CLI

```bash
# Executar suíte completa de invariantes nominais e adversariais
cargo run -- run --stage all

# Validar políticas de feature flags
cargo run -- verify-flags

# Validar harness de rollback seguro e integridade de evidência
cargo run -- verify-rollback

# Gerar golden vectors determinísticos
cargo run -- generate-vectors
```
