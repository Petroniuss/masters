// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

interface PeerSetSmartContractAPI {

    event PeerSetPermissionGraphChangeRequest(
        address peerRequestingChange,
        string proposedPeerSetPermissionGraphIPFSPointer
    );

    event PeerSetPermissionGraphUpdated(
        address peerRequestingChange,
        address peerValidatingChange,
        string updatedPeerSetPermissionGraphIPFSPointer

    );
    event PeerSetPermissionGraphChangeRejected(
        address peerRequestingChange,
        address peerValidatingChange,
        string rejectedPeerSetPermissionGraphIPFSPointer
    );

    function isPeer(address peer) external view returns (bool);

    function __callback(
        bytes32 requestId,
        bool result,
        address peerValidatingChange
    ) external;

    function proposePermissionGraphChange(
        string calldata proposedGraphIPFSPointer
    ) external;
}
