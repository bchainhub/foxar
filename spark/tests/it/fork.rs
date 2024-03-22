//! spark tests for cheat codes

use crate::{
    config::*,
    test_helpers::{filter::Filter, RE_PATH_SEPARATOR},
};
use spark::result::SuiteResult;

/// Executes reverting fork test
#[tokio::test(flavor = "multi_thread")]
async fn test_cheats_fork_revert() {
    let rpc_url: String = foxar_utils::rpc::next_http_archive_rpc_endpoint(corebc::types::Network::Mainnet);
    let mut runner = forked_runner(&rpc_url).await;
    let suite_result = runner
        .test(
            &Filter::new(
                "testNonExistingContractRevert",
                ".*",
                &format!(".*cheats{RE_PATH_SEPARATOR}Fork"),
            ),
            None,
            test_opts(),
        )
        .await;
    assert_eq!(suite_result.len(), 1);

    for (_, SuiteResult { test_results, .. }) in suite_result {
        for (_, result) in test_results {
            //CORETODO: check diagnostics error once we update ylem
            assert_eq!(
                result.reason.unwrap(),
                "Contract cb73a7f8ca25d95c21655c170b365c7a8df17b888add does not exist on active fork with id `2`\n        But exists on non active forks: `[1, 0]`"
            );
        }
    }
}

/// Executes all non-reverting fork cheatcodes
#[tokio::test(flavor = "multi_thread")]
async fn test_cheats_fork() {
    let rpc_url = foxar_utils::rpc::next_http_archive_rpc_endpoint(corebc::types::Network::Mainnet);
    let runner = forked_runner(&rpc_url).await;
    let filter = Filter::new(".*", ".*", &format!(".*cheats{RE_PATH_SEPARATOR}Fork"))
        .exclude_tests(".*Revert");
    TestConfig::with_filter(runner, filter).run().await;
}

/// Tests that we can launch in forking mode
#[tokio::test(flavor = "multi_thread")]
async fn test_launch_fork() {
    let rpc_url = foxar_utils::rpc::next_http_archive_rpc_endpoint(corebc::types::Network::Mainnet);
    let runner = forked_runner(&rpc_url).await;
    let filter = Filter::new(".*", ".*", &format!(".*fork{RE_PATH_SEPARATOR}Launch"));
    TestConfig::with_filter(runner, filter).run().await;
}

/// Tests that we can transact transactions in forking mode
#[tokio::test(flavor = "multi_thread")]
async fn test_transact_fork() {
    let filter = Filter::new(".*", ".*", &format!(".*fork{RE_PATH_SEPARATOR}Transact"));
    TestConfig::filter(filter).await.run().await;
}

/// Tests that we can create the same fork (provider,block) concurretnly in different tests
#[tokio::test(flavor = "multi_thread")]
async fn test_create_same_fork() {
    let filter = Filter::new(".*", ".*", &format!(".*fork{RE_PATH_SEPARATOR}ForkSame"));
    TestConfig::filter(filter).await.run().await;
}
