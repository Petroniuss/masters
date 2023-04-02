// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "forge-std/Vm.sol";
import "forge-std/Base.sol";
import "forge-std/console.sol";

import "../src/peer-set/PeerSetSmartContractAPI.sol";
import "../src/peer-set/PeerSetSmartContract.sol";
import "./UsingSharedSetup.t.sol";

contract PeerSetTest is Test, UsingDeployedPeerSetWithTwoPeersTest {
    function testSuccessfulGraphChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated
        vm.prank(ADDRESS_PEER_2);
        peerSetContract.submitPeerVote(proposedGraph, true);

        // then change is applied
        string memory latestGraph =
            peerSetContract.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, proposedGraph);
    }

    function testPeetSetSmartContractEmittedEventsAfterNewGraphWasValidated()
        public
    {
        // given a proposed change
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        address peerRequestingChange = ADDRESS_PEER_1;
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphChangeRequest(
            peerRequestingChange, proposedGraph
        );

        vm.prank(ADDRESS_PEER_1);
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // then peerset vote event is emitted
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphVoteReceived(proposedGraph, true);

        // then peerset updated event is emitted
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphUpdated(peerRequestingChange, proposedGraph);

        // when validation is submitted
        vm.prank(ADDRESS_PEER_2);
        peerSetContract.submitPeerVote(proposedGraph, true);
    }

    function testRejectedGraphChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated
        vm.prank(ADDRESS_PEER_2);
        peerSetContract.submitPeerVote(proposedGraph, false);

        // then change is rejected
        string memory latestGraph =
            peerSetContract.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, initialGraph);
    }

    function testValidationByTheSamePeerWhoSubmittedTheChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // then vote is rejected
        vm.expectRevert();

        // when change is validated by the same peer
        vm.prank(ADDRESS_PEER_1);
        peerSetContract.submitPeerVote(proposedGraph, true);
    }

    function testValidationByInvalidPeer() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated by invalid peer
        vm.prank(ADDRESS_PEER_3);

        // then validation is rejected.
        vm.expectRevert();
        peerSetContract.submitPeerVote(proposedGraph, true);
    }

    function testSuccessfulValidationAfterInvalidValidationWasRejected()
        public
    {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated by invalid peer
        vm.prank(ADDRESS_PEER_3);

        //  validation is rejected
        vm.expectRevert();
        peerSetContract.submitPeerVote(proposedGraph, true);

        //  change is validated by valid peer.
        vm.prank(ADDRESS_PEER_2);
        peerSetContract.submitPeerVote(proposedGraph, true);

        // change is applied.
        string memory latestGraph =
            peerSetContract.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, proposedGraph);
    }
}
