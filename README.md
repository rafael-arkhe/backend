# DMail AI DFX Backend

## Arkhe-Test Pipeline (I-01 a I-10)

O projeto inclui o pipeline contínuo `arkhe-test` em `arkhe-test/` para verificação rigorosa dos invariantes da arquitetura Arkhe AGI-P2P & ArkheOS Mobile:

- **Matriz de Invariantes I-01..I-10**: Chunk íntegro, manifesto verificável, recibos AFiR ML-DSA-65, MMR append-only, auditoria offline, vínculo Titan M2/StrongBox, governança humana, reward por prova, evidência chain-agnostic e papéis disjuntos.
- **Fail-Closed & Adversarial**: Cada invariante possui teste nominal e contraprova adversarial de rejeição obrigatória.
- **Políticas & Rollback**: Verificação de inviolabilidade de feature flags (`policies/flags.toml`) e preservação absoluta de evidências em rollbacks (`policies/rollback.toml`).

Para executar a suíte:
```bash
cd arkhe-test
cargo test
cargo run -- run --stage all
cargo run -- verify-flags
cargo run -- verify-rollback
```

## Init 
```sh
./bootstrap.sh
dfx start
```

## Deploy 
```sh
Create all canister:
Deploying all canisters.
All canisters have already been created.
Building canisters...
Building frontend...
Installing canisters...
Upgrading code for canister backend, with canister_id rrkah-fqaaa-aaaaa-aaaaq-cai
Upgrading code for canister backend_assets, with canister_id ryjl3-tyaaa-aaaaa-aaaba-cai
Authorizing our identity (default) to the asset canister...
Uploading assets to asset canister...
```

build
```sh
sudo dfx build --all
Building canisters...
```

install
```
sudo dfx canister install --all -m=reinstall
``` 


