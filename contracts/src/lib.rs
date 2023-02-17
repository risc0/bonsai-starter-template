// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

//! Rust interfaces for deploying and interfacing with HelloBonsai.
//! TODO(victor): Fill in this file

use ethers::{prelude::{abigen}};

abigen!(HelloBonsai, "artifacts/HelloBonsai.sol/HelloBonsai.json");

// TODO(victor): Write a mock for the Bonsai proxy and some tests for the fibonacci contract.
