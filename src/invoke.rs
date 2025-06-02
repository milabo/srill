use std::process::{Command, Stdio};

pub fn invoke(lambda: &str, event: &str) -> anyhow::Result<InvokeResult> {
    let output = Command::new("cargo")
        .arg("lambda")
        .arg("invoke")
        .arg(lambda)
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
