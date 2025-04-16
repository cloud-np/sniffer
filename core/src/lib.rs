/// The Command type and its data/args
/// that gets returned from a `CommandDiscovery`.
#[derive(Debug, Clone)]
pub struct Command {
    pub main_flag: Flag,
    pub flags: Vec<Flag>,
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
    fn execute_file(&self, command: &Command) -> Result<(), String>;
}

impl Command {
    // Exists in case of a graphical user interface
    pub fn new(flags: Vec<Flag>) -> Command {
        Command {
            main_flag: flags[0].clone(),
            flags,
            args: Vec::new(),
        }
    }

    pub fn with_args(flags: Vec<Flag>, args: Vec<String>) -> Self {
        Command {
            main_flag: flags[0].clone(),
            flags,
            args,
        }
    }

    pub fn discover<T: CommandDiscovery>(discovery: &T) -> Option<Command> {
        discovery.discover_command()
    }

    pub fn execute<T: CommandExecutor>(&self, executor: &T) -> Result<(), String> {
        match self.main_flag {
            Flag::Help => executor.execute_help(),
            Flag::Interface(_) => executor.execute_interface(self),
            Flag::Watch => executor.execute_watch(self),
            Flag::Details => executor.execute_watch(self),
            Flag::File(_) => executor.execute_file(self),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Flag {
    Help,
    Interface(String),
    Watch,
    Details,
    File(String),
}

impl Flag {
    // Change the return type to a slice instead of an array
    pub fn all() -> Vec<Flag> {
        vec![
            Flag::Help,
            Flag::Interface("".to_string()),
            Flag::Watch,
            Flag::Details,
        ]
    }

    // TODO: Fix the function name
    pub fn get_flag_usage(flag: &Flag) -> (&'static str, &'static str) {
        match flag {
            Flag::Help => ("h", "help"),
            Flag::Interface(_) => ("i", "interface"),
            Flag::Watch => ("w", "watch"),
            Flag::Details => ("d", "details"),
            Flag::File(_) => ("f", "file"),
        }
    }

    pub fn description(flag: &Flag) -> &str {
        match flag {
            Flag::Help => "Display help information",
            Flag::Interface(_) => "Configure network interface",
            Flag::Watch => "Monitor network packets",
            Flag::Details => "Enable detailed output",
            Flag::File(_) => "Specify a file to save output",
        }
    }
}
