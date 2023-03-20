# gh-issues-dl

Writes all the github issues for a repo to a file. Good for offline backups of important work.

## Usage

### Executables

[macos](https://github.com/ChristopherAyling/gh-issues-dl/releases/download/mvp/gh-issues-dl)

### From Source

```
git clone https://github.com/ChristopherAyling/gh-issues-dl.git
cd gh-issues-dl
cargo build --release

./target/release/gh-issues-dl --help
./target/release/gh-issues-dl --repo=ChristopherAyling/gh-issues-dl
```