use clap::Parser;
use serde::Deserialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

#[derive(Parser)]
#[command(name = "slap")]
#[command(about = "Create files and directories with ease - touch, but slappier")]
struct Cli {
    /// Print created paths to stdout
    #[arg(short = 'p')]
    print_path: bool,

    /// Create in a temporary directory
    #[arg(short = 't')]
    temp_mode: bool,

    /// Create directories instead of files
    #[arg(short = 'd')]
    dir_mode: bool,

    /// Open created paths (with $EDITOR or specify app with -o=app)
    #[arg(short = 'o', value_name = "APP")]
    open_with: Option<Option<String>>,

    /// Paths to create
    #[arg(trailing_var_arg = true)]
    paths: Vec<String>,
}

#[derive(Deserialize, Default)]
struct Config {
    tmpdir: Option<String>,
}

impl Config {
    fn load() -> Self {
        let mut config = Self::from_file().unwrap_or_default();
        
        if let Ok(tmpdir) = env::var("SLAP_TMPDIR") {
            config.tmpdir = Some(tmpdir);
        }
        
        config
    }
    
    fn from_file() -> Option<Self> {
        let config_dir = env::var("XDG_CONFIG_HOME")
            .ok()
            .map(PathBuf::from)
            .or_else(|| dirs::home_dir().map(|h| h.join(".config")))?;
        
        let config_path = config_dir.join("slap").join("config.toml");
        
        let contents = fs::read_to_string(config_path).ok()?;
        toml::from_str(&contents).ok()
    }
}

fn create_temp_dir(config: &Config) -> std::io::Result<TempDir> {
    if let Some(ref tmpdir_name) = config.tmpdir {
        let base_tmpdir = env::temp_dir();
        let normalized_name = tmpdir_name.strip_prefix('/').unwrap_or(tmpdir_name);
        let custom_tmpdir = base_tmpdir.join(normalized_name);
        
        fs::create_dir_all(&custom_tmpdir)?;
        tempfile::Builder::new().tempdir_in(custom_tmpdir)
    } else {
        TempDir::new()
    }
}

fn create_temp_file(config: &Config) -> std::io::Result<tempfile::NamedTempFile> {
    if let Some(ref tmpdir_name) = config.tmpdir {
        let base_tmpdir = env::temp_dir();
        let normalized_name = tmpdir_name.strip_prefix('/').unwrap_or(tmpdir_name);
        let custom_tmpdir = base_tmpdir.join(normalized_name);
        
        fs::create_dir_all(&custom_tmpdir)?;
        tempfile::Builder::new().tempfile_in(custom_tmpdir)
    } else {
        tempfile::NamedTempFile::new()
    }
}

fn main() {
    let cli = Cli::parse();
    let config = Config::load();
    let open_mode = cli.open_with.is_some();
    let open_with = cli.open_with.flatten();

    let mut created: Vec<PathBuf> = Vec::new();

    if cli.temp_mode {
        if cli.paths.is_empty() {
            // No paths: just create a temp file or dir
            if cli.dir_mode {
                let dir = create_temp_dir(&config).expect("Failed to create temp directory");
                let path = dir.keep();
                created.push(path);
            } else {
                let file = create_temp_file(&config).expect("Failed to create temp file");
                let path = file.into_temp_path().keep().expect("Failed to persist temp file");
                created.push(path);
            }
        } else {
            // Create temp base directory, then structure inside
            let base = create_temp_dir(&config).expect("Failed to create temp directory");
            let base_path = base.keep();

            for path in &cli.paths {
                let full = base_path.join(path);

                if cli.dir_mode || path.ends_with('/') {
                    fs::create_dir_all(&full).expect("Failed to create directory");
                } else {
                    if let Some(parent) = full.parent() {
                        if parent != base_path {
                            fs::create_dir_all(parent).expect("Failed to create parent directory");
                        }
                    }
                    fs::File::create(&full).expect("Failed to create file");
                }
                created.push(full);
            }
        }
    } else {
        for path in &cli.paths {
            let path_buf = PathBuf::from(path);

            if cli.dir_mode || path.ends_with('/') {
                fs::create_dir_all(&path_buf).expect("Failed to create directory");
            } else {
                if let Some(parent) = path_buf.parent() {
                    if parent != PathBuf::from("") && parent != PathBuf::from(".") {
                        fs::create_dir_all(parent).expect("Failed to create parent directory");
                    }
                }
                fs::File::create(&path_buf).expect("Failed to create file");
            }
            created.push(path_buf);
        }
    }

    // Print paths if -p flag or temp mode
    if cli.print_path || cli.temp_mode {
        for p in &created {
            println!("{}", p.display());
        }
    }

    // Open files if -o flag
    if open_mode && !created.is_empty() {
        let paths: Vec<&str> = created.iter().map(|p| p.to_str().unwrap()).collect();

        if let Some(app) = open_with {
            Command::new("open")
                .arg("-a")
                .arg(&app)
                .args(&paths)
                .status()
                .expect("Failed to open with application");
        } else {
            let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
            Command::new(&editor)
                .args(&paths)
                .status()
                .expect("Failed to open with editor");
        }
    }
}
