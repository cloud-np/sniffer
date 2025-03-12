use core::{Command, CommandDiscovery, CommandExecutor, CommandType};
use package_reader::packet_reader;

pub struct CliExecutor;

impl CommandExecutor for CliExecutor {
    fn execute_help(&self) -> Result<(), String> {
        println!("Available commands:");
        for cmd in [
            CommandType::Help,
            CommandType::Interface,
            CommandType::Watch,
        ]
        .iter()
        {
            let (short, long) = flags(cmd);
            println!("  -{}, --{}: {}", short, long, description(cmd));
        }
        Ok(())
    }

    fn execute_interface(&self, command: &Command) -> Result<(), String> {
        Ok(packet_reader::read(command.args[1].to_string()))
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

    let cmd_str = &cmd_args[0];
    if let Some(cmd_type) = command_type_from_arg(cmd_str) {
        return Some(Command::with_args(cmd_type, cmd_args));
    }
    None
}

fn command_type_from_arg(arg: &str) -> Option<CommandType> {
    match arg {
        "h" | "help" | "-h" | "--help" => Some(CommandType::Help),
        "i" | "interface" | "-i" | "--interface" => Some(CommandType::Interface),
        "w" | "watch" | "-w" | "--watch" => Some(CommandType::Watch),
        _ => None,
    }
}

pub fn flags(command_type: &CommandType) -> (&str, &str) {
    match command_type {
        CommandType::Help => ("h", "help"),
        CommandType::Interface => ("i", "interface"),
        CommandType::Watch => ("w", "watch"),
    }
}

pub fn description(command_type: &CommandType) -> &str {
    match command_type {
        CommandType::Help => "Display help information",
        CommandType::Interface => "Configure network interface",
        CommandType::Watch => "Monitor network packets",
    }
}
