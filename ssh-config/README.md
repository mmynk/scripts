# ssh-config

A tool to manage SSH configs.

## Build

Build the binary:
```shell
cargo build --release
```

## Usage

### Update IP of a config
```shell
./target/release/ssh-config update-ip -c '<config-name>' -i '<new-ip>'
```
