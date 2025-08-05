# 🔐 Password Analyzer CLI

A command-line tool written in Rust that allows you to analyze password security, generate secure passwords, and detect if a password is common (based on wordlists).

## 🚀 Features

- Checks the strength of given passwords (length, special characters, etc.).
- Detects passwords that appear in common password files (`wordlist.txt`).
- Generates secure passwords with the most important criteria.
- User-friendly command-line interface using [`clap`](https://docs.rs/clap/).
- Output with colors and symbols for clear visualization (requires ANSI support).

## 📦 Installation

1. Clone the repository:

```bash
git clone https://github.com/RicardoUMC/Password-Analyzer-Cli.git
cd Password-Analyzer-Cli
```

2. Make sure you have [Rust](https://www.rust-lang.org/tools/install) installed.

3. Build the project:

```bash
cargo build --release
```

## 🧪 Usage

### Analyze a password

```bash
cargo run -- MySecureP@ssw0rd!
```

### Generate a secure password

```bash
cargo run -- -g
```

or using the following form:

```bash
cargo run -- --generate
```

### Analyze passwords from a file

Suppose you have a list in `wordlist.txt` and want to check if the password is included:

```bash
cargo run -- -c wordlist.txt "123456"
```

or using the long form of the option:

```bash
cargo run -- -common wordlist.txt "123456"
```

## 📁 Project Structure

- `src/main.rs` – Main entry point of the program.
- `src/cli/` – Module responsible for handling the command-line interface and argument processing.
- `src/common/` – Module for loading common passwords (`wordlist.txt`).
- `src/password/` – Module containing the core logic for password analysis and generation.
- `src/utils/` – Module with various auxiliary functions such as screen prints.
- `wordlist.txt` – Optional wordlist of common passwords, upload your own file.

## 🧱 Technologies Used

- Rust
- [clap](https://crates.io/crates/clap) – CLI parsing
- [regex](https://crates.io/crates/regex) – Pattern validation
- [colored](https://crates.io/crates/colored) – Terminal colors

## ✅ Output Examples

```bash
Received password: MySecureP@ssw0rd!
--- Security Analysis ---

✓ Valid length (>=10)
✓ Has uppercase
✓ Has lowercase
✓ Has numbers
✓ Has symbols

Password strength: [■■■■■■■■■■] 100% (Strong)
```

## 📄 MIT License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

## ✨ Credits

Developed by [Ricardo Mora](https://github.com/RicardoUMC).
