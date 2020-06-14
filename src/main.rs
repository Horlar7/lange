mod languages;
mod nix;
mod nix_shell;

use languages::Language;
use nix_shell::NixShell;
use structopt::StructOpt;

#[derive(Debug, structopt::StructOpt)]
#[structopt(name = "lange", about = "Quickly spin up a dev environment")]
struct Opt {
    #[structopt(short, long)]
    pure: bool,

    #[structopt(required = true)]
    languages: Vec<languages::Language>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    let shell = opt
        .languages
        .iter()
        .map(Language::into_shell)
        .sum::<nix::NixShellTemplate>();
    let shell = NixShell {
        pure: opt.pure,
        template: shell,
    };
    shell.enter()
}
