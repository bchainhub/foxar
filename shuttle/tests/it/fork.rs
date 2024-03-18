//! various fork related test

use crate::{abi::*, utils};
use corebc::{
    core::rand,
    prelude::{Bytes, LocalWallet, Middleware, SignerMiddleware},
    providers::{Http, Provider},
    signers::Signer,
    types::{
        transaction::eip2718::TypedTransaction, Address, BlockNumber, Network, TransactionRequest,
        U256,
    },
};
use foxar_common::get_http_provider;
use foxar_config::Config;
use foxar_utils::{rpc, rpc::next_http_rpc_endpoint, types::ToRuint};
use futures::StreamExt;
use shuttle::{eth::EthApi, spawn, NodeConfig, NodeHandle};
use shuttle_core::{eth::transaction::EthTransactionRequest, types::Forking};
use spark::utils::h176_to_b176;
use std::{sync::Arc, time::Duration};

const BLOCK_NUMBER: u64 = 7_650_092;

const BLOCK_TIMESTAMP: u64 = 1_706_518_238u64;

/// Represents an shuttle fork of an shuttle node
#[allow(unused)]
pub struct LocalFork {
    origin_api: EthApi,
    origin_handle: NodeHandle,
    fork_api: EthApi,
    fork_handle: NodeHandle,
}

// === impl LocalFork ===
#[allow(dead_code)]
impl LocalFork {
    /// Spawns two nodes with the test config
    pub async fn new() -> Self {
        Self::setup(NodeConfig::test(), NodeConfig::test()).await
    }

    /// Spawns two nodes where one is a fork of the other
    pub async fn setup(origin: NodeConfig, fork: NodeConfig) -> Self {
        let (origin_api, origin_handle) = spawn(origin).await;

        let (fork_api, fork_handle) =
            spawn(fork.with_eth_rpc_url(Some(origin_handle.http_endpoint()))).await;
        Self { origin_api, origin_handle, fork_api, fork_handle }
    }
}

pub fn fork_config() -> NodeConfig {
    NodeConfig::test()
        .with_eth_rpc_url(Some(rpc::next_http_archive_rpc_endpoint(Network::Mainnet)))
        .with_fork_block_number(Some(BLOCK_NUMBER))
        .silent()
}

#[tokio::test(flavor = "multi_thread")]
async fn test_spawn_fork() {
    let (api, _handle) = spawn(fork_config()).await;
    assert!(api.is_fork());

    let head = api.block_number().unwrap();
    assert_eq!(head, BLOCK_NUMBER.into())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_eth_get_balance() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();
    let addresses: Vec<&str> = vec![
        "cb1958b39698a44bdae37f881e68dce073823a48a631",
        "cb88632ed69c17d318372233bcbac849317f4de784e2",
        "cb71dda309844cb29462a4e7fd5d3ece3088323d178c",
        "cb06acf799cef468f3f745fb5c4e28fb8a85d3d979d5",
        "cb28cafa2121e896c9b0cb38af7c70c73e8a9d93835f",
        "cb16e5a14dccd951f7089f74c8ce3a9d4a6a9885df50",
        "cb49e375da0d25328e0f3204befb79802adb3ef9a57f",
        "cb842591f0c38226443271ccb179840102b3e6df51e7",
        "cb666be868bee93e5655d677169cc62f8d6475e15548",
        "cb32c8136d131e03e822cd31140391a27a5bd876f509",
    ];
    for i in 0..10 {
        let addr = addresses[i].parse().unwrap();
        let balance = api.balance(addr, None).await.unwrap();
        let provider_balance = provider.get_balance(addr, None).await.unwrap();
        assert_eq!(balance, provider_balance)
    }
}

// <https://github.com/foxar-rs/foxar/issues/4082>
#[tokio::test(flavor = "multi_thread")]
async fn test_fork_eth_get_balance_after_mine() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();
    let info = api.shuttle_node_info().await.unwrap();
    let number = info.fork_config.fork_block_number.unwrap();
    assert_eq!(number, BLOCK_NUMBER);

    let address: Address = "cb49e375da0d25328e0f3204befb79802adb3ef9a57f".parse().unwrap();

    let _balance = provider
        .get_balance(address, Some(BlockNumber::Number(number.into()).into()))
        .await
        .unwrap();

    api.evm_mine(None).await.unwrap();

    let _balance = provider
        .get_balance(address, Some(BlockNumber::Number(number.into()).into()))
        .await
        .unwrap();
}

// <https://github.com/foxar-rs/foxar/issues/4082>
#[tokio::test(flavor = "multi_thread")]
async fn test_fork_eth_get_code_after_mine() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();
    let info = api.shuttle_node_info().await.unwrap();
    let number = info.fork_config.fork_block_number.unwrap();
    assert_eq!(number, BLOCK_NUMBER);

    let address: Address = "cb49e375da0d25328e0f3204befb79802adb3ef9a57f".parse().unwrap();

    let _code =
        provider.get_code(address, Some(BlockNumber::Number(number.into()).into())).await.unwrap();

    api.evm_mine(None).await.unwrap();

    let _code =
        provider.get_code(address, Some(BlockNumber::Number(number.into()).into())).await.unwrap();
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_eth_get_code() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();
    let addresses: Vec<&str> = vec![
        "cb1958b39698a44bdae37f881e68dce073823a48a631",
        "cb88632ed69c17d318372233bcbac849317f4de784e2",
        "cb71dda309844cb29462a4e7fd5d3ece3088323d178c",
        "cb06acf799cef468f3f745fb5c4e28fb8a85d3d979d5",
        "cb28cafa2121e896c9b0cb38af7c70c73e8a9d93835f",
        "cb16e5a14dccd951f7089f74c8ce3a9d4a6a9885df50",
        "cb49e375da0d25328e0f3204befb79802adb3ef9a57f",
        "cb842591f0c38226443271ccb179840102b3e6df51e7",
        "cb666be868bee93e5655d677169cc62f8d6475e15548",
        "cb32c8136d131e03e822cd31140391a27a5bd876f509",
    ];
    for i in 0..10 {
        let addr = addresses[i].parse().unwrap();
        let code = api.get_code(addr, None).await.unwrap();
        let provider_code = provider.get_code(addr, None).await.unwrap();
        assert_eq!(code, provider_code)
    }

    for address in utils::contract_addresses(Network::Mainnet) {
        let prev_code = api
            .get_code(address, Some(BlockNumber::Number((BLOCK_NUMBER - 10).into()).into()))
            .await
            .unwrap();
        let code = api.get_code(address, None).await.unwrap();
        let provider_code = provider.get_code(address, None).await.unwrap();
        assert_eq!(code, prev_code);
        assert_eq!(code, provider_code);
        assert!(!code.as_ref().is_empty());
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_eth_get_nonce() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();

    let addresses: Vec<&str> = vec![
        "cb1958b39698a44bdae37f881e68dce073823a48a631",
        "cb88632ed69c17d318372233bcbac849317f4de784e2",
        "cb71dda309844cb29462a4e7fd5d3ece3088323d178c",
        "cb06acf799cef468f3f745fb5c4e28fb8a85d3d979d5",
        "cb28cafa2121e896c9b0cb38af7c70c73e8a9d93835f",
        "cb16e5a14dccd951f7089f74c8ce3a9d4a6a9885df50",
        "cb49e375da0d25328e0f3204befb79802adb3ef9a57f",
        "cb842591f0c38226443271ccb179840102b3e6df51e7",
        "cb666be868bee93e5655d677169cc62f8d6475e15548",
        "cb32c8136d131e03e822cd31140391a27a5bd876f509",
    ];
    for i in 0..10 {
        let addr = addresses[i].parse().unwrap();
        let api_nonce = api.transaction_count(addr, None).await.unwrap();
        let provider_nonce = provider.get_transaction_count(addr, None).await.unwrap();
        assert_eq!(api_nonce, provider_nonce);
    }

    let addr = Config::default_sender(Some(&Network::Mainnet));
    let api_nonce = api.transaction_count(addr, None).await.unwrap();
    let provider_nonce = provider.get_transaction_count(addr, None).await.unwrap();
    assert_eq!(api_nonce, provider_nonce);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_reset() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();

    let accounts: Vec<_> = handle.dev_wallets().collect();
    let from = accounts[0].address();
    let to = accounts[1].address();
    let block_number = provider.get_block_number().await.unwrap();
    let balance_before = provider.get_balance(to, None).await.unwrap();
    let amount = handle.genesis_balance().checked_div(2u64.into()).unwrap();

    let initial_nonce = provider.get_transaction_count(from, None).await.unwrap();

    let tx = TransactionRequest::new().to(to).value(amount).from(from);

    let tx = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(tx.transaction_index, 0u64.into());

    let nonce = provider.get_transaction_count(from, None).await.unwrap();

    assert_eq!(nonce, initial_nonce + 1);
    let to_balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance_before.saturating_add(amount), to_balance);
    api.shuttle_reset(Some(Forking {
        json_rpc_url: None,
        block_number: Some(block_number.as_u64()),
    }))
    .await
    .unwrap();

    // reset block number
    assert_eq!(block_number, provider.get_block_number().await.unwrap());

    let nonce = provider.get_transaction_count(from, None).await.unwrap();
    assert_eq!(nonce, initial_nonce);
    let balance = provider.get_balance(from, None).await.unwrap();
    assert_eq!(balance, handle.genesis_balance());
    let balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance, handle.genesis_balance());

    // reset to latest
    api.shuttle_reset(Some(Forking::default())).await.unwrap();

    let new_block_num = provider.get_block_number().await.unwrap();
    assert!(new_block_num > block_number);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_snapshotting() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();

    let snapshot = api.evm_snapshot().await.unwrap();

    let accounts: Vec<_> = handle.dev_wallets().collect();
    let from = accounts[0].address();
    let to = accounts[1].address();
    let block_number = provider.get_block_number().await.unwrap();

    let initial_nonce = provider.get_transaction_count(from, None).await.unwrap();
    let balance_before = provider.get_balance(to, None).await.unwrap();
    let amount = handle.genesis_balance().checked_div(2u64.into()).unwrap();

    let tx = TransactionRequest::new().to(to).value(amount).from(from);

    let _ = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();

    let nonce = provider.get_transaction_count(from, None).await.unwrap();
    assert_eq!(nonce, initial_nonce + 1);
    let to_balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance_before.saturating_add(amount), to_balance);

    assert!(api.evm_revert(snapshot).await.unwrap());

    let nonce = provider.get_transaction_count(from, None).await.unwrap();
    assert_eq!(nonce, initial_nonce);
    let balance = provider.get_balance(from, None).await.unwrap();
    assert_eq!(balance, handle.genesis_balance());
    let balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance, handle.genesis_balance());
    assert_eq!(block_number, provider.get_block_number().await.unwrap());
}

/// tests that the remote state and local state are kept separate.
/// changes don't make into the read only Database that holds the remote state, which is flushed to
/// a cache file.
#[tokio::test(flavor = "multi_thread")]
async fn test_separate_states() {
    let (api, handle) = spawn(fork_config().with_fork_block_number(Some(7650530u64))).await;
    let provider = handle.http_provider();

    let addr: Address = "cb64902ddc075d1d4dc16a1b65e24bed918abcf678a9".parse().unwrap();

    let remote_balance = provider.get_balance(addr, None).await.unwrap();
    assert_eq!(remote_balance, 33902228362032000000000u128.into());

    api.shuttle_set_balance(addr, 1337u64.into()).await.unwrap();
    let balance = provider.get_balance(addr, None).await.unwrap();
    assert_eq!(balance, 1337u64.into());

    let fork = api.get_fork().unwrap();
    let fork_db = fork.database.read().await;
    let acc = fork_db.inner().db().accounts.read().get(&h176_to_b176(addr)).cloned().unwrap();

    assert_eq!(acc.balance, remote_balance.to_ruint())
}

#[tokio::test(flavor = "multi_thread")]
async fn can_deploy_greeter_on_fork() {
    let (_api, handle) = spawn(fork_config().with_fork_block_number(Some(7650530u64))).await;
    let provider = handle.http_provider();

    let wallet = handle.dev_wallets().next().unwrap();
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let greeter_contract = Greeter::deploy(Arc::clone(&client), "Hello World!".to_string())
        .unwrap()
        .send()
        .await
        .unwrap();

    let greeting = greeter_contract.greet().call().await.unwrap();
    assert_eq!("Hello World!", greeting);

    let greeter_contract =
        Greeter::deploy(client, "Hello World!".to_string()).unwrap().send().await.unwrap();

    let greeting = greeter_contract.greet().call().await.unwrap();
    assert_eq!("Hello World!", greeting);
}

#[tokio::test(flavor = "multi_thread")]
async fn can_reset_properly() {
    let (origin_api, origin_handle) = spawn(NodeConfig::test()).await;
    let account = origin_handle.dev_accounts().next().unwrap();
    let origin_provider = origin_handle.http_provider();
    let origin_nonce = 1u64.into();
    origin_api.shuttle_set_nonce(account, origin_nonce).await.unwrap();

    assert_eq!(origin_nonce, origin_provider.get_transaction_count(account, None).await.unwrap());

    let (fork_api, fork_handle) =
        spawn(NodeConfig::test().with_eth_rpc_url(Some(origin_handle.http_endpoint()))).await;

    let fork_provider = fork_handle.http_provider();
    assert_eq!(origin_nonce, fork_provider.get_transaction_count(account, None).await.unwrap());

    let to = Address::random();
    let to_balance = fork_provider.get_balance(to, None).await.unwrap();
    let tx = TransactionRequest::new().from(account).to(to).value(1337u64);
    let tx = fork_provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();

    // nonce incremented by 1
    assert_eq!(origin_nonce + 1, fork_provider.get_transaction_count(account, None).await.unwrap());

    // resetting to origin state
    fork_api.shuttle_reset(Some(Forking::default())).await.unwrap();

    // nonce reset to origin
    assert_eq!(origin_nonce, fork_provider.get_transaction_count(account, None).await.unwrap());

    // balance is reset
    assert_eq!(to_balance, fork_provider.get_balance(to, None).await.unwrap());

    // tx does not exist anymore
    assert!(fork_provider.get_transaction(tx.transaction_hash).await.unwrap().is_none())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_timestamp() {
    let addr1: Address = "cb1958b39698a44bdae37f881e68dce073823a48a631".parse().unwrap();
    let addr2: Address = "cb5073f9dbe29c3e5da2d1ec40e3e528229b7d8a0faa".parse().unwrap();
    let addr3: Address = "cb71dda309844cb29462a4e7fd5d3ece3088323d178c".parse().unwrap();
    let addr4: Address = "cb32c8136d131e03e822cd31140391a27a5bd876f509".parse().unwrap();

    let start = std::time::Instant::now();

    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();

    let block = provider.get_block(BLOCK_NUMBER).await.unwrap().unwrap();
    assert_eq!(block.timestamp.as_u64(), BLOCK_TIMESTAMP);

    let accounts: Vec<_> = handle.dev_wallets().collect();
    let from = accounts[0].address();

    let tx = TransactionRequest::new().to(addr4).value(1337u64).from(from);
    let tx = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(tx.status, Some(1u64.into()));

    let elapsed = start.elapsed().as_secs();

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();

    // ensure the diff between the new mined block and the original block is within the elapsed time
    let diff = block.timestamp - BLOCK_TIMESTAMP;
    assert!(diff <= elapsed.into(), "diff={diff}, elapsed={elapsed}");

    let start = std::time::Instant::now();
    // reset to check timestamp works after resetting
    api.shuttle_reset(Some(Forking { json_rpc_url: None, block_number: Some(BLOCK_NUMBER) }))
        .await
        .unwrap();
    let block = provider.get_block(BLOCK_NUMBER).await.unwrap().unwrap();
    assert_eq!(block.timestamp.as_u64(), BLOCK_TIMESTAMP);
    let tx = TransactionRequest::new().to(addr1).value(1337u64).from(from);
    let _tx = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    let elapsed = start.elapsed().as_secs() + 1;
    let diff = block.timestamp - BLOCK_TIMESTAMP;
    assert!(diff <= elapsed.into());

    // ensure that after setting a timestamp manually, then next block time is correct
    let start = std::time::Instant::now();
    api.shuttle_reset(Some(Forking { json_rpc_url: None, block_number: Some(BLOCK_NUMBER) }))
        .await
        .unwrap();
    api.evm_set_next_block_timestamp(BLOCK_TIMESTAMP + 1).unwrap();
    let tx = TransactionRequest::new().to(addr2).value(1337u64).from(from);
    let _tx = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    assert_eq!(block.timestamp.as_u64(), BLOCK_TIMESTAMP + 1);

    let tx = TransactionRequest::new().to(addr3).value(1337u64).from(from);
    let _tx = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    let elapsed = start.elapsed().as_secs() + 1;
    let diff = block.timestamp - (BLOCK_TIMESTAMP + 1);
    assert!(diff <= elapsed.into());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_set_empty_code() {
    let (api, _handle) = spawn(fork_config()).await;
    let addr = "cb88632ed69c17d318372233bcbac849317f4de784e2".parse().unwrap();
    let code = api.get_code(addr, None).await.unwrap();
    assert!(!code.as_ref().is_empty());
    api.shuttle_set_code(addr, Vec::new().into()).await.unwrap();
    let code = api.get_code(addr, None).await.unwrap();
    assert!(code.as_ref().is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_can_send_tx() {
    let (api, handle) =
        spawn(fork_config().with_blocktime(Some(std::time::Duration::from_millis(800)))).await;

    let wallet = LocalWallet::new(&mut rand::thread_rng(), Network::Mainnet);

    api.shuttle_set_balance(wallet.address(), U256::from(1e18 as u64)).await.unwrap();

    let provider = SignerMiddleware::new(handle.http_provider(), wallet);

    let addr: Address = "cb974637858fe37c832187bbcc8f03b280f16c87438e".parse().unwrap();
    api.shuttle_set_balance(addr, U256::from(0 as u64)).await.unwrap();

    let val = 1u128;
    let tx = TransactionRequest::new().to(addr).value(val);

    // broadcast it via the eth_sendTransaction API
    let _ = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();

    let balance = provider.get_balance(addr, None).await.unwrap();
    assert_eq!(balance, val.into());
}

// <https://github.com/foxar-rs/foxar/issues/1920>
#[tokio::test(flavor = "multi_thread")]
#[ignore = "Need to find some NFTs to test with"]
async fn test_fork_nft_set_approve_all() {
    let (api, handle) = spawn(
        fork_config()
            .with_fork_block_number(Some(7650530u64))
            .with_blocktime(Some(Duration::from_secs(5)))
            .with_chain_id(1u64.into()),
    )
    .await;

    // create and fund a random wallet
    let wallet = LocalWallet::new(&mut rand::thread_rng(), Network::Mainnet);
    api.shuttle_set_balance(wallet.address(), U256::from(1000e18 as u64)).await.unwrap();

    let provider = Arc::new(SignerMiddleware::new(handle.http_provider(), wallet.clone()));

    // pick a random nft <https://opensea.io/assets/ethereum/0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03/154>
    let nouns_addr: Address = "0x9c8ff314c9bc7f6e59a9d9225fb22946427edc03".parse().unwrap();

    let owner: Address = "0x052564eb0fd8b340803df55def89c25c432f43f4".parse().unwrap();
    let token_id: U256 = 154u64.into();

    let nouns = Erc721::new(nouns_addr, Arc::clone(&provider));

    let real_onwer = nouns.owner_of(token_id).call().await.unwrap();
    assert_eq!(real_onwer, owner);
    let approval = nouns.set_approval_for_all(nouns_addr, true);
    let tx = approval.send().await.unwrap().await.unwrap().unwrap();
    assert_eq!(tx.status, Some(1u64.into()));

    // transfer: impersonate real owner and transfer nft
    api.shuttle_impersonate_account(real_onwer).await.unwrap();

    api.shuttle_set_balance(real_onwer, U256::from(10000e18 as u64)).await.unwrap();

    let call = nouns.transfer_from(real_onwer, wallet.address(), token_id);
    let mut tx: TypedTransaction = call.tx;
    tx.set_from(real_onwer);
    provider.fill_transaction(&mut tx, None).await.unwrap();
    let tx = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(tx.status, Some(1u64.into()));

    let real_onwer = nouns.owner_of(token_id).call().await.unwrap();
    assert_eq!(real_onwer, wallet.address());
}

// <https://github.com/foxar-rs/foxar/issues/2261>
#[tokio::test(flavor = "multi_thread")]
async fn test_fork_with_custom_chain_id() {
    // spawn a forked node with some random chainId
    let (api, handle) = spawn(
        fork_config()
            .with_fork_block_number(Some(3123u64))
            .with_blocktime(Some(Duration::from_secs(5)))
            .with_chain_id(3u64.into())
            .with_eth_rpc_url(Some(rpc::next_http_archive_rpc_endpoint(Network::Devin))),
    )
    .await;

    // get the eth chainId and the txn chainId
    let eth_chain_id = api.eth_chain_id();
    let txn_chain_id = api.chain_id();

    // get the chainId in the config
    let config_chain_id = handle.config().chain_id;

    // check that the chainIds are the same
    assert_eq!(eth_chain_id.unwrap().unwrap().as_u64(), 3u64);
    assert_eq!(txn_chain_id, 3u64);
    assert_eq!(config_chain_id, Some(3u64));
}

// <https://github.com/foxar-rs/foxar/issues/1920>
#[tokio::test(flavor = "multi_thread")]
#[ignore = "We do not support OpenSea"]
async fn test_fork_can_send_opensea_tx() {
    let (api, handle) = spawn(
        fork_config()
            .with_fork_block_number(Some(7650530u64))
            .with_blocktime(Some(Duration::from_millis(5000))),
    )
    .await;

    let sender: Address = "0x8fdbae54b6d9f3fc2c649e3dd4602961967fd42f".parse().unwrap();

    // transfer: impersonate real sender
    api.shuttle_impersonate_account(sender).await.unwrap();

    let provider = handle.http_provider();

    let input: Bytes = "0xfb0f3ee1000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000003ff2e795f5000000000000000000000000000023f28ae3e9756ba982a6290f9081b6a84900b758000000000000000000000000004c00500000ad104d7dbd00e3ae0a5c00560c0000000000000000000000000003235b597a78eabcb08ffcb4d97411073211dbcb0000000000000000000000000000000000000000000000000000000000000e72000000000000000000000000000000000000000000000000000000000000000100000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000062ad47c20000000000000000000000000000000000000000000000000000000062d43104000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000df44e65d2a2cf40000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f00000000007b02230091a7ed01230072f7006a004d60a8d4e71d599b8104250f00000000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000024000000000000000000000000000000000000000000000000000000000000002e000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000001c6bf526340000000000000000000000000008de9c5a032463c561423387a9648c5c7bcc5bc900000000000000000000000000000000000000000000000000005543df729c0000000000000000000000000006eb234847a9e3a546539aac57a071c01dc3f398600000000000000000000000000000000000000000000000000000000000000416d39b5352353a22cf2d44faa696c2089b03137a13b5acfee0366306f2678fede043bc8c7e422f6f13a3453295a4a063dac7ee6216ab7bade299690afc77397a51c00000000000000000000000000000000000000000000000000000000000000".parse().unwrap();
    let to: Address = "0x00000000006c3852cbef3e08e8df289169ede581".parse().unwrap();
    let tx = TransactionRequest::new()
        .from(sender)
        .to(to)
        .value(20000000000000000u64)
        .data(input)
        .energy_price(22180711707u64)
        .energy(150_000u64);

    let tx = provider.send_transaction(tx, None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(tx.status, Some(1u64.into()));
}

#[tokio::test(flavor = "multi_thread")]
async fn test_reset_fork_on_new_blocks() {
    let (api, handle) = spawn(
        NodeConfig::test()
            .with_eth_rpc_url(Some(rpc::next_http_archive_rpc_endpoint(Network::Mainnet)))
            .silent(),
    )
    .await;

    let shuttle_provider = handle.http_provider();

    let endpoint = next_http_rpc_endpoint(Network::Mainnet);
    let provider = Arc::new(get_http_provider(&endpoint).interval(Duration::from_secs(2)));

    let current_block = shuttle_provider.get_block_number().await.unwrap();

    handle.task_manager().spawn_reset_on_new_polled_blocks(provider.clone(), api);

    let mut stream = provider.watch_blocks().await.unwrap();
    // the http watcher may fetch multiple blocks at once, so we set a timeout here to offset edge
    // cases where the stream immediately returns a block
    tokio::time::sleep(
        Network::Mainnet
            .average_blocktime_hint()
            .unwrap()
            .saturating_add(Network::Mainnet.average_blocktime_hint().unwrap()),
    )
    .await;
    stream.next().await.unwrap();
    stream.next().await.unwrap();

    let next_block = shuttle_provider.get_block_number().await.unwrap();

    assert!(next_block > current_block, "nextblock={next_block} currentblock={current_block}")
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_call() {
    let input: Bytes = "0xca725b7e00000000000000000000000000000000000000000000000000b351fc8ec39c00"
        .parse()
        .unwrap();
    let to: Address = "0xcb06acf799cef468f3f745fb5c4e28fb8a85d3d979d5".parse().unwrap();
    let from: Address = "0xcb71dda309844cb29462a4e7fd5d3ece3088323d178c".parse().unwrap();

    let block_number = 7650904u64;

    let provider =
        Provider::<Http>::try_from(rpc::next_http_archive_rpc_endpoint(Network::Mainnet)).unwrap();
    let mut tx = TypedTransaction::default();
    tx.set_to(to).set_data(input.clone()).set_from(from);
    let res0 =
        provider.call(&tx, Some(BlockNumber::Number(block_number.into()).into())).await.unwrap();

    let (api, _) = spawn(fork_config().with_fork_block_number(Some(block_number))).await;

    let res1 = api
        .call(
            EthTransactionRequest {
                to: Some(to),
                data: Some(input),
                from: Some(from),
                ..Default::default()
            },
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(res0, res1);
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_block_timestamp() {
    let (api, _) = spawn(fork_config()).await;

    let initial_block = api.block_by_number(BlockNumber::Latest).await.unwrap().unwrap();
    api.shuttle_mine(Some(1.into()), None).await.unwrap();
    let latest_block = api.block_by_number(BlockNumber::Latest).await.unwrap().unwrap();

    assert!(initial_block.timestamp.as_u64() < latest_block.timestamp.as_u64());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_snapshot_block_timestamp() {
    let (api, _) = spawn(fork_config()).await;

    let snapshot_id = api.evm_snapshot().await.unwrap();
    api.shuttle_mine(Some(1.into()), None).await.unwrap();
    let initial_block = api.block_by_number(BlockNumber::Latest).await.unwrap().unwrap();
    api.evm_revert(snapshot_id).await.unwrap();
    api.evm_set_next_block_timestamp(initial_block.timestamp.as_u64()).unwrap();
    api.shuttle_mine(Some(1.into()), None).await.unwrap();
    let latest_block = api.block_by_number(BlockNumber::Latest).await.unwrap().unwrap();

    assert_eq!(initial_block.timestamp.as_u64(), latest_block.timestamp.as_u64());
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_uncles_fetch() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();

    // Block on XCB mainnet with uncle
    let block_with_uncles = 7650025u64;

    let block =
        api.block_by_number(BlockNumber::Number(block_with_uncles.into())).await.unwrap().unwrap();

    assert_eq!(block.uncles.len(), 1);

    let count = provider.get_uncle_count(block_with_uncles).await.unwrap();
    assert_eq!(count.as_usize(), block.uncles.len());

    let count = provider.get_uncle_count(block.hash.unwrap()).await.unwrap();
    assert_eq!(count.as_usize(), block.uncles.len());

    for (uncle_idx, uncle_hash) in block.uncles.iter().enumerate() {
        // Try with block number
        let uncle = provider
            .get_uncle(block_with_uncles, (uncle_idx as u64).into())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(*uncle_hash, uncle.hash.unwrap());

        // Try with block hash
        let uncle = provider
            .get_uncle(block.hash.unwrap(), (uncle_idx as u64).into())
            .await
            .unwrap()
            .unwrap();
        assert_eq!(*uncle_hash, uncle.hash.unwrap());
    }
}

#[tokio::test(flavor = "multi_thread")]
async fn test_fork_block_transaction_count() {
    let (api, handle) = spawn(fork_config()).await;
    let provider = handle.http_provider();

    let accounts: Vec<_> = handle.dev_wallets().collect();
    let sender = accounts[0].address();

    // disable automine (so there are pending transactions)
    api.shuttle_set_auto_mine(false).await.unwrap();
    // transfer: impersonate real sender
    api.shuttle_impersonate_account(sender).await.unwrap();

    let tx = TransactionRequest::new().from(sender).value(42u64).energy(100_000);
    provider.send_transaction(tx, None).await.unwrap();

    let pending_txs =
        api.block_transaction_count_by_number(BlockNumber::Pending).await.unwrap().unwrap();
    assert_eq!(pending_txs.as_usize(), 1);

    // mine a new block
    api.shuttle_mine(None, None).await.unwrap();

    let pending_txs =
        api.block_transaction_count_by_number(BlockNumber::Pending).await.unwrap().unwrap();
    assert_eq!(pending_txs.as_usize(), 0);
    let latest_txs =
        api.block_transaction_count_by_number(BlockNumber::Latest).await.unwrap().unwrap();
    assert_eq!(latest_txs.as_usize(), 1);
    let latest_block = api.block_by_number(BlockNumber::Latest).await.unwrap().unwrap();
    let latest_txs =
        api.block_transaction_count_by_hash(latest_block.hash.unwrap()).await.unwrap().unwrap();
    assert_eq!(latest_txs.as_usize(), 1);

    // check txs count on an older block: 7650035 has 2 txs on mainnet
    let count_txs = api
        .block_transaction_count_by_number(BlockNumber::Number(7650035.into()))
        .await
        .unwrap()
        .unwrap();
    assert_eq!(count_txs.as_usize(), 2);
    let count_txs = api
        .block_transaction_count_by_hash(
            "0x59b1cb57b6d07cbe1461f40160c90c7974e8ba8c98dfc0870d0910d4e48488cd".parse().unwrap(),
        )
        .await
        .unwrap()
        .unwrap();
    assert_eq!(count_txs.as_usize(), 2);
}

// <https://github.com/foxar-rs/foxar/issues/2931>
#[tokio::test(flavor = "multi_thread")]
async fn can_impersonate_in_fork() {
    let (api, handle) = spawn(fork_config().with_fork_block_number(Some(7650947u64))).await;
    let provider = handle.http_provider();

    let token_holder: Address = "0xcb1958b39698a44bdae37f881e68dce073823a48a631".parse().unwrap();
    let to: Address = "cb71dda309844cb29462a4e7fd5d3ece3088323d178c".parse().unwrap();
    api.shuttle_set_balance(to, U256::from(0 as u64)).await.unwrap();

    let val = 1337u64;

    // fund the impersonated account
    api.shuttle_set_balance(token_holder, U256::from(1e18 as u64)).await.unwrap();

    let tx = TransactionRequest::new().from(token_holder).to(to).value(val);

    let res = provider.send_transaction(tx.clone(), None).await;
    res.unwrap_err();

    api.shuttle_impersonate_account(token_holder).await.unwrap();

    let res = provider.send_transaction(tx.clone(), None).await.unwrap().await.unwrap().unwrap();
    assert_eq!(res.from, token_holder);
    assert_eq!(res.status, Some(1u64.into()));

    let balance = provider.get_balance(to, None).await.unwrap();
    assert_eq!(balance, val.into());

    api.shuttle_stop_impersonating_account(token_holder).await.unwrap();
    let res = provider.send_transaction(tx, None).await;
    res.unwrap_err();
}

// <https://etherscan.io/block/14608400>
#[tokio::test(flavor = "multi_thread")]
async fn test_total_difficulty_fork() {
    let (api, handle) = spawn(fork_config()).await;

    let total_difficulty: U256 = 1_583_475_154_100_364u128.into();
    let difficulty: U256 = 777_568_263u128.into();

    let provider = handle.http_provider();
    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    assert_eq!(block.total_difficulty, Some(total_difficulty));
    assert_eq!(block.difficulty, difficulty);

    api.mine_one().await;
    api.mine_one().await;

    let next_total_difficulty = U256::from(1_583_475_154_100_364u128);

    let block = provider.get_block(BlockNumber::Latest).await.unwrap().unwrap();
    assert_eq!(block.total_difficulty, Some(next_total_difficulty));
    assert_eq!(block.difficulty, U256::zero());
}

// <https://etherscan.io/block/14608400>
#[tokio::test(flavor = "multi_thread")]
async fn test_transaction_receipt() {
    let (api, _) = spawn(fork_config()).await;

    // A transaction before the forked block (7650069)
    let receipt = api
        .transaction_receipt(
            "0x62e58fe4e286a27c90904c88fa52b480ad9566a92ce1bb68737b295960f6a185".parse().unwrap(),
        )
        .await
        .unwrap();
    assert!(receipt.is_some());

    // A transaction from a block in the future (	7651080)
    let receipt = api
        .transaction_receipt(
            "0x2ada1def2ca2083de908cadf9893b76deac39fd7ad748812b83bb54a4c24cfbf".parse().unwrap(),
        )
        .await
        .unwrap();
    assert!(receipt.is_none());
}

#[tokio::test(flavor = "multi_thread")]
async fn can_override_fork_chain_id() {
    let chain_id_override = 3u64;
    let (_api, handle) = spawn(
        fork_config()
            .with_fork_block_number(Some(145555u64))
            .with_chain_id(Some(chain_id_override))
            .with_eth_rpc_url(Some(rpc::next_http_archive_rpc_endpoint(Network::Devin))),
    )
    .await;
    let provider = handle.http_provider();

    let wallet = handle.dev_wallets().next().unwrap();
    let client = Arc::new(SignerMiddleware::new(provider, wallet));

    let greeter_contract = Greeter::deploy(Arc::clone(&client), "Hello World!".to_string())
        .unwrap()
        .send()
        .await
        .unwrap();

    let greeting = greeter_contract.greet().call().await.unwrap();
    assert_eq!("Hello World!", greeting);

    let greeter_contract =
        Greeter::deploy(client, "Hello World!".to_string()).unwrap().send().await.unwrap();

    let greeting = greeter_contract.greet().call().await.unwrap();
    assert_eq!("Hello World!", greeting);

    let provider = handle.http_provider();
    let chain_id = provider.get_networkid().await.unwrap();
    assert_eq!(chain_id.as_u64(), chain_id_override);
}
