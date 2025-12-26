mod cli;
mod completions;
mod config;
mod create;
mod open;

use clap::Parser;

use cli::{Cli, Commands, CreateArgs};
use config::Config;

fn main() {
    // Prevent scary panics when stdout is a pipe and the reader exits early (e.g. `fig completions fish | head`).
    // This matches the behavior of many Unix CLIs: writing to a closed pipe terminates the process via SIGPIPE.
    #[cfg(unix)]
    unsafe {
        libc::signal(libc::SIGPIPE, libc::SIG_DFL);
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Create(args) => run_create(args),
        Commands::Completions { shell } => completions::print(shell),
    }
}

fn run_create(args: CreateArgs) {
    let config = Config::load();

    if args.paths.is_empty() && !args.temp_mode {
        eprintln!("fig create: no paths provided");
        eprintln!("For more information, try 'fig create --help'");
        std::process::exit(1);
    }

    let created = if args.temp_mode {
        create::create_temp_paths(&config, &args.paths, args.dir_mode).expect("Failed to create paths")
    } else {
        create::create_paths(&args.paths, args.dir_mode).expect("Failed to create paths")
    };

    if args.print_path || args.temp_mode {
        for p in &created {
            println!("{}", p.display());
        }
    }

    if args.open_mode() && !created.is_empty() {
        open::warn_about_directories(&created);
        open::open_paths(&created, args.open_app()).expect("Failed to open paths");
    }
}
