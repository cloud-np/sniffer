use core::{Command, CommandDiscovery, CommandExecutor, Flag};
use packets::packet_reader;

pub struct CliExecutor;

impl CommandExecutor for CliExecutor {
    fn execute_help(&self) -> Result<(), String> {
        println!("Available commands:");
        for cmd in Flag::all() {
            let (short, long) = Flag::get_flag_usage(&cmd);
            println!("  -{}, --{}: {}", short, long, Flag::description(&cmd));
        }
        Ok(())
    }

    // TODO: This command shouldn't be executable. This should be the watch command
    fn execute_interface(&self, command: &Command) -> Result<(), String> {
        let interface = if let Flag::Interface(i) = &command.main_flag {
            i
        } else {
            panic!("Invalid interface passed");
        };
        packet_reader::read(
            interface,
            command
                .flags
                .iter()
                .any(|flag| matches!(flag, Flag::Details)),
        );
        Ok(())
    }

    // TODO:
    // This is basically implmented in the interface
    // need to refactor a bit.
    fn execute_watch(&self, _command: &Command) -> Result<(), String> {
        todo!()
    }

    fn execute_file(&self, command: &Command) -> Result<(), String> {
        Ok(())
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
    let cmd_args: Vec<String> = match args.iter().position(|arg| arg.starts_with("-")) {
        Some(pos) => args[pos..].to_vec(),
        None => Vec::new(),
    };

    if cmd_args.is_empty() {
        return None;
    }

    if let Some(flags) = flags_from_args(&cmd_args) {
        return Some(Command::with_args(flags, cmd_args));
    }
    None
}

fn flags_from_args(args: &[String]) -> Option<Vec<Flag>> {
    let flags: Vec<Flag> = args
        .iter()
        .enumerate()
        .filter_map(|(index, arg)| match arg.as_str() {
            "-h" | "--help" => Some(Flag::Help),
            "-i" | "--interface" => {
                if let Some(interface_name) = args.get(index + 1) {
                    Some(Flag::Interface(interface_name.clone()))
                } else {
                    panic!("No interface name provided");
                }
            }
            "-d" | "--details" => Some(Flag::Details),
            "-w" | "--watch" => Some(Flag::Watch),
            _ => None,
        })
        .collect();
    Some(flags)
}
