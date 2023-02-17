// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

#![no_main]
#![no_std]

use ethabi::ethereum_types::U256;
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
    // TODO(victor): Accept an ABI-encoded input string and return an ABI encoded callback.
    let n: U256 = env::read();
    env::commit(&fibonacci(n));
}
