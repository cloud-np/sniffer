/// The Command type and its data/args
/// that gets returned from a `CommandDiscovery`.
#[derive(Debug, Clone)]
pub struct Command {
    command_type: CommandType,
    #[allow(dead_code)]
    pub args: Vec<String>,
}

/// `CommandDiscovery` when implemented by a "discovery"
/// it should find/discover which Command needs to be run.
pub trait CommandDiscovery {
    fn discover_command(&self) -> Option<Command>;
}

/// `CommandExecutor` when implemented by an "executor"
/// executions for each command should be provided.
pub trait CommandExecutor {
    fn execute_help(&self) -> Result<(), String>;
    fn execute_interface(&self, command: &Command) -> Result<(), String>;
    fn execute_watch(&self, command: &Command) -> Result<(), String>;
}

impl Command {
    pub fn new(command_type: CommandType) -> Command {
        Command {
            command_type,
            args: Vec::new(),
        }
    }

    pub fn with_args(command_type: CommandType, args: Vec<String>) -> Self {
        Command { command_type, args }
    }

    pub fn discover<T: CommandDiscovery>(discovery: &T) -> Option<Command> {
        discovery.discover_command()
    }

    pub fn execute<T: CommandExecutor>(&self, executor: &T) -> Result<(), String> {
        match self.command_type {
            CommandType::Help => executor.execute_help(),
            CommandType::Interface => executor.execute_interface(self),
            CommandType::Watch => executor.execute_watch(self),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandType {
    Help,
    Interface,
    Watch,
}

impl CommandType {}
