use std::{env, io, io::Write, path::PathBuf, process::exit};

use clap::{ArgAction, Args, CommandFactory, Parser, Subcommand, ValueHint};
use clap_complete::generate;
use tokio::fs::remove_dir_all;

use crate::shells::Shell;

#[derive(Debug, Parser)]
#[command(author)]
pub struct Cli {
    /// Print version.
    #[arg(short('V'), long)]
    pub version: bool,

    /// Suppress output.
    #[arg(short, long, action(ArgAction::Count))]
    pub quiet: u8,

    /// Use stdio to communicate with the LSP server.
    #[arg(long)]
    pub stdio: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Check availability.
    Check(Check),

    /// Remove artifacts from the target directory.
    Clean,

    /// Generate shell completions.
    Completions {
        /// The shell to generate completions for.
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Generate a man page for the CLI.
    Manpage,
}

#[derive(Args, Debug)]
pub struct Check {
    /// The path of a file or directory to check availability.
    #[arg(value_name("path"), value_hint(ValueHint::AnyPath))]
    pub path: Option<PathBuf>,

    /// Whether to check for all targets
    /// (default: false).
    #[arg(
        long,
        default_value_t = false,
        value_name("all-targets"),
        help = "Run the check for all targets instead of current only"
    )]
    pub all_targets: bool,

    /// Whether to check for all features
    /// (default: false).
    #[arg(
        long,
        default_value_t = false,
        value_name("all-features"),
        help = "Run the check for all features instead of the current active ones only"
    )]
    pub all_features: bool,
}

impl Commands {
    /// Execute the command.
    pub async fn execute(self) {
        match self {
            Self::Check(options) => {
                let path = options.path.unwrap_or_else(|| env::current_dir().unwrap());

                if crate::Backend::check_with_options(
                    &path,
                    options.all_targets,
                    options.all_features,
                )
                .await
                {
                    log::info!("Successfully analyzed");
                    exit(0);
                }
                log::error!("Analyze failed");
                exit(1);
            }
            Self::Clean => {
                if let Ok(meta) = cargo_metadata::MetadataCommand::new().exec() {
                    let target = meta.target_directory.join("owl");
                    remove_dir_all(&target).await.ok();
                }
            }
            Self::Completions { shell } => {
                generate(shell, &mut Cli::command(), "rustowl", &mut io::stdout());
            }
            Self::Manpage => {
                let man = clap_mangen::Man::new(Cli::command());
                let mut buffer: Vec<u8> = Vec::default();
                man.render(&mut buffer).unwrap();
                io::stdout().write_all(&buffer).unwrap();
            }
        }
    }
}

impl Cli {
    /// Run the CLI application.
    pub async fn run(self) {
        if let Some(command) = self.command {
            command.execute().await;
        } else if self.version {
            if self.quiet == 0 {
                print!("RustOwl ");
            }
            println!("v{}", clap::crate_version!());
        } else {
            crate::start_lsp_server().await;
        }
    }
}
