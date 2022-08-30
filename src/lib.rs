//! Simple arg parsing library.
//!
//! Provide a schema for your cli application and attach handlers to each command. Query for flags
//! values with support for short and long flag names, as well as optional parameters.

mod error;

use std::option::Option;
use std::str::FromStr;
use std::vec::Vec;

use error::{BoxResult, ArgParseError};

/// Command handler functions take in the `FlagParse` struct which contains information on received
/// flags.
type HandlerFn = fn(flagparse: FlagParse) -> BoxResult<()>;

pub type Result<T> = std::result::Result<T, ArgParseError>;

/// Base struct to define the schema for your cli application.
pub struct Cli {
    /// Name of your application
    pub program_name: &'static str,
    /// Brief description of your application
    pub synopsis: &'static str,
    /// Command handler if no subcommand is passed
    pub root_command: Command,
    /// List of subcommand handlers
    pub subcommands: Vec<Command>,
    /// WIP: Flags that will be parsed regardless of the command
    pub global_flags: Vec<Flag>,
}

/// Struct describing a command
pub struct Command {
    pub command_name: &'static str,
    pub desc: &'static str,
    /// Attached Handler function
    pub handler: HandlerFn,
    /// List of flags the command can take
    pub flags: Vec<Flag>,
    // pub args: u8, // TODO could make this take named argument names
}

/// Information on a flag
pub struct Flag {
    pub desc: String,
    /// If the flag is required to be passed
    pub required: bool,
    /// If the flag will take in a parameter
    pub parameter: bool,
    /// Short flag name
    ///
    /// Short flags can be passed with a single dash. For example `-v`.
    pub short: Option<char>,
    /// Long flag name
    ///
    /// Long names are passed with a double dash. For example `--verbose`.
    pub long: String,
}

/// Parsed flag information
pub struct FlagParse<'a> {
    flags: Vec<(&'a Flag, Option<String>)>,
    /// List of any non-flag arguments that were picked up.
    pub args: Vec<String>,
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            program_name: "",
            synopsis: "",
            root_command: Command {
                ..Default::default()
            },
            subcommands: vec![],
            global_flags: vec![],
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Command {
            command_name: "",
            desc: "",
            handler: |_flagparse: FlagParse| Ok(()),
            flags: vec![],
        }
    }
}

impl Cli {
    /// Start the cli.
    ///
    /// Pass in the environment arguments.
    pub fn run(&self, args: &Vec<String>) -> Result<()> {
        let mut arg_it = args.iter();
        arg_it.next(); // skip program name

        // find command to dispatch
        let mut next = arg_it.next();
        let cmd: &Command = if let Some(cmd_name) = next {
            if looks_like_flag(cmd_name) {
                &self.root_command
            } else {
                next = arg_it.next();
                self.subcommands
                    .iter()
                    .find(|c| &c.command_name == cmd_name)
                    .unwrap_or(&self.root_command)
            }
        } else {
            &self.root_command
        };

        // parse flags for command
        let mut flagparse = FlagParse::new();

        while next.is_some() {
            let cur_arg = next.unwrap();

            let flag: Option<&Flag> = if cur_arg.starts_with("--") {
                // TODO maybe unneeded copy
                cmd.flags
                    .iter()
                    .find(|f| f.long == cur_arg[2..].to_string())
            } else if cur_arg.starts_with("-") {
                cmd.flags.iter().find(|f| f.short == cur_arg.chars().nth(1))
            } else {
                break;
            };

            if flag.is_none() {
                // TODO ugly
                return Err(ArgParseError::InvalidFlag(cur_arg.to_owned()));
            }
            let flag = flag.unwrap();

            // check if flag is expecting value
            if flag.parameter {
                let value = arg_it.next().ok_or(ArgParseError::MissingFlagValue(cur_arg.to_owned()))?;
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

        // TODO properly propogate user errors (maybe add error handler)
        if let Err(err) = dispatch(flagparse) {
            return Err(ArgParseError::UserError(err));
        }

        Ok(())
    }

    pub fn help_message(&self) {}
}

fn looks_like_flag(token: &str) -> bool {
    return token.starts_with("--") || token.starts_with("-");
}

impl Flag {
    /// Construct a new flag
    ///
    /// The flag is required to a have a short name
    pub fn new(long: &str) -> Self {
        Flag {
            desc: String::new(),
            required: false,
            parameter: false,
            short: None,
            long: long.to_owned(),
        }
    }

    /// Specify the description
    pub fn desc(mut self, desc: &str) -> Self {
        self.desc = desc.to_owned();
        self
    }
    /// Specify if the flag is required or not
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }
    /// Specify if the flag takes in a parameter
    pub fn parameter(mut self) -> Self {
        self.parameter = true;
        self
    }
    /// Specify an optional short name for the flag
    pub fn short(mut self, short: char) -> Self {
        self.short = Some(short);
        self
    }
}

impl<'a> FlagParse<'a> {
    /// Get the parameter of a flag
    ///
    /// Will return `None` if flag was not passed or the flag did not take parameters.
    /// TODO: should return a specific error if flag does not exist or if flag did not take
    /// parameters.
    pub fn get_flag_value<T: FromStr>(&self, long: &str) -> Option<T> {
        let pair = self.flags.iter().find(|p| p.0.long.eq(long));
        if pair.is_none() {
            return None;
        }
        let pair = pair.unwrap();

        match &pair.1 {
            Some(v) => v.parse::<T>().ok(),
            None => None,
        }
    }

    /// Check if a flag was passed
    pub fn get_flag(&self, long: &str) -> bool {
        return self.flags.iter().find(|p| p.0.long.eq(long)).is_some();
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
