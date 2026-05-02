use std::process::Command;

/// Check whether a tool is available on PATH.
pub fn has_tool(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Require that a tool is available, panicking with a clear message if not.
pub fn require_tool(name: &str) {
    if !has_tool(name) {
        panic!(
            "required external tool `{name}` is not available on PATH; \
             install it or adjust your development environment"
        );
    }
}

/// Return a `Command` for `node`, requiring it to be available.
pub fn node_command() -> Command {
    require_tool("node");
    Command::new("node")
}

/// Return a `Command` for `iwasm`, requiring it to be available.
pub fn iwasm_command() -> Command {
    require_tool("iwasm");
    Command::new("iwasm")
}
