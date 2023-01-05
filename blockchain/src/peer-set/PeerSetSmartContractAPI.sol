// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

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

    function proposePermissionGraphChange(
        string calldata proposedGraphIPFSPointer) external;

    function latestPeerSetPermissionGraphIPFSPointer()
    external view returns (string memory);

    function isPeer(address peer) external view returns (bool);

    function __callback(
        bytes32 requestId,
        bool result,
        address peerValidatingChange
    ) external;
}
