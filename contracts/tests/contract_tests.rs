// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

//! Tests for the HelloBonsai contract using a mock for the Bonsai proxy contract.
//! TODO(victor) Fill in this file.

use std::collections::HashMap;
use std::error::Error;
use std::ops::Deref;
use std::sync::Arc;
use std::future::Future;

use ethers::core::k256::ecdsa::SigningKey;
use ethers::prelude::*;
use ethers::utils::{Ganache, GanacheInstance};
use hello_bonsai_contracts::HelloBonsai;
use hello_bonsai_methods::{FIBONACCI_ELF, FIBONACCI_ID};
use risc0_zkvm::sha::Digest;
use risc0_zkvm::{serde, Prover, ProverOpts};
use tokio::task::{JoinError, JoinHandle};
use tokio::sync::oneshot;

abigen!(
    MockBonsaiProxy,
    "artifacts/MockBonsaiProxy.sol/MockBonsaiProxy.json"
);

pub type Client = Arc<SignerMiddleware<Provider<Ws>, Wallet<SigningKey>>>;

pub async fn get_ganache_client() -> Result<(GanacheInstance, Client), Box<dyn Error>> {
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

pub struct BonsaiMock {
    pub contract_address: Address,
    pub task_handle: JoinHandle<()>,
}

impl BonsaiMock {
    pub async fn spawn(
        client: Client,
        registry: HashMap<[u8; 32], &'static [u8]>,
    ) -> Result<Self, Box<dyn Error>> {
        let (tx, rx) = oneshot::channel();

        let task_handle: JoinHandle<()> = tokio::spawn(async move {
            // Deploy the MockBonsaiProxy
            let mock_bonsai_proxy = MockBonsaiProxy::deploy(client.clone(), ()).unwrap().send().await.unwrap();

            let events = mock_bonsai_proxy.events();
            let mut subscription = events.subscribe().await.unwrap();
            tx.send(mock_bonsai_proxy.address()).unwrap();

            while let Some(event) = subscription.next().await {
                let submit_request_log =
                    event.expect("error in getting next event from subscription");
                let receipt = {
                    let elf = registry.get(&submit_request_log.image_id).expect(&format!(
                        "image ID not found in registry: {:x?}",
                        submit_request_log.image_id
                    ));
                    let mut prover = Prover::new_with_opts(
                        elf.as_ref(),
                        submit_request_log.image_id,
                        ProverOpts::default().with_skip_seal(true),
                    )
                    .expect("failed to create prover");
                    prover.add_input_u32_slice(
                        &serde::to_vec(submit_request_log.input.deref()).unwrap(),
                    );
                    prover.run().expect("failed to run guest")
                };

                mock_bonsai_proxy
                    .send_callback(
                        submit_request_log.callback_address,
                        submit_request_log.callback_selector,
                        submit_request_log.image_id.into(),
                        receipt.journal.into(),
                    )
                    .send()
                    .await
                    .expect("failed to send callback");
            }
            panic!("unexpected end of event stream");
        });

        // Wait for the contract to be deployed, then return the mock.
        Ok(Self { contract_address: rx.await.unwrap(), task_handle })
    }
}

pub async fn bonsai_test<F>(registry: HashMap<[u8; 32], &'static [u8]>, test: impl FnOnce(Client, Address) -> F) -> Result<(), Box<dyn Error>>
    where F: Future<Output = Result<(), Box<dyn Error>>>,
{
    // Instantiate client as wallet on network
    let (_ganache, client) = get_ganache_client().await?;

    // Deploy the Bonsai mock contract and start a background worker.
    let mut bonsai_mock = BonsaiMock::spawn(
        client.clone(),
        registry,
    )
    .await?;

    tokio::select! {
        result = test(client.clone(), bonsai_mock.contract_address) => result,
        error = &mut bonsai_mock.task_handle => Err(format!("Bonsai mock died with result: {:?}", error).into()),
    }
}

#[tokio::test]
pub async fn test_happy_path() -> Result<(), Box<dyn Error>> {
    let image_id = Digest::from(FIBONACCI_ID);
    let registry = HashMap::from([(image_id.into(), FIBONACCI_ELF)]);

    bonsai_test(registry, |client, bonsai_mock_address| async move {
        // Deploy the HelloBonsai contract.
        let hello_bonsai = HelloBonsai::deploy(
            client.clone(),
            (bonsai_mock_address, H256(image_id.into())),
        )?
        .send()
        .await?;

        let events = hello_bonsai.events();
        let mut subscription = events.subscribe().await?;

        hello_bonsai
            .calculate_fibonacci(U256::from(10))
            .send()
            .await?;

        let callback_log = subscription.next().await.unwrap()?;
        assert_eq!(callback_log.n, U256::from(10));
        assert_eq!(callback_log.result, U256::from(89));

        // Check that the journal is used to produce the expected state change.
        let result: U256 = hello_bonsai.fibonacci(U256::from(10)).call().await?;

        assert_eq!(result, U256::from(89));
        Ok(())
    }).await
}
