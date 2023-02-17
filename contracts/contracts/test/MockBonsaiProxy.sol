// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../IBonsaiProxy.sol";

contract MockBonsaiProxy is IBonsaiProxy {
    event SubmitRequest(bytes32 image_id, bytes calldata input, address callback_contract);

    function submit_request(
        bytes32 image_id,
        bytes calldata input,
        address callback_contract
    ) external {
        emit SubmitRequest(image_id, input, callback_contract);
    }

    function send_callback(function callback_function, bytes32 image_id, bytes calldata journal) {
        callback_function(image_id, journal);
    }
}
