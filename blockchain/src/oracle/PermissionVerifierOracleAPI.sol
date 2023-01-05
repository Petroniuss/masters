// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../peer-set/PeerSetSmartContractAPI.sol";

interface PermissionVerifierOracleAPI {

    function validatePermissionGraphChange(
        string calldata proposedGraphIPFSPointer
    ) external returns (bytes32);

    function validatePermissionGraphChangeResult(
        bytes32 requestId,
        bool result
    ) external;

}
