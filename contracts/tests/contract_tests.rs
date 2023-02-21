// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

//! Tests for the HelloBonsai contract using a mock for the Bonsai proxy contract.
//! TODO(victor) Fill in this file.

use std::error::Error;
use std::sync::Arc;
use std::time::Duration;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::abi::Tokenizable;
use ethers::utils::{Ganache, GanacheInstance};
use hello_bonsai_contracts::HelloBonsai;

abigen!(MockBonsaiProxy, "artifacts/MockBonsaiProxy.sol/MockBonsaiProxy.json");

pub fn get_ganache_client() -> (
    GanacheInstance,
    Arc<SignerMiddleware<Provider<Http>, Wallet<SigningKey>>>,
) {
    // Launch ganache instance
    let ganache = Ganache::new().spawn();

    // Instantiate wallet
    let wallet: LocalWallet = ganache.keys()[0].clone().into();

    // Connect to network
    let provider = Provider::<Http>::try_from(ganache.endpoint())
        .unwrap()
        .interval(Duration::from_millis(10u64));

    // Instantiate client as wallet on network
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(1337u64),
    ));

    (ganache, client)
}

const MOCK_IMAGE_ID: [u8; 32] = [8u8; 32];

#[tokio::test]
pub async fn test_happy_path() -> Result<(), Box<dyn Error>> {
    // Instantiate client as wallet on network
    let (_ganache, client) = get_ganache_client();
    let _wallet_address = client.address();

    // Deploy the MockBonsaiProxy
    let mock_bonsai_proxy = MockBonsaiProxy::deploy(client.clone(), ())?
        .send()
        .await?;

    // Deploy the HelloBonsai contract.
    let hello_bonsai = HelloBonsai::deploy(
        client.clone(),
        (mock_bonsai_proxy.address(), MOCK_IMAGE_ID),
    )?
    .send()
    .await?;

    // Send a callback to HelloBonsai through the Bonsai proxy contract.
    let callback_selector = hello_bonsai.abi().function("calculate_fibonacci_callback")?.short_signature();
    let journal = ethers::abi::encode(&[U256::from(10).into_token(), U256::from(89).into_token()]);
    mock_bonsai_proxy.send_callback(hello_bonsai.address(), callback_selector, MOCK_IMAGE_ID, journal.into()).send().await?;

    // Check that the journal is used to produce the expected state change.
    let result: U256 = hello_bonsai.fibonacci(U256::from(10)).call().await?;

    assert_eq!(result, U256::from(89));
    Ok(())
}
