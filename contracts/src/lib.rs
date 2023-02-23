// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED

//! Rust interfaces for deploying and interfacing with HelloBonsai.

use ethers::prelude::abigen;

abigen!(HelloBonsai, "artifacts/HelloBonsai.sol/HelloBonsai.json");
