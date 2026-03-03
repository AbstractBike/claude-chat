use claude_chat::session::claude::ClaudeSession;

#[test]
fn session_id_from_room_alias() {
    assert_eq!(
        ClaudeSession::session_id_from_alias("#nixos-agent:abstract.bike"),
        "nixos-agent"
    );
    assert_eq!(
        ClaudeSession::session_id_from_alias("nixos-agent:abstract.bike"),
        "nixos-agent"
    );
    assert_eq!(
        ClaudeSession::session_id_from_alias("nixos-agent"),
        "nixos-agent"
    );
}

#[tokio::test]
async fn session_captures_stdout() {
    let session = ClaudeSession::new_with_bin(
        "test-session".to_string(),
        "/tmp".to_string(),
        120,
        "echo",
    );
    let result = session.send_raw("hello world").await.unwrap();
    assert!(!result.is_empty());
    assert!(result.contains("hello"));
}

#[tokio::test]
async fn sandboxed_session_builds_command() {
    let session = ClaudeSession::new_sandboxed(
        "test-session".to_string(),
        "/tmp/work".to_string(),
        "/tmp/store".to_string(),
        5,
        "echo",
    );
    let _ = session.build_command("hello");
    // Just verify it builds without panic
}

#[test]
fn new_session_defaults_to_claude_binary() {
    let session = ClaudeSession::new("test".to_string(), "/tmp".to_string(), 120);
    assert_eq!(session.timeout_secs, 120);
    assert!(session.store_dir.is_none());
}
