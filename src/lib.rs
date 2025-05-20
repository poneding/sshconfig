use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct HostEntry {
    pub name: String,
    pub host: String,
    pub port: Option<u16>,
    pub user: String,
    pub identity_file: Option<String>,
}

impl HostEntry {
    pub fn new(host: String) -> Self {
        Self {
            name: host.clone(),
            host,
            port: Some(22),
            user: "root".to_string(),
            identity_file: Some("~/.ssh/id_rsa".to_string()),
        }
    }
}

/// Parses an SSH config file and returns a vector of HostEntry structs.
pub fn parse_ssh_config<P: AsRef<str>>(path: P) -> io::Result<Vec<HostEntry>> {
    let file = File::open(shellexpand::tilde(path.as_ref()).into_owned())?;
    let reader = BufReader::new(file);

    let mut entries = Vec::new();
    let mut current_entry: Option<HostEntry> = None;

    for line in reader.lines() {
        let line = line?;
        let line = line.trim();

        if line.is_empty() || line.starts_with("#") {
            continue;
        }

        let parts: Vec<&str> = line.splitn(2, ' ').collect();
        if parts.len() != 2 {
            continue;
        }

        let keyword = parts[0].to_lowercase();

        if keyword == "host" {
            if let Some(config) = current_entry.take() {
                entries.push(config);
            }

            current_entry = Some(HostEntry::new(parts[1].trim().to_string()));
        } else if let Some(entry) = &mut current_entry {
            if parts.len() >= 2 {
                let value = parts[1].trim().to_string();

                match keyword.as_str() {
                    "hostname" => entry.host = value,
                    "port" => {
                        entry.port = value.parse::<u16>().ok();
                    }
                    "user" => entry.user = value,
                    "identityfile" => entry.identity_file = Some(value),
                    _ => {}
                }
            }
        }
    }

    if let Some(entry) = current_entry {
        entries.push(entry);
    }

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_ssh_config() -> io::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "# SSH Config Example")?;
        writeln!(file, "Host github.com")?;
        writeln!(file, "    HostName github.com")?;
        writeln!(file, "    User git")?;
        writeln!(file, "    IdentityFile ~/.ssh/github_rsa")?;
        writeln!(file, "    Port 22")?;
        writeln!(file)?;
        writeln!(file, "Host example.com")?;
        writeln!(file, "    HostName example.org")?;
        writeln!(file, "    User admin")?;
        writeln!(file, "    Port 2222")?;
        file.flush()?;

        let entries = parse_ssh_config(file.path().to_string_lossy())?;

        assert_eq!(entries.len(), 2);

        assert_eq!(entries[0].host, "github.com");
        assert_eq!(entries[0].name, "github.com");
        assert_eq!(entries[0].user, "git");
        assert_eq!(
            entries[0].identity_file,
            Some("~/.ssh/github_rsa".to_string())
        );
        assert_eq!(entries[0].port, Some(22));

        assert_eq!(entries[1].host, "example.org");
        assert_eq!(entries[1].name, "example.com");
        assert_eq!(entries[1].user, "admin");
        assert_eq!(entries[1].port, Some(2222));
        assert_eq!(entries[1].identity_file, Some("~/.ssh/id_rsa".to_string()));
        Ok(())
    }

    #[test]
    fn test_default_values() -> io::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Host myserver")?;
        file.flush()?;

        let entries = parse_ssh_config(file.path().to_string_lossy())?;

        assert_eq!(entries.len(), 1);

        assert_eq!(entries[0].host, "myserver");
        assert_eq!(entries[0].name, "myserver");
        assert_eq!(entries[0].user, "root");
        assert_eq!(entries[0].port, Some(22));
        assert_eq!(entries[0].identity_file, Some("~/.ssh/id_rsa".to_string()));

        Ok(())
    }

    #[test]
    fn test_multiple_hosts() -> io::Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "Host server1 server2 server3")?;
        writeln!(file, "    User shared")?;
        writeln!(file, "    Port 2222")?;
        file.flush()?;

        let entries = parse_ssh_config(file.path().to_string_lossy())?;

        assert_eq!(entries.len(), 1);

        assert_eq!(entries[0].host, "server1 server2 server3");
        assert_eq!(entries[0].user, "shared");
        assert_eq!(entries[0].port, Some(2222));

        Ok(())
    }
}
