use crate::nix::NixShellTemplate;
use askama::Template;
use std::{ffi::OsStr, io::Write, process::Command};

#[derive(Debug)]
pub struct NixShell {
    pub pure: bool,
    pub template: NixShellTemplate,
}

impl NixShell {
    #[tracing::instrument]
    pub fn enter(self) -> anyhow::Result<()> {
        let shell_code = self.template.render()?;
        let mut shell_file = tempfile::NamedTempFile::new()?;
        shell_file.write_all(shell_code.as_bytes())?;

        let mut args: Vec<&OsStr> = vec![shell_file.path().as_os_str()];
        if self.pure {
            args.push(OsStr::new("--pure"));
        }

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
