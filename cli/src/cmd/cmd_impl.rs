use clap::Subcommand;

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Build a new patch
    #[command(
        disable_help_flag(true),
        subcommand_precedence_over_arg(true),
        allow_hyphen_values(true)
    )]
    Build { args: Vec<String> },
    /// Show patch detail info
    Info {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// Show patch target info
    Target {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// Show patch status
    Status {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// List all installed patches
    List,
    /// Apply a patch
    Apply {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// Remove a patch
    Remove {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// Activate a patch
    Active {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// Deactive a patch
    Deactive {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// Accept a patch
    Accept {
        /// Patch identifier, typically would be "<TARGET_NAME>/<PATCH_NAME>"
        identifier: String,
    },
    /// Save all patch status
    Save,
    /// Restore all patch status
    Restore {
        /// Only restore ACCEPTED patches
        #[arg(long, default_value = "false")]
        accepted: bool,
    },
    /// Reboot the system
    Reboot {
        /// Target kernel name
        #[arg(short, long)]
        target: Option<String>,
        #[arg(short, long, default_value = "false")]
        /// Skip all checks, force reboot
        force: bool,
    },
}

pub enum CommandArguments {
    None,
    CommandLineArguments { args: Vec<String> },
    PatchOperationArguments { identifier: String },
    PatchRestoreArguments { accepted_only: bool },
    RebootArguments { target: Option<String>, force: bool },
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

pub trait CommandExecutor {
    fn invoke(&self, args: &CommandArguments) -> std::io::Result<i32>;
}
