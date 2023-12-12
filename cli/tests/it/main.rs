#[cfg(not(feature = "external-integration-tests"))]
mod cache;
#[cfg(not(feature = "external-integration-tests"))]
mod cast;
#[cfg(not(feature = "external-integration-tests"))]
mod cmd;
#[cfg(not(feature = "external-integration-tests"))]
mod config;
#[cfg(not(feature = "external-integration-tests"))]
mod create;
#[cfg(not(feature = "external-integration-tests"))]
mod doc;
#[cfg(not(feature = "external-integration-tests"))]
mod multi_script;
#[cfg(not(feature = "external-integration-tests"))]
mod script;
#[cfg(not(feature = "external-integration-tests"))]
mod test_cmd;
#[cfg(not(feature = "external-integration-tests"))]
mod utils;
#[cfg(not(feature = "external-integration-tests"))]
mod verify;
mod yvm;

// import spark utils as mod
#[allow(unused)]
#[path = "../../src/utils.rs"]
pub(crate) mod spark_utils;

#[cfg(feature = "external-integration-tests")]
mod integration;

pub mod constants;

fn main() {}
