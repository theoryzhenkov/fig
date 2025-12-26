use std::io::{self, Write};

use clap::CommandFactory;
use clap_complete::{generate, shells};

use crate::cli::{Cli, CompletionShell};

pub fn print(shell: CompletionShell) {
    let mut cmd = Cli::command();
    let bin_name = cmd.get_name().to_string();

    match shell {
        CompletionShell::Bash => generate(shells::Bash, &mut cmd, bin_name, &mut io::stdout()),
        CompletionShell::Zsh => generate(shells::Zsh, &mut cmd, bin_name, &mut io::stdout()),
        CompletionShell::Fish => {
            let mut buf = Vec::new();
            generate(shells::Fish, &mut cmd, &bin_name, &mut buf);
            let script = String::from_utf8_lossy(&buf);

            for line in script.lines() {
                // Disable file completion for fig by injecting fish's `-f` on every completion line.
                if line.starts_with("complete -c fig") && !line.contains(" -f") {
                    let _ = writeln!(
                        io::stdout(),
                        "{}",
                        line.replacen("complete -c fig", "complete -c fig -f", 1)
                    );
                } else {
                    let _ = writeln!(io::stdout(), "{}", line);
                }
            }
        }
    }
}