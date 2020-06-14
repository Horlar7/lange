use crate::nix::NixShellTemplate;
use anyhow::Result;
use askama::Template;
use std::ffi::OsStr;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;
use tracing::debug;

pub struct NixShell {
    pub pure: bool,
    pub template: NixShellTemplate,
}

impl NixShell {
    pub fn enter(self) -> Result<()> {
        let shell_code = self.template.render()?;
        debug!("shell: {}", shell_code);
        let mut shell_file = NamedTempFile::new()?;
        shell_file.write_all(shell_code.as_bytes())?;

        let mut args: Vec<&OsStr> = vec![shell_file.path().as_os_str()];
        if self.pure {
            args.push(OsStr::new("--pure"));
        }
        debug!("args: {:?}", args);

        let status = Command::new("nix-shell").args(&args).status()?;
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
