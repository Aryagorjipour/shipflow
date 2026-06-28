use std::io::{self, Write};
use std::process::ExitCode;

use crate::cli::{Cli, Commands};
use crate::error::ShipflowError;
use crate::utils;

pub mod add;
pub mod completions;
pub mod done;
pub mod list;
pub mod report;
pub mod status;

#[cfg(feature = "tui")]
pub mod board;

pub fn run(cli: Cli) -> ExitCode {
    init_tracing(cli.verbose);

    let result = match cli.command {
        Commands::Add {
            title,
            tags,
            note,
            global,
        } => add::run(title, tags, note, global),
        Commands::List {
            status,
            tags,
            global,
        } => list::run(status, tags, global),
        Commands::Done {
            query,
            commit,
            no_link,
            global,
        } => done::run(query, commit, no_link, global),
        Commands::Report {
            period,
            format,
            global,
        } => report::run(period, format, global),
        Commands::Status { global } => status::run(global),
        #[cfg(feature = "tui")]
        Commands::Board { global } => board::run(global),
        Commands::Completions { shell } => completions::run(shell),
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            let _ = utils::print_error(&err.to_string());
            ExitCode::from(err.exit_code() as u8)
        }
    }
}

fn init_tracing(verbose: bool) {
    use tracing_subscriber::EnvFilter;

    let filter = if verbose {
        EnvFilter::new("shipflow=debug")
    } else if let Ok(env) = std::env::var("RUST_LOG") {
        EnvFilter::new(env)
    } else {
        EnvFilter::new("off")
    };

    let _ = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_writer(io::stderr)
        .try_init();
}

pub fn writeln_stdout(line: &str) -> Result<(), ShipflowError> {
    writeln!(io::stdout(), "{line}")?;
    Ok(())
}

pub fn write_stdout(text: &str) -> Result<(), ShipflowError> {
    write!(io::stdout(), "{text}")?;
    io::stdout().flush()?;
    Ok(())
}
