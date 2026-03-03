use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum ToolCall {
    SendToAgent { agent: String, message: String },
    GetSecret(String),
}

pub fn parse_tool_calls(text: &str) -> Vec<ToolCall> {
    let mut calls = Vec::new();

    let send_re = Regex::new(
        r#"<tool>send_to_agent\s*\(\s*"([^"]+)"\s*,\s*"([^"]+)"\s*\)</tool>"#,
    )
    .unwrap();
    for cap in send_re.captures_iter(text) {
        calls.push(ToolCall::SendToAgent {
            agent: cap[1].to_string(),
            message: cap[2].to_string(),
        });
    }

    let secret_re =
        Regex::new(r#"<tool>get_secret\s*\(\s*"([^"]+)"\s*\)</tool>"#).unwrap();
    for cap in secret_re.captures_iter(text) {
        calls.push(ToolCall::GetSecret(cap[1].to_string()));
    }

    calls
}

pub fn format_agent_message(from: &str, depth: u8, text: &str) -> String {
    format!("[from:{from}, depth:{depth}] {text}")
}
