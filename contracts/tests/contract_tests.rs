// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

//! Tests for the HelloBonsai contract using a mock for the Bonsai proxy contract.
//! TODO(victor) Fill in this file.

use std::ops::Deref;
use std::error::Error;
use std::sync::Arc;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::utils::{Ganache, GanacheInstance};
use hello_bonsai_contracts::HelloBonsai;
use risc0_zkvm::{serde, Prover, ProverOpts, sha::Digest};
use hello_bonsai_methods::{FIBONACCI_ID, FIBONACCI_PATH};


abigen!(MockBonsaiProxy, "artifacts/MockBonsaiProxy.sol/MockBonsaiProxy.json");

pub async fn get_ganache_client() -> Result<(
    GanacheInstance,
    Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>,
), Box<dyn Error>> {
    // Launch ganache instance
    let ganache = Ganache::new().spawn();

    // Instantiate wallet
    let wallet: LocalWallet = ganache.keys()[0].clone().into();

    // Connect to network
    let provider = Provider::<Ws>::connect(ganache.ws_endpoint()).await?;

    // Instantiate client as wallet on network
    let client = Arc::new(SignerMiddleware::new(
        provider,
        wallet.with_chain_id(1337u64),
    ));

    Ok((ganache, client))
}

#[tokio::test]
pub async fn test_happy_path() -> Result<(), Box<dyn Error>> {
    // Instantiate client as wallet on network
    let (_ganache, client) = get_ganache_client().await?;
    let _wallet_address = client.address();

    // TODO(victor): This is uglier than it needs to be because the image ID is encoded as [u32; 8]
    let image_id: [u8; 32] = Digest::from(FIBONACCI_ID).into();

    // Deploy the MockBonsaiProxy
    let mock_bonsai_proxy = MockBonsaiProxy::deploy(client.clone(), ())?
        .send()
        .await?;

    // Deploy the HelloBonsai contract.
    let hello_bonsai = HelloBonsai::deploy(
        client.clone(),
        (mock_bonsai_proxy.address(), image_id),
    )?
    .send()
    .await?;

    let events = mock_bonsai_proxy.events();
    let mut subscription = events.subscribe().await?;

    hello_bonsai.calculate_fibonacci(U256::from(10)).send().await?;

    let submit_request_log = subscription.next().await.unwrap()?;

    let mut prover = Prover::new_with_opts(
        &std::fs::read(FIBONACCI_PATH)?,
        FIBONACCI_ID,
        ProverOpts::default().with_skip_seal(true),
    )?;
    prover.add_input_u32_slice(&serde::to_vec(submit_request_log.input.deref())?);
    let receipt = prover.run()?;

    // Send a callback to HelloBonsai through the Bonsai proxy contract.
    let callback_selector = hello_bonsai.abi().function("calculate_fibonacci_callback")?.short_signature();
    mock_bonsai_proxy.send_callback(hello_bonsai.address(), callback_selector, image_id, receipt.journal.into()).send().await?;

    // Check that the journal is used to produce the expected state change.
    let result: U256 = hello_bonsai.fibonacci(U256::from(10)).call().await?;

    assert_eq!(result, U256::from(89));
    Ok(())
}
