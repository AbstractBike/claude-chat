use anyhow::Result;
use metrics::counter;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PolicyError {
    #[error("access denied: agent '{agent}' cannot read '{secret}'")]
    Denied { agent: String, secret: String },
    #[error("secret not found: '{0}'")]
    NotFound(String),
}

#[derive(Debug, Deserialize)]
struct Policy {
    agents: HashMap<String, AgentPolicy>,
}

#[derive(Debug, Deserialize)]
struct AgentPolicy {
    allowed_secrets: Vec<String>,
}

pub struct Vault {
    root: PathBuf,
    policy: Policy,
}

impl Vault {
    pub fn load(root: &str) -> Result<Self> {
        let root = PathBuf::from(root);
        let policy_path = root.join("policy.toml");
        let policy_str = std::fs::read_to_string(&policy_path)?;
        let policy: Policy = toml::from_str(&policy_str)?;
        Ok(Self { root, policy })
    }

    pub fn read_secret(&self, agent: &str, secret: &str) -> Result<String, PolicyError> {
        let agent_policy = self.policy.agents.get(agent).ok_or_else(|| {
            self.record_access(agent, secret, "denied");
            PolicyError::Denied {
                agent: agent.to_string(),
                secret: secret.to_string(),
            }
        })?;

        if !agent_policy.allowed_secrets.iter().any(|s| s == secret) {
            self.record_access(agent, secret, "denied");
            return Err(PolicyError::Denied {
                agent: agent.to_string(),
                secret: secret.to_string(),
            });
        }

        let secret_path = self.root.join("vault").join(secret);
        let content = std::fs::read_to_string(&secret_path)
            .map_err(|_| PolicyError::NotFound(secret.to_string()))?;

        self.record_access(agent, secret, "granted");

        Ok(content)
    }

    pub fn read_public_key(&self, agent: &str) -> Result<String> {
        let key_path = self.root.join("keys").join(format!("{}.pub", agent));
        Ok(std::fs::read_to_string(key_path)?.trim().to_string())
    }

    fn record_access(&self, agent: &str, secret: &str, result: &str) {
        counter!(
            "claude_chat_mcp_secret_requests_total",
            "agent" => agent.to_string(),
            "secret" => secret.to_string(),
            "result" => result.to_string()
        )
        .increment(1);

        tracing::info!(
            service = "claude-chat",
            event = "secret_access",
            agent = agent,
            secret = secret,
            result = result
        );
    }
}
