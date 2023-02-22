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
    // TODO(victor): This is uglier than it needs to be because env::read_slice does not support
    // reading the whole input and I do not want to demonstrate code that only works for fixed
    // sized values. An alternative would be to read a length, then read that number of bytes, this
    // will depend on what the real watcher wants to do.
    let input = ethabi::decode_whole(&[ParamType::Uint(256)], &env::read::<Vec<u8>>()).unwrap();
    let n: U256 = input[0].clone().into_uint().unwrap();
    env::commit_slice(&ethabi::encode(&[
        Token::Uint(n),
        Token::Uint(fibonacci(n)),
    ]));
}
