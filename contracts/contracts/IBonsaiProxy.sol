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
//
// SPDX-License-Identifier: Apache-2.0

pragma solidity ^0.8.17;

interface IBonsaiProxy {
    /// @notice Submit a verifiable computation request to the Bonsai network.
    /// @dev Bonsai will run RISC Zero guest program specified by image ID with the specified bytes
    ///      as input. The resulting journal will be returned asynchronously via a callback.
    /// @param image_id Identifier for the RISC Zero guest program that should be run.
    ///         The associated ELF binary should be uploaded to the Bonsai network.
    /// @param input Data to be passed to the guest, accessible by calling `env::read`.
    /// @param callback_address Contract address where the execution result callback is requested.
    /// @param callback_selector 4-byte function selector for the callback.
    function submit_request(
        bytes32 image_id,
        bytes calldata input,
        address callback_address,
        bytes4 callback_selector
    ) external;
}
