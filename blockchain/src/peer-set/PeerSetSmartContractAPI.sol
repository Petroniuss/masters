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

    // cross-peerset
    // on change proposition:
    // - verify that there are no open voting rounds in both peersets,
    // - set up a state for counting votes,
    // - send events from each peerset
    //
    // on each vote:
    // - check if both peersets have reached an agreement:
    //  - if any peerset disapproved of a change - change is rejected in both peersets,
    //  - if both agreed - a change is accepted in both,
    //  - otherwise wait for more votes.
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
