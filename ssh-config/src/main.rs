use std::{fs, fmt};

use clap::Parser;

const CONFIG_PATH: &str = "/Users/mohit/.ssh/config";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Operation to perform.
    #[command(subcommand)]
    operation: Operation,
}

#[derive(Parser, Debug)]
enum Operation {
    /// Update the IP address of a config.
    UpdateIp {
        /// Name of the config.
        #[arg(short, long)]
        config_name: String,

        /// New IP address.
        #[arg(short, long)]
        ip: String,
    },
}

/// `SingleConfig` represents a single ssh config.
#[derive(Debug, Default)]
struct SingleConfig {
    name: String,
    user: String,
    host: String,
    identity_file: Option<String>,
    server_alive_interval: Option<u32>,
    server_alive_count_max: Option<u32>,
}

impl fmt::Display for SingleConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut config = format!(
r#"Host {}
    User {}
    Hostname {}
"#, self.name, self.user, self.host);

        if let Some(identity_file) = &self.identity_file {
            config.push_str(&format!("    IdentityFile {}\n", identity_file));
        }

        if let Some(server_alive_interval) = &self.server_alive_interval {
            config.push_str(&format!("    ServerAliveInterval {}\n", server_alive_interval));
        }

        if let Some(server_alive_count_max) = &self.server_alive_count_max {
            config.push_str(&format!("    ServerAliveCountMax {}\n", server_alive_count_max));
        }

        write!(f, "{}", config)
    }
}

type Config = Vec<SingleConfig>;

/// Parse SSH configs into a Vec of `Config`.
fn parse_config() -> Result<Config, String> {
    let mut configs = Vec::new();

    let contents = match fs::read_to_string(CONFIG_PATH) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("Failed to read config file: error={}", e)),
    };

    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.starts_with("Host") {
            line.split_whitespace().nth(1)
            .and_then(|name| {
                let mut config = SingleConfig::default();
                config.name = name.to_string();
                loop {
                    let line = lines.next();
                    if line.is_none() {
                        configs.push(config);
                        break;
                    }
                    let line = line.unwrap().trim();
                    if line.is_empty() {
                        configs.push(config);
                        break;
                    }
                    if line.starts_with("User") {
                        line.split_whitespace().nth(1)
                            .map(|user| config.user = user.to_string())
                            .ok_or_else(|| format!("Failed to parse user"))
                            .ok()?;
                    } else if line.starts_with("Hostname") {
                        line.split_whitespace().nth(1)
                            .map(|host| config.host = host.to_string())
                            .ok_or_else(|| format!("Failed to parse host"))
                            .ok()?;
                    } else if line.starts_with("IdentityFile") {
                        line.split_whitespace().nth(1)
                            .map(|identity_file| config.identity_file = Some(identity_file.to_string()));
                    } else if line.starts_with("ServerAliveInterval") {
                        line.split_whitespace().nth(1)
                            .and_then(|server_alive_interval| server_alive_interval.parse::<u32>().ok())
                            .map(|server_alive_interval| config.server_alive_interval = Some(server_alive_interval));
                    } else if line.starts_with("ServerAliveCountMax") {
                        line.split_whitespace().nth(1)
                            .and_then(|server_alive_count_max| server_alive_count_max.parse::<u32>().ok())
                            .map(|server_alive_count_max| config.server_alive_count_max = Some(server_alive_count_max));
                    }
                }

                Some(())
            });
        }
    }

    Ok(configs)
}

/// Write configs to file.
fn write_config(config: Config) -> Result<(), String> {
    let contents = config.into_iter()
        .map(|config| config.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    match fs::write(CONFIG_PATH, contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write config file: error={}", e)),
    }
}

/// Print the entire SSH config.
fn print_config() {
    match fs::read_to_string(CONFIG_PATH) {
        Ok(contents) => println!("{contents}"),
        Err(e) => println!("Failed to read config file: error={}", e),
    }
}


/// Update the IP address of a config.
fn update_ip(config_name: &str, ip: &str) {
    let mut configs = match parse_config() {
        Ok(configs) => configs,
        Err(e) => {
            println!("Error: {}", e);
            return;
        }
    };

    for config in &mut configs {
        if config.name == config_name {
            config.host = ip.to_string();
        }
    }

    if let Err(e) = write_config(configs) {
        println!("Failed to update config! Error: {}", e);
        return;
    } else {
        print_config();
    }
}

fn main() {
    // update_ip("playground", "3.141.20.157");
    let args = Args::parse();

    match args.operation {
        Operation::UpdateIp { config_name, ip } => update_ip(&config_name, &ip),
    }
}
