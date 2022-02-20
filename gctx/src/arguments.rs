use clap::{crate_version, Parser};

/// gcloud configuration manager
#[derive(Parser, Debug)]
#[clap(version = crate_version!())]
pub struct Opts {
    /// Switch to this context (shorthand for activate, ignores subsequent arguments)
    pub context: Option<String>,

    #[clap(subcommand)]
    pub subcmd: Option<SubCommand>,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    /// Activate a configuration by name
    Activate {
        /// Name of the configuration to activate
        name: Option<String>,
    },

    /// Copy a configuration
    Copy {
        // Name of the configuration to copy
        src_name: String,

        // Name of the new configuration
        dest_name: String,

        /// Activate the new configuration immediately
        #[clap(long)]
        activate: bool,

        /// Force a copy to overwrite an existing configuration
        #[clap(short, long)]
        force: bool,
    },

    /// Create a new configuration
    Create {
        /// Create a configuration interactively
        #[clap(short, long, conflicts_with_all(&["name", "project", "account", "zone", "region", "activate", "force"]))]
        interactive: bool,

        // Name of the new configuration
        #[clap(required_unless_present("interactive"), requires_all(&["project", "account", "zone"]))]
        name: Option<String>,

        /// Setting for core/project
        #[clap(short, long)]
        project: Option<String>,

        /// Setting for core/account
        #[clap(short, long)]
        account: Option<String>,

        /// Setting for compute/zone
        #[clap(short, long)]
        zone: Option<String>,

        /// Setting for compute/region
        #[clap(short, long)]
        region: Option<String>,

        /// Activate the new configuration immediately
        #[clap(long)]
        activate: bool,

        /// Force a create to overwrite an existing configuration
        #[clap(short, long)]
        force: bool,
    },

    /// Show the current configuration
    Current,

    /// Delete a configuration
    Delete {
        /// Name of the configuration to delete
        name: String,
    },

    /// Describe all the properties in a configuration
    Describe {
        /// Name of the configuration, defaults to current
        name: Option<String>,
    },

    /// List all available configurations
    List,

    /// Rename a configuration
    Rename {
        /// Name of an existing configuration
        old_name: String,

        /// New name
        new_name: String,

        /// Force a rename to overwrite an existing configuration
        #[clap(short, long)]
        force: bool,
    },
}
