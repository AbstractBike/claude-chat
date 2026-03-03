use claude_chat::agent::tool::{format_agent_message, parse_tool_calls, ToolCall};

#[test]
fn parses_send_to_agent_call() {
    let output = r#"I'll ask the other agent.
<tool>send_to_agent("nixos", "what is the current system generation?")</tool>
Waiting for response..."#;

    let calls = parse_tool_calls(output);
    assert_eq!(calls.len(), 1);
    if let ToolCall::SendToAgent { agent, message } = &calls[0] {
        assert_eq!(agent, "nixos");
        assert_eq!(message, "what is the current system generation?");
    } else {
        panic!("expected SendToAgent tool call");
    }
}

#[test]
fn parses_get_secret_call() {
    let output = r#"<tool>get_secret("github-token")</tool>"#;
    let calls = parse_tool_calls(output);
    assert_eq!(calls.len(), 1);
    assert!(matches!(&calls[0], ToolCall::GetSecret(name) if name == "github-token"));
}

#[test]
fn parses_multiple_tool_calls() {
    let output = r#"<tool>get_secret("npm-token")</tool>
Some text here.
<tool>send_to_agent("home", "sync packages")</tool>"#;
    let calls = parse_tool_calls(output);
    assert_eq!(calls.len(), 2);
}

#[test]
fn returns_empty_on_no_tool_calls() {
    let output = "Just a normal Claude response with no tools.";
    assert!(parse_tool_calls(output).is_empty());
}

#[test]
fn formats_inter_agent_message_with_depth() {
    let msg = format_agent_message("nixos", 0, "hello world");
    assert_eq!(msg, "[from:nixos, depth:0] hello world");
}
