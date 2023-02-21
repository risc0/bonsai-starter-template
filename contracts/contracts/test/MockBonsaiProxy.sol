// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "../IBonsaiProxy.sol";

contract MockBonsaiProxy is IBonsaiProxy {
    event SubmitRequest(bytes32 image_id, bytes input, address callback_address, bytes4 callback_selector);

    function submit_request(
        bytes32 image_id,
        bytes calldata input,
        address callback_address,
        bytes4 callback_selector
    ) external {
        emit SubmitRequest(image_id, input, callback_address, callback_selector);
    }

    function send_callback(address callback_address, bytes4 callback_selector, bytes32 image_id, bytes calldata journal) external {
        (bool success, bytes memory _data) = callback_address.call(abi.encodeWithSelector(callback_selector, image_id, journal));
        require(success, "Bonsai callback reverted");
    }
}
