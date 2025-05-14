# simplesql

**simplesql** is a modern, lightweight SQL client that runs in either Terminal (TUI) or optional Graphical (GUI) mode. Built for developers, DBAs, and power users who need fast, intuitive access to their databases.

## 🔧 Features

- ⚡ Fast and responsive Terminal User Interface (TUI)
- 🖼️ Optional Graphical User Interface (GUI) for a more visual experience
- 🛠️ Simple command-line controls
- 🔁 Cross-platform support: Linux, macOS, Windows, FreeBSD

## 🚀 Installation

Clone the repository and build it using Cargo:

```bash
git clone https://github.com/your-username/simplesql.git
cd simplesql
rustup target add aarch64-apple-darwin aarch64-unknown-linux-gnu aarch64-unknown-linux-musl aarch64-pc-windows-msvc x86_64-apple-darwin x86_64-pc-windows-msvc x86_64-unknown-freebsd x86_64-unknown-linux-gnu x86_64-unknown-linux-musl
cargo build --release
```

## ▶️ Usage

```bash
./simplesql [OPTIONS]
```

### Options

| Short | Long        | Description                                           |
|-------|-------------|-------------------------------------------------------|
| `-g`  | `--gui`     | Launch **simplesql** in graphical mode                |
| `-t`, `-c` | `--tui`| Launch in terminal mode (default)                     |
| `-h`  | `--help`    | Show help message                                     |
| `-V`  | `--version` | Show version info                                     |

## 🧪 Example

```bash
./simplesql --tui
```

## 📄 Changelog

The `Changelog.md` file is generated during the build process and included with each release.

## 📝 License

Licensed under the [MIT License](LICENSE).

---

Made with ❤️ in Rust – because SQL should be simple.
