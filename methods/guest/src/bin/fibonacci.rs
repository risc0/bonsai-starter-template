// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

#![no_main]
#![no_std]

use ethabi::Uint;
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

fn fibonacci(n: Uint) -> Uint {
    let (mut prev, mut curr) = (Uint::one(), Uint::one());
    for _ in 2..=n.as_u32() {
        (prev, curr) = (curr, prev + curr);
    }
    return curr;
}

pub fn main() {
    // TODO(victor): Accept an ABI-encoded input string and return an ABI encoded callback.
    let n: Uint = env::read();
    env::commit(&fibonacci(n));
}
