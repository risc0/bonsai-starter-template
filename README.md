# Bonsai Starter Template

Starter template for writing an application using [Bonsai].

This repository implements an application on Ethereum utilizing the Bonsai network as a coprocessor to the smart contract application.
It provides a starting point for building powerful new applications on Ethereum that offload computationally intensive, or difficult to implement tasks to a [RISC Zero] guest, with verified results sent to your Ethereum contract.

## Getting Started

Start building your application by forking this template.

### Dependencies

* Rust and Cargo: https://rustup.rs
* Ganache: https://www.npmjs.com/package/ganache#command-line-use

### Write Your App

Get started writing your application by modifying these key files:
* Replace `contracts/contracts/HelloBonsai.sol` with your on-chain application logic.
* Replace `methods/guest/src/bin/fibonacci.rs` with your Bonsai coprocessor logic.

Associated build configuration files and tests are discussed along with the [project structure](#project-structure) below.

### Build

Running the following will build the project, including Ethereum contracts and RISC Zero guest program.

```bash
cargo build
```

### Test

Running the following will run all tests, including Ethereum contracts and RISC Zero guest program.

```bash
cargo test
```

## Project Structure

```text
.
├── Cargo.toml
├── README.md
├── cli
│   ├── Cargo.toml
│   └── src
│       └── bin
│           ├── deploy.rs
│           └── poke.rs
├── contracts
│   ├── Cargo.toml
│   ├── build.rs
│   ├── contracts
│   │   ├── HelloBonsai.sol
│   │   ├── IBonsaiProxy.sol
│   │   └── test
│   │       └── MockBonsaiProxy.sol
│   ├── src
│   │   └── lib.rs
│   └── tests
│       └── contract_tests.rs
└── methods
    ├── Cargo.toml
    ├── build.rs
    ├── guest
    │   ├── Cargo.toml
    │   └── src
    │       └── bin
    │           └── fibonacci.rs
    └── src
        └── lib.rs
```

### Contracts

Ethereum contracts are in the `contracts` directory.
Solidity can be found in `contracts/contracts` including the application source and an interface and mock implementation of the Bonsai proxy.

Contracts are built and tested with [Cargo] with the Rust [ethers] libraries.
The build configuration is in `contracts/build.rs`.
Tests are written in Rust and run against Ganache.
Tests are defined in `contracts/tests`.

### Methods

[RISC Zero] guest programs are defined in the `methods` directory.

### CLI

A starter CLI is provided to help deploy contracts to Ethereum and RISC Zero guest programs to Bonsai.
Additionally the CLI includes example commands to interact with the smart contract via ethers.

[Bonsai]: https://example.com
[RISC Zero]: https://www.risczero.com/
[ethers]: https://docs.rs/ethers/latest/ethers/
[Cargo]: https://doc.rust-lang.org/cargo/
<!--
TODO
* Use links to public Bonsai materials.
* Ensure Docker images gets open-sourced.
* Include a docker-compose.yml file to run Bonsai local.
* Finish a set of contract functional tests that integrate with the guest and a Mock proxy.
* Get the Bonsai contracts open-sourced and import IBonsaiProxy from them.
* Folks need to install ganache via `npm install -g ganache` to run tests.
* Build a cli that can:
    * Deploy the contract to Ethereum and ELF to Bonsai.
    * Poke the HelloBonsai contract to prove its working.
* Ensure that any NPM dependencies (e.g. ganache) are managed in a sane way.
* Add solhint configurations.
-->
