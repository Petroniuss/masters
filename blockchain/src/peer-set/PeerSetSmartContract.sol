// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./PeerSetSmartContractAPI.sol";

contract PeerSetSmartContract is PeerSetSmartContractAPI {
    mapping(address => bool) public peers;
    address[] public peersArray;
    string public currentCID;

    VotingRound public votingRound;

    struct VotingRound {
        address peerRequestingChange;
        string pendingCID;

        uint peerVotesCount;
        uint positivePeerVotesCount;
        mapping(address => bool) voted;
    }


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
        require(!isVotingRoundOpen(), "There is already a pending request");

        address peerRequestingChange = msg.sender;
        require(isPeer(peerRequestingChange), "Caller is not a peer");

        emit PeerSetPermissionGraphChangeRequest(
            msg.sender, proposedGraphIPFSPointer
        );

        if (peersCount() == 1) {
            currentCID = proposedGraphIPFSPointer;
            emit PeerSetPermissionGraphUpdated(
                peerRequestingChange,
                proposedGraphIPFSPointer
            );
            return;
        }

        // start a voting round
        votingRound.peerRequestingChange = peerRequestingChange;
        votingRound.pendingCID = proposedGraphIPFSPointer;
        votingRound.peerVotesCount = 1;
        votingRound.positivePeerVotesCount = 1;
        for (uint256 i = 0; i < peersArray.length; i++) {
            votingRound.voted[peersArray[i]] = false;
        }
        votingRound.voted[peerRequestingChange] = true;
    }


    function submitPeerVote(string calldata cid, bool vote) external {
        require(isPeer(msg.sender), "Caller is not a peer");
        require(isVotingRoundOpen(), "There are no pending changes");
        require(matchesVotingRoundCID(cid), "Vote CID does not match pending CID");
        require(votingRound.voted[msg.sender] == false, "Peer already voted");

        emit PeerSetPermissionGraphVoteReceived(votingRound.pendingCID, vote);
        votingRound.voted[msg.sender] = true;
        votingRound.peerVotesCount++;
        if (vote) {
            votingRound.positivePeerVotesCount++;
        }

        if (votingRound.peerVotesCount > (peersCount() / 2)) {
            if (votingRound.positivePeerVotesCount > (peersCount() / 2)) {
                currentCID = votingRound.pendingCID;
                emit PeerSetPermissionGraphUpdated(
                    votingRound.peerRequestingChange,
                    votingRound.pendingCID
                );
            } else {
                emit PeerSetPermissionGraphChangeRejected(
                    votingRound.peerRequestingChange,
                    votingRound.pendingCID
                );
            }

            // reset voting round
            votingRound.peerRequestingChange = address(0);
        }
    }

    function peersCount() public view returns (uint) {
        return peersArray.length;
    }

    function isVotingRoundOpen() public view returns (bool) {
        return votingRound.peerRequestingChange != address(0);
    }

    function isPeer(address _peer) public view returns (bool) {
        return peers[_peer];
    }

    // todo: this is a hack to compare strings. find a better way
    function matchesVotingRoundCID(string calldata voteCID) public view returns (bool) {
        return keccak256(abi.encodePacked(voteCID)) == keccak256(abi.encodePacked(votingRound.pendingCID));
    }
}
