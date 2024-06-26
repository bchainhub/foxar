//! tests for custom shuttle endpoints
use crate::abi::*;
use corebc::{
    abi::ethereum_types::BigEndianHash,
    prelude::{Middleware, SignerMiddleware},
    types::{Address, BlockNumber, TransactionRequest, H256, U256, U64},
};
use shuttle::{spawn, Hardfork, NodeConfig};
use shuttle_core::{
    eth::EthRequest,
    types::{NodeEnvironment, NodeForkConfig, NodeInfo},
};
use spark::revm::primitives::SpecId;
use std::{
    str::FromStr,
    sync::Arc,
    time::{Duration, SystemTime},
};

#[tokio::test(flavor = "multi_thread")]
async fn can_set_energy_price() {
    let (api, handle) = spawn(NodeConfig::test().with_hardfork(Some(Hardfork::Istanbul))).await;
    let provider = handle.http_provider();

    let energy_price = 1337u64.into();
    api.shuttle_set_min_energy_price(energy_price).await.unwrap();
    assert_eq!(energy_price, provider.get_energy_price().await.unwrap());
}

#[tokio::test(flavor = "multi_thread")]
async fn can_set_block_energy_limit() {
    let (api, _) = spawn(NodeConfig::test().with_hardfork(Some(Hardfork::Istanbul))).await;

    let block_energy_limit = 1337u64.into();
    assert!(api.evm_set_block_energy_limit(block_energy_limit).unwrap());
    // Mine a new block, and check the new block energy limit
    api.mine_one().await;
    let latest_block = api.block_by_number(BlockNumber::Latest).await.unwrap().unwrap();
    assert_eq!(block_energy_limit, latest_block.energy_limit);
}

// Ref <https://github.com/foxar-rs/foxar/issues/2341>
#[tokio::test(flavor = "multi_thread")]
async fn can_set_storage() {
    let (api, _handle) = spawn(NodeConfig::test()).await;
    let s = r#"{"jsonrpc": "2.0", "method": "hardhat_setStorageAt", "id": 1, "params": ["0xcb66e9e7CEA3DedcA5984780Bafc599bD69ADd087D56", "0xa6eef7e35abe7026729641147f7915573c7e97b47efa546f5f6e3230263bcb49", "0x0000000000000000000000000000000000000000000000000000000000003039"]}"#;
    let req = serde_json::from_str::<EthRequest>(s).unwrap();
    let (addr, slot, val) = match req.clone() {
        EthRequest::SetStorageAt(addr, slot, val) => (addr, slot, val),
        _ => unreachable!(),
    };

    api.execute(req).await;

    let storage_value = api.storage_at(addr, slot, None).await.unwrap();
    assert_eq!(val, storage_value);
    assert_eq!(val, H256::from_uint(&U256::from(12345)));
}

#[tokio::test(flavor = "multi_thread")]
async fn can_impersonate_account() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let impersonate = Address::random();
    let to = Address::random();
    let val = 1337u64;
    let funding = U256::from(1e18 as u64);
    // fund the impersonated account
    api.shuttle_set_balance(impersonate, funding).await.unwrap();

    let balance = api.balance(impersonate, None).await.unwrap();
    assert_eq!(balance, funding);

    let tx = TransactionRequest::new().from(impersonate).to(to).value(val);

    let res = provider.send_transaction(tx.clone(), None).await;
    res.unwrap_err();

    api.shuttle_impersonate_account(impersonate).await.unwrap();

    let res = provider.send_transaction(tx.clone(), None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(res.from, impersonate);

    let nonce = provider.get_transaction_count(impersonate, None).await.unwrap();
    assert_eq!(nonce, 1u64.into());

    let balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance, val.into());

    api.shuttle_stop_impersonating_account(impersonate).await.unwrap();
    let res = provider.send_transaction(tx, None).await;
    res.unwrap_err();
}

#[tokio::test(flavor = "multi_thread")]
async fn can_auto_impersonate_account() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let impersonate = Address::random();
    let to = Address::random();
    let val = 1337u64;
    let funding = U256::from(1e18 as u64);
    // fund the impersonated account
    api.shuttle_set_balance(impersonate, funding).await.unwrap();

    let balance = api.balance(impersonate, None).await.unwrap();
    assert_eq!(balance, funding);

    let tx = TransactionRequest::new().from(impersonate).to(to).value(val);

    let res = provider.send_transaction(tx.clone(), None).await;
    res.unwrap_err();

    api.shuttle_auto_impersonate_account(true).await.unwrap();

    let res = provider.send_transaction(tx.clone(), None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(res.from, impersonate);

    let nonce = provider.get_transaction_count(impersonate, None).await.unwrap();
    assert_eq!(nonce, 1u64.into());

    let balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance, val.into());

    api.shuttle_auto_impersonate_account(false).await.unwrap();
    let res = provider.send_transaction(tx, None).await;
    res.unwrap_err();
}

#[tokio::test(flavor = "multi_thread")]
async fn can_impersonate_contract() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let wallet = handle.dev_wallets().next().unwrap();
    let provider = Arc::new(SignerMiddleware::new(provider, wallet));

    let box_contract = Box::deploy(provider, ()).unwrap().send().await.unwrap();
    // let greeter_contract =
    //     Greeter::deploy(provider, "Hello World!".to_string()).unwrap().send().await.unwrap();
    let impersonate = box_contract.address();

    let to = Address::random();
    let val = 1337u64;

    let provider = handle.http_provider();

    // fund the impersonated account
    api.shuttle_set_balance(impersonate, U256::from(1e18 as u64)).await.unwrap();

    let tx = TransactionRequest::new().from(impersonate).to(to).value(val);

    let res = provider.send_transaction(tx.clone(), None).await;
    res.unwrap_err();

    let _ = box_contract.store(U256::from(100)).send().await.unwrap();

    let stored_val = box_contract.get().call().await.unwrap();
    assert_eq!(U256::from(100), stored_val);

    api.shuttle_impersonate_account(impersonate).await.unwrap();

    let res = provider.send_transaction(tx.clone(), None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(res.from, impersonate);

    let balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance, val.into());

    api.shuttle_stop_impersonating_account(impersonate).await.unwrap();
    let res = provider.send_transaction(tx, None).await;
    res.unwrap_err();

    let stored_val = box_contract.get().call().await.unwrap();
    assert_eq!(U256::from(100), stored_val);
}

#[tokio::test(flavor = "multi_thread")]
async fn can_impersonate_multiple_account() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let impersonate0 = Address::random();
    let impersonate1 = Address::random();
    let to = Address::random();

    let val = 1337u64;
    let funding = U256::from(1e18 as u64);
    // fund the impersonated accounts
    api.shuttle_set_balance(impersonate0, funding).await.unwrap();
    api.shuttle_set_balance(impersonate1, funding).await.unwrap();

    let tx = TransactionRequest::new().from(impersonate0).to(to).value(val);

    api.shuttle_impersonate_account(impersonate0).await.unwrap();
    api.shuttle_impersonate_account(impersonate1).await.unwrap();

    let res0 = provider.send_transaction(tx.clone(), None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(res0.from, impersonate0);

    let nonce = provider.get_transaction_count(impersonate0, None).await.unwrap();
    assert_eq!(nonce, 1u64.into());

    let receipt = provider.get_transaction_receipt(res0.transaction_hash).await.unwrap().unwrap();
    assert_eq!(res0, receipt);

    let res1 = provider
        .send_transaction(tx.from(impersonate1), None)
        .await
        .unwrap()
        .await
        .unwrap()
        .unwrap();
    assert_eq!(res1.from, impersonate1);

    let nonce = provider.get_transaction_count(impersonate1, None).await.unwrap();
    assert_eq!(nonce, 1u64.into());

    let receipt = provider.get_transaction_receipt(res1.transaction_hash).await.unwrap().unwrap();
    assert_eq!(res1, receipt);

    assert_ne!(res0, res1);
}

#[tokio::test(flavor = "multi_thread")]
async fn can_mine_manually() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let start_num = provider.get_block_number().await.unwrap();

    for (idx, _) in std::iter::repeat(()).take(10).enumerate() {
        api.evm_mine(None).await.unwrap();
        let num = provider.get_block_number().await.unwrap();
        assert_eq!(num, start_num + idx + 1);
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_set_next_timestamp() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let next_timestamp = now + Duration::from_secs(60);

    // mock timestamp
    api.evm_set_next_block_timestamp(next_timestamp.as_secs()).unwrap();

    api.evm_mine(None).await.unwrap();

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

    assert_eq!(block.number.unwrap().as_u64(), 1);
    assert_eq!(block.timestamp.as_u64(), next_timestamp.as_secs());

    api.evm_mine(None).await.unwrap();

    let next = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    assert_eq!(next.number.unwrap().as_u64(), 2);

    assert!(next.timestamp > block.timestamp);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_evm_set_time() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let timestamp = now + Duration::from_secs(120);

    // mock timestamp
    api.evm_set_time(timestamp.as_secs()).unwrap();

    // mine a block
    api.evm_mine(None).await.unwrap();
    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

    assert!(block.timestamp.as_u64() >= timestamp.as_secs());

    api.evm_mine(None).await.unwrap();
    let next = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

    assert!(next.timestamp > block.timestamp);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_evm_set_time_in_past() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    let now = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

    let timestamp = now - Duration::from_secs(120);

    // mock timestamp
    api.evm_set_time(timestamp.as_secs()).unwrap();

    // mine a block
    api.evm_mine(None).await.unwrap();
    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

    assert!(block.timestamp.as_u64() >= timestamp.as_secs());
    assert!(block.timestamp.as_u64() < now.as_secs());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_timestamp_interval() {
    let (api, handle) = spawn(NodeConfig::test()).await;
    let provider = handle.http_provider();

    api.evm_mine(None).await.unwrap();
    let interval = 10;

    for _ in 0..5 {
        let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

        // mock timestamp
        api.evm_set_block_timestamp_interval(interval).unwrap();
        api.evm_mine(None).await.unwrap();

        let new_block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

        assert_eq!(new_block.timestamp, block.timestamp + interval);
    }

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

    let next_timestamp = block.timestamp + 50;
    api.evm_set_next_block_timestamp(next_timestamp.as_u64()).unwrap();

    api.evm_mine(None).await.unwrap();
    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    assert_eq!(block.timestamp, next_timestamp);

    api.evm_mine(None).await.unwrap();

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    // interval also works after setting the next timestamp manually
    assert_eq!(block.timestamp, next_timestamp + interval);

    assert!(api.evm_remove_block_timestamp_interval().unwrap());

    api.evm_mine(None).await.unwrap();
    let new_block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

    // offset is applied correctly after resetting the interval
    assert!(new_block.timestamp > block.timestamp);

    api.evm_mine(None).await.unwrap();
    let another_block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    // check interval is disabled
    assert!(another_block.timestamp - new_block.timestamp < U256::from(interval));
}

#[tokio::test(flavor = "multi_thread")]
async fn can_get_node_info() {
    let (api, handle) = spawn(NodeConfig::test()).await;

    let node_info = api.shuttle_node_info().await.unwrap();

    let provider = handle.http_provider();

    let block_number = provider.get_block_number().await.unwrap();
    let block = provider.get_block(block_number).await.unwrap().unwrap();

    let expected_node_info = NodeInfo {
        current_block_number: U64([0]),
        current_block_timestamp: 1,
        current_block_hash: block.hash.unwrap(),
        hard_fork: SpecId::ISTANBUL,
        transaction_order: "fees".to_owned(),
        environment: NodeEnvironment {
            chain_id: U256::from_str("0x1").unwrap(),
            energy_limit: U256::from_str("0x1c9c380").unwrap(),
            energy_price: U256::from_str("0x6FC23AC0").unwrap(),
        },
        fork_config: NodeForkConfig {
            fork_url: None,
            fork_block_number: None,
            fork_retry_backoff: None,
        },
    };

    assert_eq!(node_info, expected_node_info);
}
