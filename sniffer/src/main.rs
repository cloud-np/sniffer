use core::Command;
use sniffer_cli::{CliDiscovery, CliExecutor};

fn main() {
    // We can use the "with" pattern to enstablish
    // how the command should be run. e.g:
    // Command()
    //      .with_parser(CustomParser)
    //      .with_discover(CustomDiscovery)
    //      .with_executor(CliExecutor)
    let command = Command::discover(&CliDiscovery);

    match command {
        Some(cmd) => {
            if let Err(e) = cmd.execute(&CliExecutor) {
                eprintln!("Error executing command: {}", e);
                std::process::exit(1);
            }
        }
        None => {
            eprintln!("Unknown or missing command. Use -h or --help for usage information.");
            std::process::exit(1);
        }
    }
}
