use crate::cmd::spark::{
    bind::BindArgs,
    build::BuildArgs,
    cache::CacheArgs,
    config, coverage,
    create::CreateArgs,
    debug::DebugArgs,
    doc::DocArgs,
    flatten,
    fmt::FmtArgs,
    fourbyte::UploadSelectorsArgs,
    geiger,
    init::InitArgs,
    inspect,
    install::InstallArgs,
    remappings::RemappingArgs,
    remove::RemoveArgs,
    script::ScriptArgs,
    selectors::SelectorsSubcommands,
    snapshot, test, tree, update,
    verify::{VerifyArgs, VerifyCheckArgs},
};
use clap::{Parser, Subcommand, ValueHint};
use corebc::ylem::{artifacts::output_selection::ContractOutputSelection, CvmVersion};
use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Parser)]
#[clap(name = "spark", version = crate::utils::VERSION_MESSAGE)]
pub struct Opts {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Build, test, fuzz, debug and deploy Ylem contracts.",
    after_help = "Find more information in the book: https://foxar.dev/",
    next_display_order = None
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    /// Run the project's tests.
    #[clap(visible_alias = "t")]
    Test(test::TestArgs),

    /// Run a smart contract as a script, building transactions that can be sent onchain.
    Script(ScriptArgs),

    /// Generate coverage reports.
    Coverage(coverage::CoverageArgs),

    /// Generate Rust bindings for smart contracts.
    #[clap(alias = "bi")]
    Bind(BindArgs),

    /// Build the project's smart contracts.
    #[clap(visible_aliases = ["b", "compile"])]
    Build(BuildArgs),

    /// Debugs a single smart contract as a script.
    #[clap(visible_alias = "d")]
    Debug(DebugArgs),

    /// Update one or multiple dependencies.
    ///
    /// If no arguments are provided, then all dependencies are updated.
    #[clap(visible_alias = "u")]
    Update(update::UpdateArgs),

    /// Install one or multiple dependencies.
    ///
    /// If no arguments are provided, then existing dependencies will be installed.
    #[clap(visible_alias = "i")]
    Install(InstallArgs),

    /// Remove one or multiple dependencies.
    #[clap(visible_alias = "rm")]
    Remove(RemoveArgs),

    /// Get the automatically inferred remappings for the project.
    #[clap(visible_alias = "re")]
    Remappings(RemappingArgs),

    /// Verify smart contracts on Etherscan.
    #[clap(visible_alias = "v")]
    VerifyContract(VerifyArgs),

    /// Check verification status on Etherscan.
    #[clap(visible_alias = "vc")]
    VerifyCheck(VerifyCheckArgs),

    /// Deploy a smart contract.
    #[clap(visible_alias = "c")]
    Create(CreateArgs),

    /// Create a new Spark project.
    Init(InitArgs),

    /// Generate shell completions script.
    #[clap(visible_alias = "com")]
    Completions {
        #[clap(value_enum)]
        shell: clap_complete::Shell,
    },

    /// Generate Fig autocompletion spec.
    #[clap(visible_alias = "fig")]
    GenerateFigSpec,

    /// Remove the build artifacts and cache directories.
    #[clap(visible_alias = "cl")]
    Clean {
        /// The project's root path.
        ///
        /// By default root of the Git repository, if in one,
        /// or the current working directory.
        #[clap(long, value_hint = ValueHint::DirPath, value_name = "PATH")]
        root: Option<PathBuf>,
    },

    /// Manage the Foxar cache.
    Cache(CacheArgs),

    /// Create a snapshot of each test's gas usage.
    #[clap(visible_alias = "s")]
    Snapshot(snapshot::SnapshotArgs),

    /// Display the current config.
    #[clap(visible_alias = "co")]
    Config(config::ConfigArgs),

    /// Flatten a source file and all of its imports into one file.
    #[clap(visible_alias = "f")]
    Flatten(flatten::FlattenArgs),

    /// Format Solidity source files.
    Fmt(FmtArgs),

    /// Get specialized information about a smart contract.
    #[clap(visible_alias = "in")]
    Inspect(inspect::InspectArgs),

    /// Uploads abi of given contract to the https://sig.eth.samczsun.com
    /// function selector database.
    #[clap(visible_alias = "up")]
    UploadSelectors(UploadSelectorsArgs),

    /// Display a tree visualization of the project's dependency graph.
    #[clap(visible_alias = "tr")]
    Tree(tree::TreeArgs),

    /// Detects usage of unsafe cheat codes in a project and its dependencies.
    Geiger(geiger::GeigerArgs),

    /// Generate documentation for the project.
    Doc(DocArgs),

    /// Function selector utilities
    #[clap(visible_alias = "se")]
    Selectors {
        #[clap(subcommand)]
        command: SelectorsSubcommands,
    },
}

// A set of ylem compiler settings that can be set via command line arguments, which are intended
// to be merged into an existing `foxar_config::Config`.
//
// See also [`BuildArgs`]
#[derive(Default, Debug, Clone, Parser, Serialize)]
#[clap(next_help_heading = "Compiler options")]
pub struct CompilerArgs {
    /// The target CVM version.
    #[clap(long, value_name = "VERSION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvm_version: Option<CvmVersion>,

    /// Activate the Solidity optimizer.
    #[clap(long)]
    #[serde(skip)]
    pub optimize: bool,

    /// The number of optimizer runs.
    #[clap(long, value_name = "RUNS")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimizer_runs: Option<usize>,

    /// Extra output to include in the contract's artifact.
    ///
    /// Example keys: evm.assembly, ewasm, ir, irOptimized, metadata
    ///
    /// For a full description, see https://docs.soliditylang.org/en/v1.1.0/using-the-compiler.html#input-description
    #[clap(long, num_args(1..), value_name = "SELECTOR")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra_output: Vec<ContractOutputSelection>,

    /// Extra output to write to separate files.
    ///
    /// Valid values: metadata, ir, irOptimized, ewasm, evm.assembly
    #[clap(long, num_args(1..), value_name = "SELECTOR")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub extra_output_files: Vec<ContractOutputSelection>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse_evm_version() {
        let args: CompilerArgs =
            CompilerArgs::parse_from(["foxar-cli", "--cvm-version", "nucleus"]);
        assert_eq!(args.cvm_version, Some(CvmVersion::Nucleus));
    }

    #[test]
    fn can_parse_extra_output() {
        let args: CompilerArgs =
            CompilerArgs::parse_from(["foxar-cli", "--extra-output", "metadata", "ir-optimized"]);
        assert_eq!(
            args.extra_output,
            vec![ContractOutputSelection::Metadata, ContractOutputSelection::IrOptimized]
        );
    }

    #[test]
    fn can_parse_extra_output_files() {
        let args: CompilerArgs = CompilerArgs::parse_from([
            "foxar-cli",
            "--extra-output-files",
            "metadata",
            "ir-optimized",
        ]);
        assert_eq!(
            args.extra_output_files,
            vec![ContractOutputSelection::Metadata, ContractOutputSelection::IrOptimized]
        );
    }
}
