use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlagConfig {
    pub enabled: bool,
    pub target: String,
    pub allows_disabling_security: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FlagsPolicy {
    pub flags: HashMap<String, FlagConfig>,
    pub inviolable_security_checks: InviolableChecks,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InviolableChecks {
    pub mandatory: Vec<String>,
}

impl FlagsPolicy {
    pub fn parse(toml_str: &str) -> Result<Self, toml::de::Error> {
        toml::from_str(toml_str)
    }

    /// I-T06: Feature flags can activate features, NEVER disable mandatory security verifications!
    pub fn validate_safety(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();
        let forbidden_targets = [
            "signature",
            "hash",
            "mmr",
            "append_only",
            "role_separation",
            "security",
            "attestation",
        ];

        for (flag_name, config) in &self.flags {
            if config.allows_disabling_security {
                errors.push(format!(
                    "I-T06 VIOLATION: Flag '{}' specifies allows_disabling_security = true",
                    flag_name
                ));
            }

            let lower_name = flag_name.to_lowercase();
            let lower_target = config.target.to_lowercase();
            for forbidden in &forbidden_targets {
                if (lower_name.contains(forbidden) && lower_name.contains("disable"))
                    || (lower_target.contains(forbidden) && lower_target.contains("bypass"))
                {
                    errors.push(format!(
                        "I-T06 VIOLATION: Flag '{}' attempts to control inviolable security check '{}'",
                        flag_name, forbidden
                    ));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
