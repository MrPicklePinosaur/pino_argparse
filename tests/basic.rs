use pino_argparse::{Cli, Command, Flag, FlagParse};

#[test]
fn basic_test() {
    let cli = Cli {
        root_command: Command {
            flags: vec![Flag::new("dummy")],
            handler: |flagparse: FlagParse| -> Result<(), Box<dyn std::error::Error>> {
                println!("found flag {}", flagparse.get_flag("dummy"));
                Ok(())
            },
            ..Default::default()
        },
        ..Default::default()
    };
    let args = vec![String::from("myprogram"), String::from("--dummy")];
    cli.run(&args);
}
