use base64::Engine;
use claude_chat::secrets::vault::{Vault, PolicyError};
use tempfile::TempDir;
use std::fs;

fn setup_vault(dir: &TempDir) -> Vault {
    fs::create_dir_all(dir.path().join("vault")).unwrap();
    fs::write(dir.path().join("vault/github-token"), "ghp_test123\n").unwrap();
    fs::write(dir.path().join("vault/npm-token"), "npm_test456\n").unwrap();

    let policy = r#"
[agents.nixos]
allowed_secrets = ["github-token"]

[agents.claude-chat]
allowed_secrets = ["github-token", "npm-token"]
"#;
    fs::write(dir.path().join("policy.toml"), policy).unwrap();

    Vault::load(dir.path().to_str().unwrap()).unwrap()
}

#[test]
fn grants_access_to_allowed_secret() {
    let dir = TempDir::new().unwrap();
    let vault = setup_vault(&dir);
    let result = vault.read_secret("nixos", "github-token");
    assert!(result.is_ok());
    assert!(result.unwrap().contains("ghp_test123"));
}

#[test]
fn denies_access_to_forbidden_secret() {
    let dir = TempDir::new().unwrap();
    let vault = setup_vault(&dir);
    let result = vault.read_secret("nixos", "npm-token");
    assert!(matches!(result, Err(PolicyError::Denied { .. })));
}

#[test]
fn denies_access_for_unknown_agent() {
    let dir = TempDir::new().unwrap();
    let vault = setup_vault(&dir);
    let result = vault.read_secret("unknown-agent", "github-token");
    assert!(matches!(result, Err(PolicyError::Denied { .. })));
}

#[test]
fn denies_nonexistent_secret() {
    let dir = TempDir::new().unwrap();
    let vault = setup_vault(&dir);
    let result = vault.read_secret("claude-chat", "nonexistent");
    assert!(result.is_err());
}

#[test]
fn encrypts_and_decrypts_secret() {
    use age::secrecy::ExposeSecret;
    use age::x25519;

    let identity = x25519::Identity::generate();
    let pubkey = identity.to_public();
    let pubkey_str = pubkey.to_string();

    let plaintext = "ghp_test_token_value";
    let encrypted =
        claude_chat::secrets::vault::encrypt_for_agent(plaintext, &pubkey_str).unwrap();

    // Encrypted blob must not contain plaintext
    assert!(
        !encrypted
            .windows(plaintext.len())
            .any(|w| w == plaintext.as_bytes()),
        "plaintext leaked into ciphertext"
    );

    // Decrypt and verify round-trip
    let identity_str = identity.to_string().expose_secret().to_string();
    let decrypted =
        claude_chat::secrets::vault::decrypt_with_identity(&encrypted, &identity_str).unwrap();
    assert_eq!(decrypted, plaintext);
}

// --- MCP Server tests ---

use claude_chat::secrets::mcp_server::SecretsVaultServer;

#[test]
fn mcp_server_builds_without_panic() {
    let dir = TempDir::new().unwrap();
    setup_vault(&dir);
    let server = SecretsVaultServer::new(dir.path().to_str().unwrap(), "nixos");
    assert!(server.is_ok());
}

#[test]
fn mcp_server_handles_get_secret_with_encryption() {
    use age::secrecy::ExposeSecret;
    use age::x25519;

    let dir = TempDir::new().unwrap();
    setup_vault(&dir);

    // Create agent keypair
    let identity = x25519::Identity::generate();
    let pubkey = identity.to_public();

    // Write public key
    fs::create_dir_all(dir.path().join("keys")).unwrap();
    fs::write(dir.path().join("keys/nixos.pub"), pubkey.to_string()).unwrap();

    let server = SecretsVaultServer::new(dir.path().to_str().unwrap(), "nixos").unwrap();
    let result = server.handle_get_secret("github-token");
    assert!(result.is_ok(), "get_secret should succeed: {:?}", result);

    // The result should be base64-encoded encrypted data
    let b64 = result.unwrap();
    let encrypted =
        base64::engine::general_purpose::STANDARD.decode(&b64).unwrap();

    // Decrypt and verify
    let identity_str = identity.to_string().expose_secret().to_string();
    let decrypted =
        claude_chat::secrets::vault::decrypt_with_identity(&encrypted, &identity_str).unwrap();
    assert!(decrypted.contains("ghp_test123"));
}

#[test]
fn mcp_server_denies_unauthorized_secret() {
    let dir = TempDir::new().unwrap();
    setup_vault(&dir);
    let server = SecretsVaultServer::new(dir.path().to_str().unwrap(), "nixos").unwrap();
    let result = server.handle_get_secret("npm-token");
    assert!(result.is_err());
}
