# gh-issues-dl

Writes all the github issues for a repo to a file. Good for offline backups of important work.

## Usage

### Executables

Get from [releases page](https://github.com/ChristopherAyling/gh-issues-dl/releases)

### From Source

```
git clone https://github.com/ChristopherAyling/gh-issues-dl.git
cd gh-issues-dl
cargo build --release

./target/release/gh-issues-dl --help
./target/release/gh-issues-dl --repo=ChristopherAyling/gh-issues-dl
```