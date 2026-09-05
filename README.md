# himd

> Your shell remembers. `himd` makes sense of it.

`himd` is a fast command-line tool that reads your local Bash or Zsh history,
shows useful usage statistics, and searches previous commands. History stays on
your computer, and `himd` never executes commands from it.

Licensed under the MIT License.

## Features

- Total, unique, Git, Cargo, and Node command statistics
- Most-used and most-recent commands
- Case-insensitive history search
- Configurable search result limit
- Copy search results directly to the system clipboard
- Automatic GitHub release checks with a Homebrew upgrade hint

## Usage

```console
himd
himd stats
himd search git
himd search "git commit" --limit 10
himd search "cargo run" --limit 5 --copy
himd version
himd --help
```

Search reports the total number of matching history entries. `--limit` only
controls how many of the newest matches are displayed and copied.

## Install from source

Rust is required for this installation method:

```console
cargo install --git https://github.com/mkasaii16/history_cmd.git --locked
```

## Install a release binary on macOS

Download the archive matching your Mac from the GitHub Releases page:

- Apple Silicon: `himd-aarch64-apple-darwin.tar.gz`
- Intel: `himd-x86_64-apple-darwin.tar.gz`

Then extract and install it:

```console
tar -xzf himd-aarch64-apple-darwin.tar.gz
mkdir -p "$HOME/.local/bin"
install -m 755 himd "$HOME/.local/bin/himd"
himd --version
```

Make sure `$HOME/.local/bin` is included in your `PATH`.

## Homebrew

Install from the official `himd` tap:

```console
brew tap mkasaii16/tap
brew trust --formula mkasaii16/tap/himd
brew install mkasaii16/tap/himd
```

Homebrew requires explicit trust for third-party formulae. The command above
trusts only `himd`, not every current or future formula in the tap.

Update or uninstall later with:

```console
brew upgrade himd
brew uninstall himd
```

## Releasing a new version

1. Update `version` in `Cargo.toml`.
2. Run `cargo test --locked`.
3. Commit and push the changes.
4. Create and push a matching tag, such as `v0.0.4`.

```console
git tag v0.0.4
git push origin v0.0.4
```

The release workflow verifies that the tag matches `Cargo.toml`, builds Apple
Silicon and Intel macOS binaries, generates checksums, and publishes a GitHub
release. Installed copies check that release on every normal run. If a newer
version exists, `himd` prints:

```text
🚀 A new himd version is available: 0.0.1 → 0.0.2
✨ Update it with: brew update && brew upgrade himd
```
