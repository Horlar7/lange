mod nix;

use nix::NixShellTemplateBuilder;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rust_shell = NixShellTemplateBuilder::default()
        .name("Rust")
        .build_inputs(&["rust-analyzer", "cargo"][..])
        .shell_hook("echo 'foo'")
        .build()?
        .render();
    let mut tmp_shell_file = NamedTempFile::new()?;
    tmp_shell_file.write_all(rust_shell.as_bytes())?;
    Command::new("nix-shell")
        .arg(tmp_shell_file.path())
        .status()?;
    Ok(())
}
