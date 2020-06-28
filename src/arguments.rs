use crate::{commands, error::Error};
use clap::Clap;

/// Run the application using the command line arguments
pub fn run() -> Result<(), Error> {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::List(_) => commands::list()?,
    }

    Ok(())
}

/// gcloud configuration manager
#[derive(Clap)]
pub struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    List(List),
}

/// List all available configurations
#[derive(Clap)]
struct List {}
