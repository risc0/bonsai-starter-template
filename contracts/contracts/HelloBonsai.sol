// Copyright 2023 RISC Zero, Inc.
// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./IBonsaiProxy.sol";

/// @title A starter application using Bonsai through the on-chain proxy.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
//       or difficult to implement function onto Bonsai.
contract HelloBonsai {
    // Address of the Bonsai proxy contract on the current chain.
    IBonsaiProxy public immutable bonsai_proxy;
    // Image ID of the associated RISC Zero guest program.
    bytes32 public immutable image_id;

    // Cache of the results calculated by our guest program in Bonsai.
    mapping(uint256 => uint256) public fibonnaci_cache;

    // Initialize the contract, binding it to a specified Bonsai proxy and RISC Zero image.
    constructor(IBonsaiProxy _bonsai_proxy, bytes32 _image_id) {
        bonsai_proxy = _bonsai_proxy;
        image_id = _image_id;
    }

    /// @notice Returns nth number in the Fibonacci sequence.
    /// @dev The sequence is defined as 1, 1, 2, 3, 5 ... with fibonnacci(0) == 1.
    ///      Only precomputed results can be returned. Call calculate_fibonacci(n) to precompute.
    function fibonacci(uint256 n) external view returns (uint256) {
        uint256 result = fibonnaci_cache[n];
        require(result != 0);
        return result;
    }

    /// @notice Sends a request to Bonsai to have have the nth Fibonacci number calculated.
    /// @dev This function sends the request to Bonsai through the on-chain proxy. The request will
    ///      trigger the Bonsai network to run the specified RISC Zero guest image with the given
    ///      input and asynchonrously return the verified results to use via the callback below.
    function calculate_fibonacci(uint256 n) external {
        bonsai_proxy.submit_request(image_id, abi.encode(n), this.calculate_fibonacci_callback);
    }

    /// @notice Callback function to be called by the Bonsai proxy when the result is ready.
    function calculate_fibonacci_callback(
        bytes32 _image_id,
        bytes calldata journal
    ) external {
        require(msg.sender == address(bonsai_proxy));
        require(_image_id == image_id);
        uint256 n;
        uint256 result;
        (n, result) = abi.decode(journal, (uint256, uint256));
        fibonnaci_cache[n] = result;
    }
}
