use anyhow::Result;
use std::net::SocketAddr;

pub async fn start_metrics_server(addr: SocketAddr) -> Result<()> {
    metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_http_listener(addr)
        .install()?;
    tracing::info!(%addr, "metrics server started");
    Ok(())
}

pub fn register_metrics() {
    use metrics::{describe_counter, describe_gauge, describe_histogram, Unit};

    // Matrix
    describe_counter!("claude_chat_matrix_messages_received_total", "Matrix messages received");
    describe_counter!("claude_chat_matrix_messages_sent_total", "Matrix messages sent");
    describe_counter!("claude_chat_matrix_sync_errors_total", "Matrix sync errors");
    describe_histogram!(
        "claude_chat_matrix_sync_duration_seconds",
        Unit::Seconds,
        "Sync duration"
    );
    describe_counter!("claude_chat_matrix_api_requests_total", "Matrix API requests");
    describe_histogram!(
        "claude_chat_matrix_api_duration_seconds",
        Unit::Seconds,
        "API request duration"
    );

    // Auth
    describe_counter!("claude_chat_auth_checks_total", "Auth checks performed");

    // Sessions
    describe_counter!("claude_chat_session_started_total", "Claude sessions started");
    describe_counter!("claude_chat_session_completed_total", "Claude sessions completed");
    describe_gauge!("claude_chat_session_active", Unit::Count, "Active Claude sessions");
    describe_histogram!(
        "claude_chat_session_duration_seconds",
        Unit::Seconds,
        "Session duration"
    );
    describe_histogram!(
        "claude_chat_session_output_bytes",
        Unit::Bytes,
        "Session output size"
    );
    describe_counter!("claude_chat_session_resume_total", "Session resume attempts");

    // Commands
    describe_counter!("claude_chat_command_executed_total", "Subprocess commands executed");
    describe_histogram!(
        "claude_chat_command_duration_seconds",
        Unit::Seconds,
        "Command duration"
    );
    describe_histogram!(
        "claude_chat_command_stdout_bytes",
        Unit::Bytes,
        "Command stdout size"
    );

    // Sandbox
    describe_counter!("claude_chat_bwrap_spawns_total", "Bubblewrap sandbox spawns");
    describe_counter!("claude_chat_bwrap_failures_total", "Bubblewrap failures");
    describe_gauge!("claude_chat_store_bytes", Unit::Bytes, "Agent store size");

    // Secrets (MCP)
    describe_counter!("claude_chat_mcp_secret_requests_total", "MCP secret requests");
    describe_counter!(
        "claude_chat_mcp_secret_decrypt_errors_total",
        "MCP decrypt errors"
    );
    describe_counter!("claude_chat_mcp_requests_total", "MCP requests total");
    describe_histogram!(
        "claude_chat_mcp_duration_seconds",
        Unit::Seconds,
        "MCP request duration"
    );

    // Inter-agent
    describe_counter!("claude_chat_agent_messages_sent_total", "Inter-agent messages sent");
    describe_counter!(
        "claude_chat_agent_messages_received_total",
        "Inter-agent messages received"
    );
    describe_histogram!(
        "claude_chat_agent_roundtrip_seconds",
        Unit::Seconds,
        "Inter-agent roundtrip"
    );
    describe_counter!("claude_chat_agent_tool_calls_total", "Agent tool calls");
    describe_counter!("claude_chat_agent_queue_rejected_total", "Agent queue rejected");
    describe_counter!("claude_chat_agent_loop_rejected_total", "Agent loop rejected");

    // Queue
    describe_gauge!(
        "claude_chat_agent_pending_messages",
        Unit::Count,
        "Pending messages"
    );
    describe_gauge!(
        "claude_chat_agent_processing_lag_seconds",
        Unit::Seconds,
        "Processing lag"
    );
    describe_counter!(
        "claude_chat_agent_messages_processed_total",
        "Messages processed"
    );

    // HTTP
    describe_counter!("claude_chat_http_requests_total", "HTTP requests");
    describe_histogram!(
        "claude_chat_http_duration_seconds",
        Unit::Seconds,
        "HTTP duration"
    );

    // Control
    describe_counter!("claude_chat_control_commands_total", "Control room commands");

    // System
    describe_gauge!("claude_chat_uptime_seconds", Unit::Seconds, "Bot uptime");
    describe_gauge!("claude_chat_rooms_configured", Unit::Count, "Rooms configured");
    describe_gauge!("claude_chat_rooms_active", Unit::Count, "Rooms active");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn metrics_server_starts() {
        let addr: SocketAddr = "127.0.0.1:0".parse().unwrap();
        let result = start_metrics_server(addr).await;
        assert!(result.is_ok(), "metrics server failed: {:?}", result);
    }

    #[test]
    fn register_does_not_panic() {
        register_metrics();
    }
}
