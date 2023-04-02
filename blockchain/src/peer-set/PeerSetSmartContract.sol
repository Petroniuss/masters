// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./PeerSetSmartContractAPI.sol";

contract PeerSetSmartContract is PeerSetSmartContractAPI {
    mapping(address => bool) public peers;
    address[] public peersArray;
    string public currentCID;

    VotingRound private votingRound;

    enum VotingType {
        WITHIN_PEERSET_VOTING,
        CROSS_PEERSET_VOTING
    }
    enum VotingState {
        ACCEPTED,
        REJECTED,
        IN_PROGRESS
    }

    struct VotingRound {
        // either a peer from this peerset or a different peerset
        address changeRequester;
        string pendingCID;
        // cross-peerset change
        PeerSetSmartContractAPI otherPeerset;
        string otherPeersetPendingCID;
        uint256 peerVotesCount;
        uint256 positivePeerVotesCount;
        mapping(address => bool) voted;
    }

    // On change proposition:
    // 1. Verify that there are no open voting rounds in both peersets,
    // 2. Set up a state for counting votes (just like in a single peerset case)
    // 3. Send events to each peerset (separately)

    // 1. On each vote check if both smart contracts have reached an agreement,
    // 2. If any of them disapproved of a change - change is rejected in both peersets
    // 3. If both agreed - a change is accepted in both.
    // 4. Peerset should have a way to abort transaction if the other peerset takes too long
    //      (peer could simply vote to reject a given transaction, even after approving)

    // todo: creating a peerset should happen after peers agree to join a peerset.
    constructor(
        address[] memory _peers,
        string memory _peerSetPermissionGraphIPFSPointer
    ) {
        currentCID = _peerSetPermissionGraphIPFSPointer;

        for (uint256 i = 0; i < _peers.length; i++) {
            peers[_peers[i]] = true;
            peersArray.push(_peers[i]);
        }
    }

    function currentPeerSetPermissionGraphIPFSPointer()
        external
        view
        returns (string memory)
    {
        return currentCID;
    }

    function proposePermissionGraphChange(
        string calldata proposedGraphIPFSPointer
    ) external {
        require(!isVotingOpen(), "There is already a pending request");

        address peerRequestingChange = msg.sender;
        require(isPeer(peerRequestingChange), "Caller is not a peer");

        emit PeerSetPermissionGraphChangeRequest(
            msg.sender, proposedGraphIPFSPointer
        );

        if (peersCount() == 1) {
            currentCID = proposedGraphIPFSPointer;
            emit PeerSetPermissionGraphUpdated(
                peerRequestingChange, proposedGraphIPFSPointer
            );
            return;
        }

        // start a voting round
        votingRound.changeRequester = peerRequestingChange;
        votingRound.pendingCID = proposedGraphIPFSPointer;
        votingRound.peerVotesCount = 1;
        votingRound.positivePeerVotesCount = 1;
        for (uint256 i = 0; i < peersArray.length; i++) {
            votingRound.voted[peersArray[i]] = false;
        }
        votingRound.voted[peerRequestingChange] = true;
    }

    function proposeCrossPeersetChange(
        string calldata thisPeersetProposedCID,
        string calldata otherPeersetProposedCID,
        PeerSetSmartContractAPI otherPeerset
    ) external {
        require(!isVotingOpen(), "There is already a pending request");
        require(
            isPeerOrPeerset(msg.sender, otherPeerset),
            "Caller must either be a peerset smart contract or a peer"
        );

        // set transaction state
        votingRound.changeRequester = msg.sender;
        votingRound.pendingCID = thisPeersetProposedCID;
        votingRound.otherPeersetPendingCID = otherPeersetProposedCID;
        for (uint256 i = 0; i < peersArray.length; i++) {
            votingRound.voted[peersArray[i]] = false;
        }

        // if a peer from this peerset triggered this transaction count its vote.
        if (isPeer(msg.sender)) {
            votingRound.peerVotesCount = 1;
            votingRound.positivePeerVotesCount = 1;
            votingRound.voted[msg.sender] = true;
        }

        // notify smart contract listeners
        emit CrossPeersetGraphChangeRequest(
            votingRound.changeRequester,
            votingRound.pendingCID,
            votingRound.otherPeerset,
            votingRound.otherPeersetPendingCID
        );

        // start transaction in the other peerset.
        if (isPeer(msg.sender)) {
            otherPeerset.proposeCrossPeersetChange(
                otherPeersetProposedCID, thisPeersetProposedCID, this
            );
        }
    }

    function submitPeerVote(string calldata cid, bool vote) external {
        require(isPeer(msg.sender), "Caller is not a peer");
        require(isVotingOpen(), "There are no pending changes");
        require(
            matchesVotingRoundCID(cid), "Vote CID does not match pending CID"
        );
        // todo: remove this assertion to allow aborting hanging cross-peerset transactions.
        require(votingRound.voted[msg.sender] == false, "Peer already voted");

        emit PeerSetPermissionGraphVoteReceived(votingRound.pendingCID, vote);
        votingRound.voted[msg.sender] = true;
        votingRound.peerVotesCount++;
        if (vote) {
            votingRound.positivePeerVotesCount++;
        }

        VotingState state = votingState();
        if (state != VotingState.IN_PROGRESS) {
            VotingType vType = votingType();

            if (vType == VotingType.WITHIN_PEERSET_VOTING) {
                if (state == VotingState.ACCEPTED) {
                    currentCID = votingRound.pendingCID;
                    emit PeerSetPermissionGraphUpdated(
                        votingRound.changeRequester, votingRound.pendingCID
                    );
                } else if (state == VotingState.REJECTED) {
                    emit PeerSetPermissionGraphChangeRejected(
                        votingRound.changeRequester, votingRound.pendingCID
                    );
                }

                // reset voting round
                votingRound.changeRequester = address(0);
            } else if (vType == VotingType.CROSS_PEERSET_VOTING) {
                if (state == VotingState.ACCEPTED) {
                    bool accepted =
                        votingRound.otherPeerset.otherPeersetAcceptedChange();
                    if (accepted) {
                        this.otherPeersetAcceptedChange();
                    }
                } else if (state == VotingState.REJECTED) {
                    votingRound.otherPeerset.otherPeersetRejectedChange();
                    this.otherPeersetRejectedChange();
                }
            }
        }
    }

    // called when the other peerset has accepted the transaction
    function otherPeersetAcceptedChange() external returns (bool) {
        require(
            isPeerset(msg.sender, votingRound.otherPeerset),
            "Caller is not a peerset"
        );

        // this peerset also accepts the change
        if (votingState() == VotingState.ACCEPTED) {
            currentCID = votingRound.pendingCID;
            votingRound.changeRequester = address(0);

            emit PeerSetPermissionGraphUpdated(
                votingRound.changeRequester, votingRound.pendingCID
            );

            return true;
        }

        // not yet
        return false;
    }

    function otherPeersetRejectedChange() external {
        require(
            isPeerset(msg.sender, votingRound.otherPeerset),
            "Caller is not a peerset"
        );

        votingRound.changeRequester = address(0);
        emit PeerSetPermissionGraphChangeRejected(
            votingRound.changeRequester, votingRound.pendingCID
        );
    }

    function votingState() public view returns (VotingState) {
        if (votingRound.peerVotesCount > (peersCount() / 2)) {
            if (votingRound.positivePeerVotesCount > (peersCount() / 2)) {
                return VotingState.ACCEPTED;
            } else {
                return VotingState.REJECTED;
            }
        }

        return VotingState.IN_PROGRESS;
    }

    function peersCount() public view returns (uint256) {
        return peersArray.length;
    }

    function isVotingOpen() public view returns (bool) {
        return votingRound.changeRequester != address(0);
    }

    function votingType() private view returns (VotingType) {
        if (votingRound.changeRequester == address(votingRound.otherPeerset)) {
            return VotingType.CROSS_PEERSET_VOTING;
        } else {
            return VotingType.WITHIN_PEERSET_VOTING;
        }
    }

    function isPeer(address _peer) public view returns (bool) {
        return peers[_peer];
    }

    function isPeerset(address sender, PeerSetSmartContractAPI peerset)
        private
        pure
        returns (bool)
    {
        return sender == address(peerset);
    }

    function isPeerOrPeerset(address sender, PeerSetSmartContractAPI peerset)
        private
        view
        returns (bool)
    {
        return isPeer(sender) || isPeerset(sender, peerset);
    }

    function matchesVotingRoundCID(string calldata voteCID)
        private
        view
        returns (bool)
    {
        return keccak256(abi.encodePacked(voteCID))
            == keccak256(abi.encodePacked(votingRound.pendingCID));
    }
}
