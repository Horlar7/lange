use crate::nix::NixShellTemplate;

#[derive(Debug, strum_macros::EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum Language {
    Bash,
    Rust,
}

impl Language {
    pub fn into_shell(&self) -> NixShellTemplate {
        use Language::*;
        match self {
            Bash => NixShellTemplate {
                name: "Bash".into(),
                build_inputs: ["shellcheck", "shfmt"].as_ref().into(),
                ..Default::default()
            },
            Rust => NixShellTemplate {
                name: "Rust".into(),
                build_inputs: ["cargo", "rust-analyzer"].as_ref().into(),
                ..Default::default()
            },
        }
    }
}
