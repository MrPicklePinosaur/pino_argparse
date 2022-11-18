
<div align="center">

# pino_argparse
a tiny zero-dependency argparsing library

[![crates.io](https://img.shields.io/crates/v/pino_argparse.svg)](https://crates.io/crates/pino_argparse)
[![docs.rs](https://docs.rs/pino_argparse/badge.svg)](https://docs.rs/pino_argparse)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

**pino_argparse** is a bite-sized argparsing library that can handle short and
long flags with or without values, subcommands and basic validation.

## USING IN YOUR PROJECT

Add the following to your `Cargo.toml`:
```
pino_argparse = { version = "0.1.0" }
```

A simple cli would look something like:
```rust
fn main() {

    // Get arguments
    let args = std::env::args().collect();

    // Initialize the CLI
    let cli = Cli {
        program_name: "myprogram",
        synopsis: "a simple program to show of the argparse library",
        root_command: Command {
            flags: vec![
                Flag::new("help").short('h'),
                Flag::new("verbose").short('v'),
            ],
            handler: |flagparse: FlagParse| -> Result<(), Box<dyn std::error::Error>> {
                if flagparse.get_flag("help") {
                    println!("We called the help flag!");
                }

                Ok(())
            },
            ..Default::default()
        },
        ..Default::default()
    };

    // Run the CLI
    let flagparse = cli.run(&args).unwrap();
}
```

## TODO

- [ ] auto doc/help message generation?
- [ ] iterator to give flags in order
