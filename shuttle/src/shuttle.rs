//! The `shuttle` cli
use clap::{CommandFactory, Parser, Subcommand};
use shuttle::cmd::NodeArgs;

/// A fast local Ethereum development node.
#[derive(Debug, Parser)]
#[clap(name = "shuttle", version = shuttle::VERSION_MESSAGE, next_display_order = None)]
pub struct App {
    #[clap(flatten)]
    pub node: NodeArgs,

    #[clap(subcommand)]
    pub cmd: Option<Commands>,
}

#[derive(Clone, Debug, Subcommand, Eq, PartialEq)]
pub enum Commands {
    /// Generate shell completions script.
    #[clap(visible_alias = "com")]
    Completions {
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Generate Fig autocompletion spec.
    #[clap(visible_alias = "fig")]
    GenerateFigSpec,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::parse();
    app.node.evm_opts.resolve_rpc_alias();

    if let Some(ref cmd) = app.cmd {
        match cmd {
            Commands::Completions { shell } => {
                clap_complete::generate(
                    *shell,
                    &mut App::command(),
                    "shuttle",
                    &mut std::io::stdout(),
                );
            }
            Commands::GenerateFigSpec => clap_complete::generate(
                clap_complete_fig::Fig,
                &mut App::command(),
                "shuttle",
                &mut std::io::stdout(),
            ),
        }
        return Ok(())
    }

    let _ = fdlimit::raise_fd_limit();
    app.node.run().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_help() {
        let _: App = App::parse_from(["shuttle", "--help"]);
    }

    #[test]
    fn can_parse_completions() {
        let args: App = App::parse_from(["shuttle", "completions", "bash"]);
        assert_eq!(args.cmd, Some(Commands::Completions { shell: clap_complete::Shell::Bash }));
    }
}
