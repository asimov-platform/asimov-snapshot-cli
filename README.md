# ASIMOV Snapshot Command-Line Interface (CLI)

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/asimov-snapshot-cli)](https://crates.io/crates/asimov-snapshot-cli)

🚧 _We are building in public. This is presently under heavy construction._

## ✨ Features

## 🛠️ Prerequisites

- [Rust](https://rust-lang.org) 1.85+ (2024 edition)

## ⬇️ Installation

The intended installation method is through Homebrew.

### Installation via Homebrew

Snapshot CLI can be installed along [ASIMOV CLI] through Homebrew:

```bash
brew tap asimov-platform/tap
brew install asimov-cli # or just asimov-snapshot-cli
```

#### Installation via Cargo

```bash
cargo install asimov-snapshot-cli --version 25.0.0-dev.4
```

## 👉 Examples

The binary can be invoked either through [ASIMOV CLI] with `asimov snapshot <cmd>` or directly as `asimov-snapshot <cmd>`.

### Create a snapshot of a URL

```bash
asimov-snapshot https://getasimov.ai
```

### List previously saved URLs

```console
$ asimov-snapshot list -v
https://getasimov.ai/ (last updated one minute ago)
```

### List saved versions of a URL

```console
$ asimov-snapshot log https://getasimov.ai
ed82093b (one minute ago)
```

### Compact storage by removing previous versions

```bash
asimov-snapshot compact
```

## 📚 Reference

TBD

## 👨‍💻 Development

```bash
git clone https://github.com/asimov-platform/asimov-snapshot-cli.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/asimov-platform/asimov-snapshot-cli&text=ASIMOV%20Snapshot%20Command-Line%20Interface%20%28CLI%29)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/asimov-platform/asimov-snapshot-cli&title=ASIMOV%20Snapshot%20Command-Line%20Interface%20%28CLI%29)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/asimov-platform/asimov-snapshot-cli&t=ASIMOV%20Snapshot%20Command-Line%20Interface%20%28CLI%29)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/asimov-platform/asimov-snapshot-cli)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/asimov-platform/asimov-snapshot-cli)

[ASIMOV CLI]: https://github.com/asimov-platform/asimov-cli
