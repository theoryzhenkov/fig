use std::io;

use clap::CommandFactory;
use clap_complete::{generate, shells};

use crate::cli::{Cli, CompletionShell};

pub fn print(shell: CompletionShell) {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();
    let mut out = io::stdout();

    match shell {
        CompletionShell::Bash => generate(shells::Bash, &mut cmd, bin_name, &mut out),
        CompletionShell::Zsh => generate(shells::Zsh, &mut cmd, bin_name, &mut out),
        CompletionShell::Fish => generate(shells::Fish, &mut cmd, bin_name, &mut out),
    }
}