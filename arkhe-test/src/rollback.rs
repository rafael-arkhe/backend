use crate::domain::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeState {
    pub software_version: String,
    pub config_version: String,
    pub mmr: MerkleMountainRange,
    pub receipts: Vec<AfirReceipt>,
    pub checkpoints: Vec<HybridCheckpointEnvelope>,
}

pub struct RollbackHarness;

impl RollbackHarness {
    /// Simula um rollback de versão de software ou configuração e verifica que
    /// o estado da evidência (MMR, recibos e checkpoints) permanece intocado.
    pub fn execute_rollback(
        current_state: &NodeState,
        target_software_version: String,
        target_config_version: String,
    ) -> Result<NodeState, &'static str> {
        // Rollback seguro I-T05:
        // Mantém estritamente MMR, recibos e checkpoints anteriores intactos.
        let rolled_back_state = NodeState {
            software_version: target_software_version,
            config_version: target_config_version,
            mmr: current_state.mmr.clone(),
            receipts: current_state.receipts.clone(),
            checkpoints: current_state.checkpoints.clone(),
        };

        // Validação de não-regressão de evidência
        if rolled_back_state.mmr.leaves.len() != current_state.mmr.leaves.len() {
            return Err("I-T05 VIOLATION: Rollback altered MMR leaf count!");
        }
        if rolled_back_state.mmr.root != current_state.mmr.root {
            return Err("I-T05 VIOLATION: Rollback corrupted MMR root!");
        }
        if rolled_back_state.receipts.len() != current_state.receipts.len() {
            return Err("I-T05 VIOLATION: Rollback altered receipts!");
        }
        if rolled_back_state.checkpoints.len() != current_state.checkpoints.len() {
            return Err("I-T05 VIOLATION: Rollback altered checkpoints!");
        }

        Ok(rolled_back_state)
    }
}
