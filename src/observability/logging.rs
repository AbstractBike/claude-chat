use tracing_subscriber::{fmt, EnvFilter};

pub fn init() {
    let _ = fmt()
        .json()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive("claude_chat=info".parse().unwrap()),
        )
        .with_current_span(true)
        .try_init();
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_does_not_panic() {
        super::init();
        super::init(); // must be idempotent
    }
}
