// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

interface UsingPeerSetEvents {
    event PeerSetPermissionGraphChangeRequest(
        address peerRequestingChange,
        string proposedPeerSetPermissionGraphIPFSPointer
    );

    event PeerSetPermissionGraphUpdated(
        address peerRequestingChange,
        string updatedPeerSetPermissionGraphIPFSPointer
    );

    event PeerSetPermissionGraphChangeRejected(
        address peerRequestingChange,
        string rejectedPeerSetPermissionGraphIPFSPointer
    );

    event PeerSetPermissionGraphVoteReceived(string cid, bool vote);
}

interface PeerSetSmartContractAPI is UsingPeerSetEvents {
    function proposePermissionGraphChange(
        string calldata proposedGraphIPFSPointer
    ) external;

    function submitPeerVote(string calldata cid, bool vote) external;

    function currentPeerSetPermissionGraphIPFSPointer()
        external
        view
        returns (string memory);

    function isPeer(address peer) external view returns (bool);
}
