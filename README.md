<!-- cargo-rdme start -->

# simplesql
**simplesql** is a modern, lightweight SQL client that runs in either optional Terminal (TUI) or Graphical (GUI) mode. Built for developers, DBAs, and power users who need fast, intuitive access to their databases.

## 🔧 Features

- Optional ⚡ Fast and responsive Terminal User Interface (TUI)
- 🖼️ Graphical User Interface (GUI) for a more visual experience [WIP]
- 🛠️ Simple command-line controls
- 🔁 Cross-platform support: Linux, macOS, Windows, FreeBSD

## 🚀 Installation

Clone the repository and build it using Cargo:

```bash
git clone https://github.com/comboomPunkTsucht/simplesql.git
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
| `-g`  | `--gui`     | Launch **simplesql** in graphical mode (default)      |
| `-t`, `-c` | `--tui`, `--cli`| Launch in terminal mode                      |
| `-h`  | `--help`    | Show help message                                     |
| `-V`  | `--version` | Show version info                                     |

## 🧪 Example

```bash
./simplesql --tui
```

## Special Behavior
The application will automatically detect if it is running in a terminal or not. If it is running in a terminal, it will default to TUI mode unless the `--gui` flag is set. If it is not running in a terminal, it will default to GUI mode.
The Terminal might be appearing in the background, but it is only for logging in GUI Mode.
The .app/.desktop/.link starts the Programm automatikly in GUI mode.

## 📄 Changelog

The `Changelog.md` file is generated during the build process and included with each release.

## 📝 License

Licensed under the [MIT License](LICENSE).

---

Made with ❤️ in Rust – because SQL should be simple.

<!-- cargo-rdme end -->
