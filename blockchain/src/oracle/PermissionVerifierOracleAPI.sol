// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../peer-set/PeerSetSmartContractAPI.sol";

interface PermissionVerifierOracleAPI {
    event PermissionGraphValidationRequested(
        bytes32 requestId, PeerSetSmartContractAPI peerSetSmartContract, string proposedGraphIPFSPointer
    );

    event PermissionGraphChangeValidated(bytes32 requestId, bool valid);

    function validatePermissionGraphChange(string calldata proposedGraphIPFSPointer) external returns (bytes32);

    function submitPeerValidation(bytes32 requestId, bool result) external;
}
