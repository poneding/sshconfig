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
