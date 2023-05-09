// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./PeerSetSmartContractAPI.sol";

contract PeerSetSmartContract is PeerSetSmartContractAPI {
    // idea:
    // see which option is better?
    // have slightly slower computation to see who's part of peerset
    // or save on storage just by saving an array without map.
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
        address changeRequester;
        string pendingCID;
        PeerSetSmartContractAPI otherPeerset;
        string otherPeersetPendingCID;
        uint256 peerVotesCount;
        uint256 positivePeerVotesCount;
        mapping(address => bool) voted;
    }

    // todo: creating a peerset should happen after peers agree to join a peerset.
    constructor(
        address[] memory _peers,
        string memory _peerSetPermissionGraphIPFSPointer
    ) {
        currentCID = _peerSetPermissionGraphIPFSPointer;

        for (uint256 i = 0; i < _peers.length; i++) {
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
        assert(!isVotingOpen());

        address peerRequestingChange = msg.sender;
        assert(isPeer(peerRequestingChange));

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
        votingRound.otherPeersetPendingCID = "";
        votingRound.otherPeerset = (PeerSetSmartContractAPI)(address(0));
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
        assert(!isVotingOpen());
        assert(isPeerOrPeerset(msg.sender, otherPeerset));

        // set transaction state
        votingRound.changeRequester = msg.sender;
        votingRound.pendingCID = thisPeersetProposedCID;
        votingRound.otherPeersetPendingCID = otherPeersetProposedCID;
        votingRound.otherPeerset = otherPeerset;
        votingRound.peerVotesCount = 0;
        votingRound.positivePeerVotesCount = 0;
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
        assert(isPeer(msg.sender));
        assert(isVotingOpen());
        assert(matchesVotingRoundCID(cid));
        // todo: remove this assertion to allow aborting hanging cross-peerset transactions.
        assert(votingRound.voted[msg.sender] == false);

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
        assert(
            isPeerset(msg.sender, votingRound.otherPeerset)
        );

        // this peerset also accepts the change
        if (votingState() == VotingState.ACCEPTED) {
            emit PeerSetPermissionGraphUpdated(
                votingRound.changeRequester, votingRound.pendingCID
            );

            currentCID = votingRound.pendingCID;
            votingRound.changeRequester = address(0);

            return true;
        }

        // not yet
        return false;
    }

    function otherPeersetRejectedChange() external {
        assert(
            isPeerset(msg.sender, votingRound.otherPeerset)
        );

        emit PeerSetPermissionGraphChangeRejected(
            votingRound.changeRequester, votingRound.pendingCID
        );

        votingRound.changeRequester = address(0);
    }

    function votingState() public view returns (VotingState) {
        uint256 rejectedVotesCount =
        votingRound.peerVotesCount - votingRound.positivePeerVotesCount;
        uint256 approvedVotesCount = votingRound.positivePeerVotesCount;
        uint256 majority = peersCount() / 2;

        if (approvedVotesCount > majority) {
            return VotingState.ACCEPTED;
        }

        if (rejectedVotesCount >= majority) {
            return VotingState.REJECTED;
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
        if (address(votingRound.otherPeerset) != address(0)) {
            return VotingType.CROSS_PEERSET_VOTING;
        } else {
            return VotingType.WITHIN_PEERSET_VOTING;
        }
    }

    function isPeer(address _peer) public view returns (bool) {
        for (uint256 i = 0; i < peersArray.length; i++) {
            if (peersArray[i] == _peer) {
                return true;
            }
        }

        return false;
    }

    function isPeerset(address sender, PeerSetSmartContractAPI peerset)
    private
    view
    returns (bool)
    {
        return sender == address(peerset) || sender == address(this);
    }

    function isPeerOrPeerset(address sender, PeerSetSmartContractAPI peerset)
    private
    view
    returns (bool)
    {
        return isPeer(sender) || isPeerset(sender, peerset);
    }

    // Question:
    // Is it really the most efficient way to compare two strings? Why?
    // Benchmarks: https://fravoll.github.io/solidity-patterns/string_equality_comparison.html
    function matchesVotingRoundCID(string calldata voteCID)
    private
    view
    returns (bool)
    {
        return keccak256(abi.encodePacked(voteCID))
        == keccak256(abi.encodePacked(votingRound.pendingCID));
    }
}
