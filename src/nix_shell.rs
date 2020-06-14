use crate::nix::NixShellTemplate;
use anyhow::Result;
use askama::Template;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

pub struct NixShell {
    pub pure: bool,
    pub template: NixShellTemplate,
}

impl NixShell {
    pub fn enter(self) -> Result<()> {
        let shell_code = self.template.render()?;
        let mut shell_file = NamedTempFile::new()?;
        shell_file.write_all(shell_code.as_bytes())?;

        let status = Command::new("nix-shell")
            .arg(if self.pure { "-p" } else { "" })
            .arg(shell_file.path())
            .status()?;
        if status.success() {
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Failed to launch nix-shell '{:?}'",
                shell_file.path()
            ))
        }
    }
}
