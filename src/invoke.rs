use std::process::{Command, Stdio};

/// Invoke a watched Lambda on the configured local endpoint.
///
/// We thread the port through explicitly so `srill` stays aligned with
/// `cargo lambda watch --invoke-port ...` instead of silently assuming 9000.
pub fn invoke(lambda: &str, event: &str, invoke_port: u16) -> anyhow::Result<InvokeResult> {
    let output = Command::new("cargo")
        .arg("lambda")
        .arg("invoke")
        .arg(lambda)
        .arg("--invoke-port")
        .arg(invoke_port.to_string())
        .arg("--data-ascii")
        .arg(event)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;
    Ok(InvokeResult {
        success: output.status.success(),
    })
}

#[derive(Debug, Clone)]
pub struct InvokeResult {
    pub success: bool,
}
