mod consts;
#[cfg(test)]
mod test;

use std::{env, fmt, fs};

use clap::Parser;
use indexmap::IndexMap;

use crate::consts::*;

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
    fields: IndexMap<String, String>, // preserve order
}

impl SingleConfig {
    pub fn new(name: &str, fields: &Vec<(&str, &str)>) -> Self {
        Self {
            name: name.to_string(),
            fields: fields
                .into_iter()
                .map(|(key, value)| (key.to_string(), value.to_string()))
                .collect::<_>(),
        }
    }
}

impl fmt::Display for SingleConfig {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut config = format!(
            r#"Host {}
"#,
            self.name
        );

        for (key, value) in &self.fields {
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
    let mut name = "default";
    let mut fields = Vec::new();

    while let Some(line) = lines.next() {
        let line = line.trim();
        if line.starts_with("Host ") {
            if !&fields.is_empty() {
                configs.push(SingleConfig::new(name, &fields));
            }
            name = line.split(' ').collect::<Vec<&str>>()[1];
            fields = Vec::new();
        } else if line.is_empty() {
            if !&fields.is_empty() {
                configs.push(SingleConfig::new(name, &fields));
            }
            fields = Vec::new();
        } else {
            let splits = if line.contains(' ') {
                line.splitn(2, ' ').collect::<Vec<&str>>()
            } else if line.contains('=') {
                line.splitn(2, '=').collect::<Vec<&str>>()
            } else {
                continue;
            };
            let key = splits[0];
            let value = splits[1];

            if line.starts_with("#") {
                continue;
            } else {
                fields.push((key, value));
            }
        }
    }
    if !&fields.is_empty() {
        configs.push(SingleConfig::new(name, &fields));
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
            println!("Failed to parse config! error={}", e);
            return;
        }
    };

    for config in &mut configs {
        if config.name == config_name {
            config.fields[HOST] = ip.to_string();
        }
    }

    if let Err(e) = write_config(config_path, configs) {
        println!("Failed to update config! error={}", e);
        return;
    }

    print_config(config_path);
}

fn main() {
    let args = Args::parse();

    let mut config_path = args.config_path;
    config_path = if config_path == CONFIG_PATH {
        let home = match env::var("HOME") {
            Ok(home) => home,
            Err(e) => {
                println!("Failed to get home directory! error={}", e);
                return;
            }
        };
        config_path.replace("~", &home)
    } else {
        config_path
    };

    match args.operation {
        Operation::UpdateIp { config_name, ip } => update_ip(&config_path, &config_name, &ip),
    }
}
