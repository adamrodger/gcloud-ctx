mod arguments;
mod commands;
mod configuration;
mod error;
mod fzf;
mod properties;

use anyhow::{bail, Result};
pub use arguments::Opts;
use arguments::SubCommand;

/// Run the application using the command line arguments
pub fn run(opts: Opts) -> Result<()> {
    if let Some(name) = opts.context {
        // shortcut for activate
        commands::activate(&name)?;
        return Ok(());
    } else if let Some(subcmd) = opts.subcmd {
        match subcmd {
            SubCommand::Activate { name } => match (name, fzf::is_fzf_installed()) {
                (Some(name), _) => commands::activate(&name)?,
                (None, true) => commands::activate(&fzf::fuzzy_find_config()?)?,
                (None, false) => bail!("You must supply a configuration to activate"),
            },
            SubCommand::Copy {
                src_name,
                dest_name,
                activate,
                force,
            } => {
                commands::copy(&src_name, &dest_name, force, activate)?;
            }
            SubCommand::Create {
                name,
                project,
                account,
                zone,
                region,
                activate,
                force,
            } => {
                commands::create(&name, &project, &account, &zone, region.as_deref(), force, activate)?;
            }
            SubCommand::Current => commands::current()?,
            SubCommand::Describe { name } => commands::describe(&name)?,
            SubCommand::List => commands::list()?,
            SubCommand::Rename {
                old_name,
                new_name,
                force,
            } => commands::rename(&old_name, &new_name, force)?,
        }
    } else {
        commands::current()?;
    }

    Ok(())
}
