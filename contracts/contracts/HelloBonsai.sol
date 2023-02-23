// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pragma solidity ^0.8.17;

import "./IBonsaiProxy.sol";

/// @title A starter application using Bonsai through the on-chain proxy.
/// @dev This contract demonstrates one pattern for offloading the computation of an expensive
//       or difficult to implement function to a RISC Zero guest running on Bonsai.
contract HelloBonsai {
    // Address of the Bonsai proxy contract.
    IBonsaiProxy public immutable bonsai_proxy;
    // Image ID of the associated RISC Zero guest program.
    bytes32 public immutable image_id;

    // Cache of the results calculated by our guest program in Bonsai.
    mapping(uint256 => uint256) public fibonnaci_cache;

    event CalculateFibonacciCallback(uint256 indexed n, uint256 result);

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
        bonsai_proxy.submit_request(image_id, abi.encode(n), address(this), this.calculate_fibonacci_callback.selector);
    }

    /// @notice Callback function to be called by the Bonsai proxy when the result is ready.
    function calculate_fibonacci_callback(
        bytes32 _image_id,
        bytes calldata journal
    ) external {
        // Require that caller is the trusted proxy contract and guest program.
        // After security checks, the output is verified to come from the guest. Decode the journal.
        require(msg.sender == address(bonsai_proxy));
        require(_image_id == image_id);
        (uint256 n, uint256 result) = abi.decode(journal, (uint256, uint256));

        emit CalculateFibonacciCallback(n, result);
        fibonnaci_cache[n] = result;
    }
}
