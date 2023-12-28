//! PilotCommand
//!
//! This module holds the [PilotCommand] enum, which contains all builtin commands that
//! can be executed within the REPL.

use crate::prelude::PilotDispatcher;
use std::{error::Error, str::FromStr};
use strum::EnumIter;

/// Builtin pilot command variants
#[derive(Debug, EnumIter)]
pub enum PilotCommand {
    /// Print helpful information about pilot
    Help,
    /// Quit the REPL
    Quit,
    /// Clear the current session source
    Clear,
    /// Print the generated source contract
    Source,
    /// Save the current session to the cache
    /// Takes: [session-id]
    Save,
    /// Load a previous session from cache
    /// Takes: <session-id>
    ///
    /// WARNING: This will overwrite the current session (though the current session will be
    /// optimistically cached)
    Load,
    /// List all cached sessions
    ListSessions,
    /// Clear the cache of all stored sessions
    ClearCache,
    /// Fork an RPC in the current session
    /// Takes <fork-url|env-var|rpc_endpoints-alias>
    Fork,
    /// Enable / disable traces for the current session
    Traces,
    /// Set calldata (`msg.data`) for the current session (appended after function selector)
    Calldata,
    /// Dump the raw memory
    MemDump,
    /// Dump the raw stack
    StackDump,
    /// Export the current REPL session source to a Script file
    Export,
    /// Fetch an interface of a verified contract on Etherscan
    /// Takes: <addr> <interface-name>
    //todo:error2215 commented out (waiting for blockindex implementation)
    // Fetch,
    /// Executes a shell command
    Exec,
    /// Display the raw value of a variable's stack allocation.
    RawStack,
    /// Open the current session in an editor
    Edit,
}

/// Attempt to convert a string slice to a `PilotCommand`
impl FromStr for PilotCommand {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_ref() {
            "help" | "h" => Ok(PilotCommand::Help),
            "quit" | "q" => Ok(PilotCommand::Quit),
            "clear" | "c" => Ok(PilotCommand::Clear),
            "source" | "so" => Ok(PilotCommand::Source),
            "save" | "s" => Ok(PilotCommand::Save),
            "list" | "ls" => Ok(PilotCommand::ListSessions),
            "load" | "l" => Ok(PilotCommand::Load),
            "clearcache" | "cc" => Ok(PilotCommand::ClearCache),
            "fork" | "f" => Ok(PilotCommand::Fork),
            "traces" | "t" => Ok(PilotCommand::Traces),
            "calldata" | "cd" => Ok(PilotCommand::Calldata),
            "memdump" | "md" => Ok(PilotCommand::MemDump),
            "stackdump" | "sd" => Ok(PilotCommand::StackDump),
            "export" | "ex" => Ok(PilotCommand::Export),
            //todo:error2215 commented out (waiting for blockindex implementation)
            // "fetch" | "fe" => Ok(PilotCommand::Fetch),
            "exec" | "e" => Ok(PilotCommand::Exec),
            "rawstack" | "rs" => Ok(PilotCommand::RawStack),
            "edit" => Ok(PilotCommand::Edit),
            _ => Err(PilotDispatcher::make_error(format!(
                "Unknown command \"{s}\"! See available commands with `!help`.",
            ))
            .into()),
        }
    }
}

/// A category for [PilotCommand]s
#[derive(Debug, EnumIter)]
pub enum CmdCategory {
    /// General category
    General,
    /// Session category
    Session,
    /// Environment category
    Env,
    /// Debug category
    Debug,
}

impl core::fmt::Display for CmdCategory {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let string = match self {
            CmdCategory::General => "General",
            CmdCategory::Session => "Session",
            CmdCategory::Env => "Environment",
            CmdCategory::Debug => "Debug",
        };
        f.write_str(string)
    }
}

/// A command descriptor type
pub type CmdDescriptor = (&'static [&'static str], &'static str, CmdCategory);

/// Convert a `PilotCommand` into a `CmdDescriptor` tuple
impl From<PilotCommand> for CmdDescriptor {
    fn from(cmd: PilotCommand) -> Self {
        match cmd {
            // General
            PilotCommand::Help => (&["help", "h"], "Display all commands", CmdCategory::General),
            PilotCommand::Quit => (&["quit", "q"], "Quit Pilot", CmdCategory::General),
            PilotCommand::Exec => (&["exec <command> [args]", "e <command> [args]"], "Execute a shell command and print the output", CmdCategory::General),
            // Session
            PilotCommand::Clear => (&["clear", "c"], "Clear current session source", CmdCategory::Session),
            PilotCommand::Source => (&["source", "so"], "Display the source code of the current session", CmdCategory::Session),
            PilotCommand::Save => (&["save [id]", "s [id]"], "Save the current session to cache", CmdCategory::Session),
            PilotCommand::Load => (&["load <id>", "l <id>"], "Load a previous session ID from cache", CmdCategory::Session),
            PilotCommand::ListSessions => (&["list", "ls"], "List all cached sessions", CmdCategory::Session),
            PilotCommand::ClearCache => (&["clearcache", "cc"], "Clear the pilot cache of all stored sessions", CmdCategory::Session),
            PilotCommand::Export => (&["export", "ex"], "Export the current session source to a script file", CmdCategory::Session),
            //todo:error2215 commented out (waiting for blockindex implementation)
            // PilotCommand::Fetch => (&["fetch <addr> <name>", "fe <addr> <name>"], "Fetch the interface of a verified contract on Etherscan", CmdCategory::Session),
            // Environment
            PilotCommand::Fork => (&["fork <url>", "f <url>"], "Fork an RPC for the current session. Supply 0 arguments to return to a local network", CmdCategory::Env),
            PilotCommand::Traces => (&["traces", "t"], "Enable / disable traces for the current session", CmdCategory::Env),
            PilotCommand::Calldata => (&["calldata [data]", "cd [data]"], "Set calldata (`msg.data`) for the current session (appended after function selector). Clears it if no argument provided.", CmdCategory::Env),
            // Debug
            PilotCommand::MemDump => (&["memdump", "md"], "Dump the raw memory of the current state", CmdCategory::Debug),
            PilotCommand::StackDump => (&["stackdump", "sd"], "Dump the raw stack of the current state", CmdCategory::Debug),
            PilotCommand::Edit => (&["edit"], "Open the current session in an editor", CmdCategory::Session),
            PilotCommand::RawStack => (&["rawstack <var>", "rs <var>"], "Display the raw value of a variable's stack allocation. For variables that are > 32 bytes in length, this will display their memory pointer.", CmdCategory::Debug),
        }
    }
}
