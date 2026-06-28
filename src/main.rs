use clap::Parser;
use shipflow::cli::Cli;
use shipflow::commands;

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    commands::run(cli)
}
