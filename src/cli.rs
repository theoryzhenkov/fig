use clap::{Parser, Subcommand, ValueEnum, ValueHint};

const AFTER_HELP: &str = "\
CONFIGURATION:
    Config file: ~/.config/fig/config.toml (or $XDG_CONFIG_HOME/fig/config.toml)
    
    Example config:
        tmpdir = \"fig\"    # temp files go to /tmp/fig/.tmpXXXXXX
    
    Environment variables:
        FIG_TMPDIR     Override temp directory subdirectory
        EDITOR         Editor for -o flag (default: vi)

EXAMPLES:
    fig create src/main.rs     Create file (parents auto-created)
    fig c -d my_project/       Create directory
    fig c -t scratch.txt       Create temp file, print path
    fig c -o draft.md          Create and open in $EDITOR
    fig c -o=code file.rs      Create and open with specific app
    cd $(fig c -t -d)          cd into fresh temp directory";

#[derive(Parser)]
#[command(name = "fig")]
#[command(version)]
#[command(about = "Stupid file manager")]
#[command(after_help = AFTER_HELP)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create files and directories (default behavior)
    #[command(alias = "c")]
    Create(CreateArgs),

    Completions {
        #[arg(value_enum)]
        shell: CompletionShell,
    },
}

#[derive(Parser, Clone, Debug)]
pub struct CreateArgs {
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
    #[arg(trailing_var_arg = true, value_hint = ValueHint::Other)]
    pub paths: Vec<String>,
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum CompletionShell {
    Bash,
    Zsh,
    Fish,
}

impl CreateArgs {
    /// Returns true if open mode is enabled (-o flag was passed)
    pub fn open_mode(&self) -> bool {
        self.open_with.is_some()
    }

    /// Returns the app to open with, if specified
    pub fn open_app(&self) -> Option<String> {
        self.open_with.clone().flatten()
    }
}

