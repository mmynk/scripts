#[cfg(test)]
mod test;

use std::{collections::BTreeMap, fmt, fs};

use clap::Parser;

const CONFIG_PATH: &str = "/Users/mohit/.ssh/config";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Operation to perform.
    #[command(subcommand)]
    operation: Operation,

    /// Path to the config file.
    #[arg(short, long, default_value = CONFIG_PATH)]
    config_path: String,
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
#[derive(Debug)]
struct SingleConfig {
    name: String,
    user: String,
    host: String,
    raw_fields: BTreeMap<String, String>,
}

impl SingleConfig {
    pub fn new(name: &str, host: &str, user: &str, raw_fields: BTreeMap<&str, &str>) -> Self {
        Self {
            name: name.to_string(),
            host: host.to_string(),
            user: user.to_string(),
            raw_fields: raw_fields
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<BTreeMap<String, String>>(),
        }
    }
}

impl fmt::Display for SingleConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut config = format!(
            r#"Host {}
    User {}
    Hostname {}
"#,
            self.name, self.user, self.host
        );

        for (key, value) in &self.raw_fields {
            config.push_str(&format!("    {} {}\n", key, value));
        }

        write!(f, "{}", config)
    }
}

type Config = Vec<SingleConfig>;

/// Parse SSH configs into a Vec of `Config`.
fn parse_config(config_path: &str) -> Result<Config, String> {
    let mut configs = Vec::new();

    let contents = match fs::read_to_string(config_path) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("Failed to read config file: error={}", e)),
    };

    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.starts_with("Host") {
            line.split_whitespace().nth(1).and_then(|name| {
                let mut user = "";
                let mut host = "";
                let mut raw_fields = BTreeMap::new();

                loop {
                    let line = lines.next();
                    if line.is_none() {
                        configs.push(SingleConfig::new(name, host, user, raw_fields));
                        break;
                    }
                    let line = line.unwrap().trim();
                    if line.is_empty() {
                        configs.push(SingleConfig::new(name, host, user, raw_fields));
                        break;
                    }
                    if line.starts_with("User") {
                        line.split_whitespace()
                            .nth(1)
                            .map(|u| user = u)
                            .ok_or_else(|| format!("Failed to parse user"))
                            .ok()?;
                    } else if line.starts_with("Hostname") {
                        line.split_whitespace()
                            .nth(1)
                            .map(|h| host = h)
                            .ok_or_else(|| format!("Failed to parse host"))
                            .ok()?;
                    } else {
                        let splits = line.splitn(2, ' ').collect::<Vec<&str>>();
                        raw_fields.insert(splits[0], splits[1]);
                    }
                }

                Some(())
            });
        }
    }

    Ok(configs)
}

/// Write configs to file.
fn write_config(config_path: &str, config: Config) -> Result<(), String> {
    let contents = config
        .into_iter()
        .map(|config| config.to_string())
        .collect::<Vec<String>>()
        .join("\n");

    match fs::write(config_path, contents) {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to write config file: error={}", e)),
    }
}

/// Print the entire SSH config.
fn print_config(config_path: &str) {
    match fs::read_to_string(config_path) {
        Ok(contents) => println!("{contents}"),
        Err(e) => println!("Failed to read config file: error={}", e),
    }
}

/// Update the IP address of a config.
fn update_ip(config_path: &str, config_name: &str, ip: &str) {
    let mut configs = match parse_config(config_path) {
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

    if let Err(e) = write_config(config_path, configs) {
        println!("Failed to update config! Error: {}", e);
        return;
    }

    print_config(config_path);
}

fn main() {
    // update_ip("playground", "3.141.20.157");
    let args = Args::parse();

    match args.operation {
        Operation::UpdateIp { config_name, ip } => update_ip(&args.config_path, &config_name, &ip),
    }
}
