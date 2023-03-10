// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../peer-set/PeerSetSmartContractAPI.sol";
import "./PermissionVerifierOracleAPI.sol";

// The simplest possible implementation that I could come up with.
// Once again we could have a single contract that handles all the
// permission graph changes, or we could have one per PeerSet.
contract PermissionVerifierOracle is PermissionVerifierOracleAPI {
    mapping(bytes32 => PeerSetSmartContractAPI) public requests;

    constructor() { }

    function validatePermissionGraphChange(
        string calldata proposedGraphIPFSPointer
    ) external returns (bytes32) {
        PeerSetSmartContractAPI caller = PeerSetSmartContractAPI(msg.sender);
        bytes32 requestId =
            keccak256(abi.encodePacked(proposedGraphIPFSPointer));

        emit PermissionGraphValidationRequested(
            requestId, caller, proposedGraphIPFSPointer
            );

        requests[requestId] = caller;

        return requestId;
    }

    function submitPeerValidation(bytes32 requestId, bool result) external {
        PeerSetSmartContractAPI peerSetSmartContract = requests[requestId];
        require(
            address(peerSetSmartContract) != address(0),
            "RequestId is not valid"
        );

        address peerValidatingChange = msg.sender;
        require(
            peerSetSmartContract.isPeer(peerValidatingChange),
            "only a peer can validate permission graph change"
        );

        emit PermissionGraphChangeValidated(requestId, result);

        peerSetSmartContract.__callback(requestId, result, peerValidatingChange);

        delete requests[requestId];
    }
}
