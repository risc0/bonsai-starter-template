// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

//! Tests for the HelloBonsai contract using a mock for the Bonsai proxy contract.
//! TODO(victor) Fill in this file.

pub mod utils;

use std::collections::HashMap;
use std::error::Error;

use ethers::prelude::*;
use hello_bonsai_contracts::HelloBonsai;
use hello_bonsai_methods::{FIBONACCI_ELF, FIBONACCI_ID};
use risc0_zkvm::sha::Digest;

use crate::utils::bonsai_test;

#[tokio::test]
pub async fn test_happy_path() -> Result<(), Box<dyn Error>> {
    let image_id = Digest::from(FIBONACCI_ID);
    let registry = HashMap::from([(image_id.into(), FIBONACCI_ELF)]);

    bonsai_test(registry, |client, bonsai_mock_address| async move {
        // Deploy the HelloBonsai contract.
        let hello_bonsai =
            HelloBonsai::deploy(client.clone(), (bonsai_mock_address, H256(image_id.into())))?
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
    })
    .await
}
