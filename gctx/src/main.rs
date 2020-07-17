//! A fast replacement for [`gcloud config configurations`](https://cloud.google.com/sdk/gcloud/reference/config/configurations)
//! written in Rust for managing [Google Cloud Platform](https://cloud.google.com/) `gcloud` configurations easily and quickly
//!
//! **Note**: This project is independent and not afilliated with Google in any way
//!
//! ## Installation
//!
//! ### From source
//!
//! Get the latest stable version of Rust via [`rustup`](https://rustup.rs/) and run:
//!
//! ```bash
//! cargo install gctx
//! ## gctx will now be on your PATH
//! ```
//!
//! ### Pre-Built Binaries
//!
//! Grab the [latest binary](https://github.com/adamrodger/gcloud-ctx/releases/latest), extract it and add it to your `PATH`.
//!
//! ### Others (e.g. `scoop` and `brew`)
//!
//! **TODO**: [Support for `scoop`](https://github.com/adamrodger/gcloud-ctx/issues/13)
//!
//! **TODO**: [Support for `brew`](https://github.com/adamrodger/gcloud-ctx/issues/14)
//!
//! ## Usage
//!
//! ```bash
//! ## show the current configuration (useful for adding to default prompt)
//! gctx current
//! gctx          # shorthand, just omit current
//!
//! ## list all configurations
//! gctx list
//!
//! ## activate a different configuration
//! gctx my-config
//! gctx activate my-config   # explicitly activate, e.g. if your configuration name clashes with a gctx command
//! gctx activate             # if fzf is installed, you can omit the name and select from a list
//!
//! ## create (and optionally activate) a new configuration
//! gctx create my-config --project foo \
//!                       --account a.user@example.org \
//!                       --zone europe-west1-d \
//!                       --region europe-west1 \
//!                       --force \
//!                       --activate
//!
//! ## copy an existing configuration
//! gctx copy src-name dest-name --force --activate
//!
//! ## show the properties of a configuration (like gcloud config configurations describe)
//! gctx describe           # defaults to the current configuration
//! gctx describe name      # describe a named configuration
//!
//! ## rename a configuration
//! gctx rename old-name new-name
//! gctx rename --force old-name existing-name   # use force to overwrite an existing configuration
//!
//! ## delete a configuration. note: you can't delete the active configuration
//! gctx delete my-config
//!
//! ## show help and usage
//! gctx --help
//! ```

mod arguments;
mod commands;
mod fzf;

use anyhow::{bail, Result};
use arguments::{Opts, SubCommand};
use clap::Clap;

fn main() -> Result<()> {
    let opts = Opts::parse();
    run(opts)?;
    Ok(())
}

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
                commands::copy(&src_name, &dest_name, force.into(), activate.into())?;
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
                commands::create(
                    &name,
                    &project,
                    &account,
                    &zone,
                    region.as_deref(),
                    force.into(),
                    activate.into(),
                )?;
            }
            SubCommand::Current => commands::current()?,
            SubCommand::Delete { name } => commands::delete(&name)?,
            SubCommand::Describe { name } => commands::describe(name.as_deref())?,
            SubCommand::List => commands::list()?,
            SubCommand::Rename {
                old_name,
                new_name,
                force,
            } => commands::rename(&old_name, &new_name, force.into())?,
        }
    } else {
        commands::current()?;
    }

    Ok(())
}
