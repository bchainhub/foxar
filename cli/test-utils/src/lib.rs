// Macros useful for testing.
mod macros;

// Utilities for making it easier to handle tests.
pub mod util;
pub use util::{TestCommand, TestProject};

pub mod script;
pub use script::{ScriptOutcome, ScriptTester};

// re-exports for convenience
pub use corebc_addressbook;
pub use corebc_ylem;
pub use tempfile;
