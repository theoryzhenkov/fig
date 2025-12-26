use clap::{Parser, Subcommand, ValueEnum};

const AFTER_HELP: &str = "\
CONFIGURATION:
    Config file: ~/.config/slap/config.toml (or $XDG_CONFIG_HOME/slap/config.toml)
    
    Example config:
        tmpdir = \"slap\"    # temp files go to /tmp/slap/.tmpXXXXXX
    
    Environment variables:
        SLAP_TMPDIR    Override temp directory subdirectory
        EDITOR         Editor for -o flag (default: vi)

EXAMPLES:
    slap src/main.rs           Create file (parents auto-created)
    slap -d my_project/        Create directory
    slap -t scratch.txt        Create temp file, print path
    slap -o draft.md           Create and open in $EDITOR
    slap -o=code file.rs       Create and open with specific app
    cd $(slap -t -d)           cd into fresh temp directory";

#[derive(Parser)]
#[command(name = "slap")]
#[command(version)]
#[command(about = "Create files and directories with ease - touch, but slappier")]
#[command(after_help = AFTER_HELP)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Print created paths to stdout
    #[arg(short = 'p')]
    pub print_path: bool,

    /// Create in a temporary directory
    #[arg(short = 't')]
    pub temp_mode: bool,

    /// Create directories instead of files
    #[arg(short = 'd')]
    pub dir_mode: bool,

    /// Open created paths (with $EDITOR or specify app with -o=app)
    #[arg(short = 'o', value_name = "APP")]
    pub open_with: Option<Option<String>>,

    /// Paths to create
    #[arg(trailing_var_arg = true)]
    pub paths: Vec<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generate shell completion scripts (prints to stdout)
    Completions {
        #[arg(value_enum)]
        shell: CompletionShell,
    },
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
}

impl Cli {
    /// Returns true if open mode is enabled (-o flag was passed)
    pub fn open_mode(&self) -> bool {
        self.open_with.is_some()
    }

    /// Returns the app to open with, if specified
    pub fn open_app(&self) -> Option<String> {
        self.open_with.clone().flatten()
    }
}

