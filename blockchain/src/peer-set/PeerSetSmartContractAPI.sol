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

    event CrossPeersetGraphChangeRequest(
        address peerRequestingChange,
        string thisPeersetProposedCID,
        PeerSetSmartContractAPI otherPeerset,
        string otherPeersetProposedCID
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

    // cross-peerset
    function proposeCrossPeersetChange(
        string calldata thisPeersetCID,
        string calldata otherPeersetCID,
        PeerSetSmartContractAPI otherPeerset
    ) external;

    // called when the other peerset verified that a change has been verified
    // on his end and asks the other peerset to commit transaction if he also has verified the change.
    function otherPeersetAcceptedChange() external returns (bool);

    // if a peerset decides to abort/reject the transaction
    function otherPeersetRejectedChange() external;
}
