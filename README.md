# sshconfig

A Rust library for parsing SSH config files into a structured format in Rust.

## Installation

Add `sshconfig` crate to your cargo project:

```shell
cargo add sshconfig
```

## Usage

### Basic Usage

```rust
use sshconfig::parse_ssh_config;
use std::io;

fn main() -> io::Result<()> {
    // Parse an SSH config file
    let entries = parse_ssh_config("./examples/parse-ssh-config/example_config")?;

    // Process the entries
    for entry in entries {
        println!("name: {}", entry.name);
        println!("host: {}", entry.host);
        println!("user: {}", entry.user);
        println!("port: {}", entry.port.unwrap_or(22));
        println!(
            "identity file: {}",
            entry.identity_file.unwrap_or("~/.ssh/id_rsa".to_string())
        );
    }

    Ok(())
}
```

## HostEntry Structure

Each `HostEntry` object represents a Host entry in the SSH config file and contains:

- `name`: The host alias name
- `host`: The actual hostname or IP address
- `port`: The port number (default: 22)
- `user`: The username (default: "root")
- `identity_file`: The identity file path (default: "~/.ssh/id_rsa")

## License

This project is licensed under the [MIT](./LICENSE-MIT) or [Apache-2.0](./LICENSE-APACHE) license.
