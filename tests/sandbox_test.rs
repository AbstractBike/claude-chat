use claude_chat::sandbox::bwrap::BwrapBuilder;

#[test]
fn bwrap_command_contains_required_args() {
    let cmd = BwrapBuilder::new("/home/user/git/nixos", "/home/user/.agent-store/nixos")
        .build_args();

    assert!(cmd.iter().any(|a| a == "/nix"), "missing /nix ro-bind");
    assert!(cmd.iter().any(|a| a == "/usr"), "missing /usr ro-bind");
    assert!(
        cmd.iter().any(|a| a == "/etc/resolv.conf"),
        "missing resolv.conf"
    );
    assert!(
        cmd.iter().any(|a| a == "/home/user/git/nixos"),
        "missing workdir bind"
    );
    assert!(
        cmd.iter().any(|a| a == "/home/user/.agent-store/nixos"),
        "missing store bind"
    );
    assert!(
        cmd.contains(&"--unshare-all".to_string()),
        "missing --unshare-all"
    );
    assert!(
        cmd.contains(&"--share-net".to_string()),
        "missing --share-net"
    );
    assert!(
        cmd.contains(&"--die-with-parent".to_string()),
        "missing --die-with-parent"
    );
    assert!(cmd.contains(&"--proc".to_string()), "missing --proc");
    assert!(cmd.contains(&"--dev".to_string()), "missing --dev");
    assert!(cmd.contains(&"--tmpfs".to_string()), "missing --tmpfs");
}

#[test]
fn bwrap_does_not_expose_home_dir() {
    let cmd = BwrapBuilder::new("/home/user/git/nixos", "/home/user/.agent-store/nixos")
        .build_args();

    let args_str = cmd.join(" ");
    // Must NOT blindly bind entire home directory
    assert!(
        !args_str.contains("--bind /home/user /home/user"),
        "exposes entire home dir"
    );
    assert!(!args_str.contains(".ssh"), "exposes .ssh");
    assert!(!args_str.contains(".gnupg"), "exposes .gnupg");
    assert!(!args_str.contains(".config"), "exposes .config");
}

#[test]
fn bwrap_ro_binds_are_read_only() {
    let cmd = BwrapBuilder::new("/tmp/work", "/tmp/store").build_args();

    // /nix must be --ro-bind, not --bind
    // Pattern is [--ro-bind, /nix, /nix] so first /nix is at offset 1 from the flag
    let nix_idx = cmd.iter().position(|a| a == "/nix").unwrap();
    assert_eq!(cmd[nix_idx - 1], "--ro-bind", "/nix should be ro-bind");

    // /usr must be --ro-bind
    let usr_idx = cmd.iter().position(|a| a == "/usr").unwrap();
    assert_eq!(cmd[usr_idx - 1], "--ro-bind", "/usr should be ro-bind");
}

#[test]
fn bwrap_workdir_is_read_write() {
    let cmd = BwrapBuilder::new("/tmp/work", "/tmp/store").build_args();

    let work_idx = cmd.iter().position(|a| a == "/tmp/work").unwrap();
    assert_eq!(
        cmd[work_idx - 1],
        "--bind",
        "workdir should be --bind (r/w)"
    );
}

#[test]
fn wrap_command_creates_valid_command() {
    let bwrap = BwrapBuilder::new("/tmp/work", "/tmp/store");
    let cmd = bwrap.wrap_command("claude", &["--resume", "test", "-p", "hello"]);
    // Just verify it builds without panic
    let _ = cmd;
}
