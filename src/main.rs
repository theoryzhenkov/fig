mod cli;
mod completions;
mod config;
mod create;
mod open;

use clap::Parser;

use cli::{Cli, Commands};
use config::Config;

fn main() {
    // Prevent scary panics when stdout is a pipe and the reader exits early (e.g. `slap completions fish | head`).
    // This matches the behavior of many Unix CLIs: writing to a closed pipe terminates the process via SIGPIPE.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let cli = Cli::parse();

    if let Some(Commands::Completions { shell }) = cli.command {
        completions::print(shell);
        return;
    }

    let config = Config::load();

    // If no paths provided and not in temp mode, print usage and exit
    if cli.paths.is_empty() && !cli.temp_mode {
        eprintln!("slap: no paths provided");
        eprintln!("Usage:");
        eprintln!("  slap [OPTIONS] [PATHS]...");
        eprintln!("  slap completions <bash|zsh|fish>");
        eprintln!();
        eprintln!("Options:");
        eprintln!("  -p          Print created paths to stdout");
        eprintln!("  -t          Create in a temporary directory");
        eprintln!("  -d          Create directories instead of files");
        eprintln!("  -o [APP]    Open created paths (with $EDITOR or specify app)");
        eprintln!();
        eprintln!("For more information, try 'slap --help'");
        std::process::exit(1);
    }

    // Create paths
    let created = if cli.temp_mode {
        create::create_temp_paths(&config, &cli.paths, cli.dir_mode)
            .expect("Failed to create paths")
    } else {
        create::create_paths(&cli.paths, cli.dir_mode)
            .expect("Failed to create paths")
    };

    // Print paths if -p flag or temp mode
    if cli.print_path || cli.temp_mode {
        for p in &created {
            println!("{}", p.display());
        }
    }

    // Open files if -o flag
    if cli.open_mode() && !created.is_empty() {
        // Warn about directories being opened
        open::warn_about_directories(&created);

        open::open_paths(&created, cli.open_app())
            .expect("Failed to open paths");
    }
}
