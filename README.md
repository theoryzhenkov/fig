# fig

Stupid file manager: create files and directories with ease.

## Installation

```bash
cargo install --path .
```

## Usage

```bash
fig create [OPTIONS] [PATHS]...
fig c [OPTIONS] [PATHS]...
```

### Options

| Flag | Description |
|------|-------------|
| `-p` | Print created paths to stdout |
| `-t` | Create in a temporary directory |
| `-d` | Create directories instead of files |
| `-o [APP]` | Open created paths (with `$EDITOR` or specify app) |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

### Examples

```bash
# Create a file (parent directories are created automatically)
fig create src/utils/helpers.rs

# Create multiple files
fig c foo.txt bar.txt baz.txt

# Create a directory (use -d or trailing slash)
fig c -d my_project
fig c my_project/

# Create files in a temp directory
fig c -t scratch.txt notes.md
# Output: /tmp/.tmpXXXXXX/scratch.txt
#         /tmp/.tmpXXXXXX/notes.md

# Create and immediately open in editor
fig c -o draft.md

# Create and open with specific app
fig c -o=code src/main.rs

# Create temp file and print its path
fig c -t -p
# Output: /tmp/.tmpXXXXXX

# Combine flags: temp dir, print path, open in editor
fig c -tpo notes.md
```

## Configuration

fig can be configured via a config file or environment variables.

### Config File

Create `~/.config/fig/config.toml` (or `$XDG_CONFIG_HOME/fig/config.toml`):

```toml
# Custom subdirectory for temp files (under system temp dir)
tmpdir = "fig"
```

With this config, temp files will be created in `/tmp/fig/.tmpXXXXXX` instead of `/tmp/.tmpXXXXXX`.

### Environment Variables

| Variable | Description |
|----------|-------------|
| `FIG_TMPDIR` | Override the temp directory subdirectory (takes precedence over config file) |
| `EDITOR` | Editor to use with `-o` flag (defaults to `vi`) |

### Config Precedence

1. Environment variable `FIG_TMPDIR` (highest priority)
2. Config file `tmpdir` setting
3. System default temp directory (lowest priority)

## Tips

- **Trailing slash = directory**: `fig c foo/` creates a directory, even without `-d`
- **Auto-create parents**: `fig c deep/nested/path/file.txt` creates all parent directories
- **Temp mode prints by default**: `-t` implies `-p`, so paths are always printed
- **Combine with other tools**: `cd $(fig c -t -d)` to cd into a fresh temp directory

## License

MIT

