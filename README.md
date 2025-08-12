# AutoLeet

AutoLeet is a CLI tool that instantly sets up Rust project environments for solving LeetCode problems. With this tool, you can specify a problem number and title to automatically generate a consistently structured Cargo project, allowing you to focus immediately on coding.

## ✨ Features

- **Instant Setup**: Create a new LeetCode problem project with a single command
- **Consistent Structure**: All generated projects have a clean architecture with separated `solver.rs` (for solution logic) and `main.rs` (for local testing)
- **Template-Driven**: Uses the Tera template engine, making it easy to customize the scaffolding of generated files
- **Safe Development**: Use the `--dry-run` flag to safely preview what directories and files will be created without actually writing to the filesystem

## 📦 Installation

1. Clone this repository:
   ```bash
   git clone https://github.com/ragman53/autoleet.git
   ```

2. Navigate to the cloned directory:
   ```bash
   cd autoleet
   ```

3. Build using `cargo`:
   ```bash
   cargo build --release
   ```

4. The executable will be created at `target/release/autoleet`. Either add it to your PATH or run it directly via `cargo run`.

## 🚀 Usage

AutoLeet takes two arguments: `index` (`-i`) and `title` (`-t`).

```bash
cargo run -- -i <problem_number> -t "<problem_title>"
```

### Example

To create a project for LeetCode problem #13 "Roman to Integer":

```bash
cargo run -- -i 13 -t "Roman to Integer"
```

This command creates a new folder named `p0013-roman-to-integer` in the current directory, with all necessary files ready for coding:

```
.
└── p0013-roman-to-integer/
    ├── src/
    │   ├── main.rs      # Local test runner
    │   └── solver.rs    # Write your solution logic here
    ├── .gitignore
    └── Cargo.toml
```

### Dry Run Mode

When developing or when you just want to see what files will be generated, use the `--dry-run` flag. This previews the execution results without writing anything to the filesystem:

```bash
cargo run -- -i 13 -t "Roman to Integer" --dry-run
```

**Output:**
```
---Dry Run Mode !!!---
No.13 - "Roman to Integer"
Args { index: 13, title: "Roman to Integer", dry_run: true }
Directory name: p0013-roman-to-integer
Processing template: Cargo.toml.tera
[Dry Run] would create directory: "p0013-roman-to-integer/Cargo.toml"
File Operation was skipped: "p0013-roman-to-integer/Cargo.toml"
...
```

## 🔧 Customization

The heart of this tool is the `templates/` folder in the root directory. You can freely modify the project structure and content by editing the `.tera` files within:

- **`templates/Cargo.toml.tera`**: Add or modify dependency crates
- **`templates/src/main.rs.tera`**: Customize the local test runner behavior
- **`templates/src/solver.rs.tera`**: Modify the initial template for solution files

## 📁 Project Structure

```
autoleet/
├── src/
│   └── main.rs              # CLI application logic
├── templates/               # Template files for project generation
│   ├── Cargo.toml.tera
│   └── src/
│       ├── main.rs.tera
│       └── solver.rs.tera
├── Cargo.toml
└── README.md
```

## 📄 License

This project is open source and available under the [MIT License](LICENSE).
