// MCP secrets server module
use anyhow::Result;
use base64::Engine;
use metrics::histogram;
use std::time::Instant;

use super::vault::Vault;

/// MCP server exposing the get_secret tool to a Claude agent.
/// One instance per agent — knows which agent is calling.
pub struct SecretsVaultServer {
    vault: Vault,
    agent_name: String,
}

impl SecretsVaultServer {
    pub fn new(vault_root: &str, agent_name: &str) -> Result<Self> {
        let vault = Vault::load(vault_root)?;
        Ok(Self {
            vault,
            agent_name: agent_name.to_string(),
        })
    }

    /// Handle a get_secret tool call.
    /// Returns age-encrypted blob as base64, or error message.
    pub fn handle_get_secret(&self, secret_name: &str) -> Result<String, String> {
        let start = Instant::now();

        // Read secret (enforces policy)
        let plaintext = self
            .vault
            .read_secret(&self.agent_name, secret_name)
            .map_err(|e| e.to_string())?;

        // Get agent's public key
        let pubkey = self
            .vault
            .read_public_key(&self.agent_name)
            .map_err(|e| format!("public key not found for {}: {e}", self.agent_name))?;

        // Encrypt with agent's public key
        let encrypted = super::vault::encrypt_for_agent(&plaintext, &pubkey)
            .map_err(|e| format!("encryption failed: {e}"))?;

        let elapsed = start.elapsed().as_secs_f64();
        histogram!(
            "claude_chat_mcp_duration_seconds",
            "agent" => self.agent_name.clone(),
            "tool" => "get_secret"
        )
        .record(elapsed);

        // Return as base64
        Ok(base64::engine::general_purpose::STANDARD.encode(&encrypted))
    }

    /// System prompt fragment for this agent
    pub fn system_prompt(&self, available_agents: &[String]) -> String {
        let agents_list = available_agents.join(", ");
        format!(
            r#"You have the following tools available:

1. get_secret(name: string) -> string
   Fetches a secret by name. Returns the value age-encrypted with your public key.
   Decrypt with: echo "<value>" | base64 -d | age --decrypt -i ~/.agent-store/{agent}/agent.key

2. send_to_agent(agent: string, message: string) -> string
   Sends a message to another agent and waits for its response.
   Available agents: {agents_list}
"#,
            agent = self.agent_name
        )
    }
}
