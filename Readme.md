# rust-git-cli

`rust-git-cli` is a Git-like command-line interface implemented in Rust. This project allows you to perform basic Git operations such as initializing a repository, creating and switching branches, committing changes, and more.

## Features

- Initialize a new Git repository
- Create and list branches
- Checkout branches
- View commit logs
- Add files to the staging area
- Check the status of the repository
- Create folders
- Commit changes with a message

## Installation

To use `rust-git-cli`, you need to have [Rust](https://www.rust-lang.org/) installed on your system. If Rust is not installed, you can install it using [rustup](https://rustup.rs/).

Clone the repository and navigate to the project directory:

```bash
git clone https://github.com/shivabhattacharjee/git-rust
cd git-rust
```

Build the project:

```bash
cargo build
```

## Usage

You can run the CLI commands using `cargo run -p rust-git-cli`. Below are the available commands:

### Initialize a Repository

```bash
cargo run -p rust-git-cli init
```

### List All Branches

```bash
cargo run -p rust-git-cli branch
```

### Create a Branch

```bash
cargo run -p rust-git-cli branch main
```

### Checkout a Branch

```bash
cargo run -p rust-git-cli checkout main
```

### View Commit Logs

```bash
cargo run -p rust-git-cli log
```

### Add Files to Staging Area

```bash
cargo run -p rust-git-cli add .
```

### Check Repository Status

```bash
cargo run -p rust-git-cli status
```

### Create a Folder

```bash
cargo run -p rust-git-cli create folder
```

### Commit Changes

```bash
cargo run -p rust-git-cli commit "Your commit message"
```

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue to discuss potential changes or improvements.

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/ShivaBhattacharjee/git-rust/blob/main/LICENSE) file for details.

