// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../IBonsaiProxy.sol";

contract MockBonsaiProxy is IBonsaiProxy {
    event SubmitRequest(bytes32 image_id, bytes input, function(bytes32, bytes memory) external callback_function);

    function submit_request(
        bytes32 image_id,
        bytes calldata input,
        address callback_address,
        bytes4 callback_selector
    ) external {
        emit SubmitRequest(image_id, input, callback_function);
    }

    function send_callback(address callback_address, bytes4 callback_selector, bytes32 image_id, bytes memory journal) external {
        callback_function(image_id, journal);
    }
}
