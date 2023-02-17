// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

interface IBonsaiProxy {
    /// @notice Submit a verifiable computation request to the Bonsai network.
    /// @dev Bonsai will run RISC Zero guest program specified by image ID with the specified bytes
    //       as input. The resulting journal will be returned asynchronously via a callback.
    function submit_request(
        bytes32 image_id,
        bytes calldata input,
        address callback_address,
        bytes4 callback_selector
    ) external;
}
