use core::{Command, CommandDiscovery, CommandExecutor, Flag};
use package_reader::packet_reader;

pub struct CliExecutor;

impl CommandExecutor for CliExecutor {
    fn execute_help(&self) -> Result<(), String> {
        println!("Available commands:");
        for cmd in [Flag::Help, Flag::Interface(0b0), Flag::Watch].iter() {
            let (short, long) = flags(cmd);
            println!("  -{}, --{}: {}", short, long, description(cmd));
        }
        Ok(())
    }

    fn execute_interface(&self, command: &Command) -> Result<(), String> {
        let flags = if let Flag::Interface(v) = command.flags {
            v
        } else {
            0
        };
        packet_reader::read(command.args[1].to_string(), flags);
        Ok(())
    }

    fn execute_watch(&self, _command: &Command) -> Result<(), String> {
        todo!()
    }
}

pub struct CliDiscovery;

impl CommandDiscovery for CliDiscovery {
    fn discover_command(&self) -> Option<Command> {
        let args: Vec<String> = std::env::args().collect();
        command_from_args(&args)
    }
}

fn command_from_args(args: &[String]) -> Option<Command> {
    let cmd_args: Vec<String> = args
        .iter()
        .skip_while(|arg| !arg.starts_with("-"))
        .cloned()
        .collect();

    if cmd_args.is_empty() {
        return None;
    }

    if let Some(flag) = flag_from_args(&cmd_args) {
        return Some(Command::with_args(flag, cmd_args));
    }
    None
}

fn flag_from_args(args: &[String]) -> Option<Flag> {
    let cmd_str = &args[0];
    match cmd_str.as_str() {
        "h" | "help" | "-h" | "--help" => Some(Flag::Help),
        "i" | "interface" | "-i" | "--interface" => {
            // assuming the user provides the interface name as the next argument
            // then verbose as the third argument: "-i eth0 v" or "i eth0 --verbose"
            let is_verbose = args
                .get(2)
                .map(|arg| matches!(arg.as_str(), "v" | "-v" | "--verbose"))
                .unwrap_or(false);
            Some(Flag::Interface(if is_verbose { 0b1 } else { 0b0 }))
        }
        "w" | "watch" | "-w" | "--watch" => Some(Flag::Watch),
        _ => None,
    }
}

pub fn flags(flag: &Flag) -> (&str, &str) {
    match flag {
        Flag::Help => ("h", "help"),
        Flag::Interface(_) => ("i", "interface"),
        Flag::Watch => ("w", "watch"),
    }
}

pub fn description(flag: &Flag) -> &str {
    match flag {
        Flag::Help => "Display help information",
        Flag::Interface(_) => "Configure network interface",
        Flag::Watch => "Monitor network packets",
    }
}
