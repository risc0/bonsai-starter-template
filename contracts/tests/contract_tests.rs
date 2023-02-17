// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

use std::sync::Arc;
use std::time::Duration;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::abi::ethereum_types::Secret;
use ethers::utils::{Ganache, GanacheInstance};
use crate::HelloBonsai;

//! Tests for the HelloBonsai contract using a mock for the Bonsai proxy contract.
//! TODO(victor) Fill in this file.

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

#[tokio::test]
pub async fn test_happy_path() {
    // Instantiate client as wallet on network
    let (_ganache, client) = get_ganache_client();
    let wallet_address = client.address();

    /* Deploy dummies
    let dummy_erc20 = ERC20Dummy::deploy(client.clone(), ())
        .unwrap()
        .send()
        .await
        .unwrap();
    */

    // Deploy the HelloBonsai contract.
    let clob_contract = HelloBonsai::deploy(
        client.clone(),
        (dummy_bpn.address(), dummy_erc20.address(), Secret::zero()),
    )
    .expect("Failed to create CLOB deployment tx")
    .send()
    .await
    .expect("Failed to send CLOB deployment tx");
    assert_eq!(
        client
            .get_balance(clob_contract.address(), None)
            .await
            .unwrap(),
        U256::zero()
    );

    let trade_orders = vec![
        TradeOrder {
            owner_id: wallet_address.into(),
            order_type: OrderType::BUY,
            limit_price: 20,
            amount: 150,
            matched_in: 0,
            matched_out: 0,
        },
        TradeOrder {
            owner_id: wallet_address.into(),
            order_type: OrderType::SELL,
            limit_price: 20,
            amount: 120,
            matched_in: 0,
            matched_out: 0,
        },
    ];

    // Send a BUY order for 150 ERC at 20 WEI each (costs 3000 WEI)
    submit_trade_order(&clob_contract, &trade_orders[0])
        .send()
        .await
        .expect("Failed to send CLOB BUY tx");
    assert_eq!(
        client
            .get_balance(clob_contract.address(), None)
            .await
            .unwrap(),
        U256::from(3000u32)
    );

    // Send a SELL order for 120 ERC at 20 WEI each (costs 120 ERC)
    submit_trade_order(&clob_contract, &trade_orders[1])
        .send()
        .await
        .expect("Failed to send CLOB SELL tx");
    assert_eq!(
        client
            .get_balance(clob_contract.address(), None)
            .await
            .unwrap(),
        U256::from(3000u32)
    );

    // Read events for orders
    let logged_orders = read_order_logs(&clob_contract, 0).await;
    assert_eq!(logged_orders, trade_orders,);

    // Create settlements
    let validation = MatchingValidationInput {
        range: Default::default(),
        orders: trade_orders,
        accumulator: Default::default(),
    };
    let journal = validation.validate(&HashMapKeyValueStore::new()).unwrap();

    // Settle
    submit_settlements(
        &clob_contract,
        2,
        &journal.settlements,
        journal.final_state.0,
        vec![],
    )
    .send()
    .await
    .expect("Failed to send CLOB settlement tx");
    // Check balance
    assert_eq!(
        client
            .get_balance(clob_contract.address(), None)
            .await
            .unwrap(),
        U256::from(600u32)
    );
    // Check state
    assert_eq!(
        clob_contract.accumulator().call().await.unwrap(),
        journal.accumulator.0
    );
    assert_eq!(
        clob_contract.last_processed().call().await.unwrap(),
        clob_contract.last_submitted().call().await.unwrap()
    );
}
