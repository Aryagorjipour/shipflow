use std::io;

use clap::CommandFactory;
use clap_complete::generate;

use crate::cli::Cli;
use crate::error::Result;

pub fn run(shell: clap_complete::Shell) -> Result<()> {
    let mut cmd = Cli::command();
    let name = cmd.get_name().to_owned();
    generate(shell, &mut cmd, name, &mut io::stdout());
    Ok(())
}
