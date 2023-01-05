// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../peer-set/PeerSetSmartContractAPI.sol";

// We could also define events in a library
// But this might bring its own set of problems:
// https://web.archive.org/web/20180922101404/https://blog.aragon.org/library-driven-development-in-solidity-2bebcaf88736/
interface UsingPermissionVerifierOracleEvents {
    event PermissionGraphValidationRequested(
        bytes32 requestId,
        PeerSetSmartContractAPI peerSetSmartContract,
        string proposedGraphIPFSPointer
    );

    event PermissionGraphChangeValidated(bytes32 requestId, bool valid);
}

interface PermissionVerifierOracleAPI is UsingPermissionVerifierOracleEvents {
    function validatePermissionGraphChange(
        string calldata proposedGraphIPFSPointer
    ) external returns (bytes32);

    function submitPeerValidation(bytes32 requestId, bool result) external;
}
