mod languages;
mod nix;
mod nix_shell;

use nix_shell::NixShell;
use structopt::StructOpt;
use tracing::{debug, info};
use tracing_subscriber;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "lange", about = "Quickly spin up a dev environment")]
struct Opt {
    #[structopt(short, long)]
    pure: bool,

    #[structopt(required = true)]
    languages: Vec<languages::Language>,
}

fn main() -> anyhow::Result<()> {
    let _subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .init();

    let opt = Opt::from_args();
    debug!("Options: {:?}", opt);
    let shell = opt
        .languages
        .iter()
        .map(Into::into)
        .sum::<nix::NixShellTemplate>();
    debug!("Shell Code: {}", shell);
    let shell = NixShell {
        pure: opt.pure,
        template: shell,
    };
    shell.enter()
}
