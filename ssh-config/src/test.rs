use super::*;

/// Create a temporary config file for testing.
fn setup() {
    let configs = vec![
        SingleConfig::new(
            "config1",
            &vec![
                ("Hostname", "1.3.3.7"),
                ("IdentityFile", "~/.ssh/id_rsa"),
                ("Port", "22"),
                ("ServerAliveCountMax", "2"),
                ("ServerAliveInterval", "60"),
            ]
        ),
        SingleConfig::new("config2", &vec![("Hostname", "0.4.2.0")]),
        SingleConfig::new("config3", &Vec::new()),
    ];
    let _ = write_config("src/config", configs);
}

/// Remove the temporary config file.
fn teardown() {
    let _ = fs::remove_file("src/config");
}

#[cfg(test)]
#[test]
fn test_update_ip() {
    setup();

    let config_path = "src/config";
    let config_name = "config2";
    let ip = "0.0.4.2";

    update_ip(config_path, config_name, ip);
    let configs = parse_config(config_path).unwrap();

    assert_eq!(configs.len(), 2);
    assert_eq!(configs[1].fields["Hostname"], ip);

    teardown();
}
