// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;

use ethabi::ethereum_types::U256;
use ethabi::{ParamType, Token};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn fibonacci(n: U256) -> U256 {
    let (mut prev, mut curr) = (U256::one(), U256::one());
    for _ in 2..=n.as_u32() {
        (prev, curr) = (curr, prev + curr);
    }
    return curr;
}

pub fn main() {
    // Decode input pass in from the Bonsai bridge.
    let input = ethabi::decode_whole(&[ParamType::Uint(256)], &env::read::<Vec<u8>>()).unwrap();
    let n: U256 = input[0].clone().into_uint().unwrap();

    // Run the computation.
    let result = fibonacci(n);

    // Commit the journal that will be decoded in the application contract.
    env::commit_slice(&ethabi::encode(&[Token::Uint(n), Token::Uint(result)]));
}
