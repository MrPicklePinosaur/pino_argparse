
mod error;

use std::str::FromStr;
use std::vec::Vec;
use std::option::Option;

use error::{BoxResult, Error};

// mini argparsing library

type HandlerFn = fn(flagparse: FlagParse) -> BoxResult<()>;

pub struct Cli {
    pub program_name: &'static str,
    pub synopsis: &'static str,
    pub root_command: Command,
    pub subcommands: Vec<Command>,
    pub global_flags: Vec<Flag>,
}

pub struct Command {
    pub command_name: &'static str,
    pub desc: &'static str,
    pub handler: HandlerFn,
    pub flags: Vec<Flag>,
    // pub args: u8, // TODO could make this take named argument names
}

pub struct Flag {
    pub desc: String,
    pub required: bool,
    pub parameter: bool,
    pub short: char,
    pub long: Option<String>,
}

pub struct FlagParse<'a> {
    flags: Vec<(&'a Flag, Option<String>)>,
    pub args: Vec<String>,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            program_name: "myprogram",
            synopsis: "a cli program",
            root_command: Command {
                command_name: "hello",
                desc: "hello world command",
                handler: |_flagparse: FlagParse| {
                    println!("hello world!");
                    Ok(())
                },
                flags: vec![]
            },
            subcommands: vec![],
            global_flags: vec![]
        }
    }
}

impl Cli {
    
    pub fn run(&self, args: &Vec<String>) -> BoxResult<()> {
        
        let mut arg_it = args.iter();
        arg_it.next(); // skip program name

        // find command to dispatch
        let cmd: &Command = if let Some(cmd_name) = arg_it.next() {
            self.subcommands
                .iter()
                .find(|c| &c.command_name == cmd_name).ok_or(Error::InvalidCommand)?
        } else {
            &self.root_command 
        };

        // parse flags for command
        let mut flagparse = FlagParse::new();

        let mut next = arg_it.next();
        while next.is_some() {

            let cur_arg = next.unwrap();

            let flag: Option<&Flag> = if cur_arg.starts_with("--") {
                // TODO maybe unneeded copy
                cmd.flags.iter().find(|f| f.long == Some(cur_arg[2..].to_string()))
            } else if cur_arg.starts_with("-") {
                cmd.flags.iter().find(|f| Some(f.short) == cur_arg.chars().nth(1))
            } else {
                break;
            };

            if flag.is_none() {
                // TODO ugly
                return Err(Box::new(Error::InvalidFlag));
            }
            let flag = flag.unwrap();

            // check if flag is expecting value
            if flag.parameter {
                let value = arg_it.next().ok_or(Error::MissingFlagValue)?;
                flagparse.add_flag_with_value(flag, value);
            } else {
                flagparse.add_flag(flag);
            }

            next = arg_it.next();
        }

        // read rest of arguments
        while next.is_some() {
            flagparse.args.push(next.unwrap().to_string());
            next = arg_it.next();
        }

        // TODO check if all mandatory flags were called

        // pass control to command handler
        let dispatch = cmd.handler;
        dispatch(flagparse)?;

        Ok(())
    }

    pub fn help_message(&self) {

    }

}

impl Flag {

    pub fn new(short: char) -> Self {
        Flag {
            desc: String::new(),
            required: false,
            parameter: false,
            short,
            long: None,
        }
    }

    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = desc.to_string();
        self
    }
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    pub fn parameter(mut self) -> Self {
        self.parameter = true;
        self
    }
    pub fn long(mut self, long: &str) -> Self {
        self.long = Some(long.to_string());
        self
    }

}

impl<'a> FlagParse<'a> {

    pub fn get_flag_value<T: FromStr>(&self, short: char) -> Option<T> {
        let pair = self.flags.iter().find(|p| p.0.short == short);
        if pair.is_none() { return None; }
        let pair = pair.unwrap();

        match &pair.1 {
            Some(v) => v.parse::<T>().ok(),
            None => None,
        }
    }

    pub fn get_flag(&self, short: char) -> bool {
        return self.flags.iter().find(|p| p.0.short == short).is_some();
    }

    fn new() -> Self {
        FlagParse {
            flags: Vec::new(),
            args: Vec::new(),
        }
    }

    fn add_flag(&mut self, flag: &'a Flag) {
        self.flags.push((flag, None));
    }

    fn add_flag_with_value(&mut self, flag: &'a Flag, value: &str) {
        self.flags.push((flag, Some(value.to_string())));
    }

}

