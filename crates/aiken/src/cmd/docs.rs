use aiken_project::watch::{self, watch_project, with_project};
use std::{path::PathBuf, process};

#[derive(clap::Args)]
/// Build the documentation for an Aiken project
pub struct Args {
    /// Path to project
    directory: Option<PathBuf>,

    /// Deny warnings; warnings will be treated as errors
    #[clap(short = 'D', long)]
    deny: bool,

    /// When enabled, re-run the command on file changes instead of exiting
    #[clap(short, long)]
    watch: bool,

    /// Output directory for the documentation
    #[clap(short = 'o', long)]
    destination: Option<PathBuf>,
}

pub fn exec(
    Args {
        directory,
        deny,
        watch,
        destination,
    }: Args,
) -> miette::Result<()> {
    let result = if watch {
        watch_project(
            directory.as_deref(),
            watch::default_filter,
            u32::default(),
            500,
            |p| p.docs(destination.clone()),
        )
    } else {
        with_project(directory.as_deref(), u32::default(), deny, |p| {
            p.docs(destination.clone())
        })
    };

    result.map_err(|_| process::exit(1))
}
