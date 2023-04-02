// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "forge-std/Vm.sol";
import "forge-std/Base.sol";
import "forge-std/console.sol";

import "../src/peer-set/PeerSetSmartContractAPI.sol";
import "../src/peer-set/PeerSetSmartContract.sol";
import "./UsingSharedSetup.t.sol";

contract CrossPeersetTest is Test, UsingTwoPeersetsWithTwoPeersTest {
    function testCrossPeersetChangeSuccessfulVoting() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraphPeerset1 = "https://ipfs.io/p1-cid-2";
        string memory proposedGraphPeerset2 = "https://ipfs.io/p2-cid-2";
        peerset1.proposeCrossPeersetChange(
            proposedGraphPeerset1, proposedGraphPeerset2, peerset2
        );

        // when peer 2 accepts
        vm.prank(ADDRESS_PEER_2);
        peerset1.submitPeerVote(proposedGraphPeerset1, true);

        // then transaction is not yet committed
        string memory latestGraph =
            peerset1.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, initialCIDPeerset1);

        // only when peers from peerset2 accept
        vm.prank(ADDRESS_PEER_3);
        peerset2.submitPeerVote(proposedGraphPeerset2, true);

        vm.prank(ADDRESS_PEER_4);
        peerset2.submitPeerVote(proposedGraphPeerset2, true);

        // then change is committed in both peersets
        string memory latestGraphPeerset1 =
            peerset1.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraphPeerset1, proposedGraphPeerset1);

        string memory latestGraphPeerset2 =
            peerset2.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraphPeerset2, proposedGraphPeerset2);
    }

    function testEmittedEventsDuringSuccessfulChange() public {
        // when change is proposed
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraphPeerset1 = "https://ipfs.io/p1-cid-2";
        string memory proposedGraphPeerset2 = "https://ipfs.io/p2-cid-2";

        vm.expectEmit(true, true, true, true);
        // then event from peerset1 is emitted
        emit CrossPeersetGraphChangeRequest(
            ADDRESS_PEER_1,
            proposedGraphPeerset1,
            peerset2,
            proposedGraphPeerset2
        );

        // then event from peerset2 is emitted
        emit CrossPeersetGraphChangeRequest(
            address(peerset1),
            proposedGraphPeerset2,
            peerset1,
            proposedGraphPeerset1
        );

        peerset1.proposeCrossPeersetChange(
            proposedGraphPeerset1, proposedGraphPeerset2, peerset2
        );

        // when vote from peer 2 is submitted

        // then the vote is received
        vm.prank(ADDRESS_PEER_2);
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphVoteReceived(proposedGraphPeerset1, true);

        peerset1.submitPeerVote(proposedGraphPeerset1, true);

        // when vote is submitted to peerset2

        // then the vote is received
        vm.prank(ADDRESS_PEER_3);
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphVoteReceived(proposedGraphPeerset2, true);

        peerset2.submitPeerVote(proposedGraphPeerset2, true);

        // when final vote is submitted
        vm.prank(ADDRESS_PEER_4);
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphVoteReceived(proposedGraphPeerset2, true);

        // then events about update are emitted from both peersets
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphUpdated(
            ADDRESS_PEER_1, proposedGraphPeerset1
        );

        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphUpdated(
            address(peerset1), proposedGraphPeerset2
        );

        peerset2.submitPeerVote(proposedGraphPeerset2, true);
    }

    function testRejectedCrossPeersetChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraphPeerset1 = "https://ipfs.io/p1-cid-2";
        string memory proposedGraphPeerset2 = "https://ipfs.io/p2-cid-2";
        peerset1.proposeCrossPeersetChange(
            proposedGraphPeerset1, proposedGraphPeerset2, peerset2
        );

        // when peerset2 rejects the change
        vm.prank(ADDRESS_PEER_3);
        peerset2.submitPeerVote(proposedGraphPeerset2, false);

        // then transaction is rejected
        string memory latestGraphPeerset1 =
            peerset1.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraphPeerset1, initialCIDPeerset1);

        string memory latestGraphPeerset2 =
            peerset2.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraphPeerset2, initialCIDPeerset2);
    }

    function testEventsEmittedDuringRejectedCrossPeersetTransaction() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraphPeerset1 = "https://ipfs.io/p1-cid-2";
        string memory proposedGraphPeerset2 = "https://ipfs.io/p2-cid-2";

        vm.expectEmit(true, true, true, true);
        // then event from peerset1 is emitted
        emit CrossPeersetGraphChangeRequest(
            ADDRESS_PEER_1,
            proposedGraphPeerset1,
            peerset2,
            proposedGraphPeerset2
        );

        // then event from peerset2 is emitted
        emit CrossPeersetGraphChangeRequest(
            address(peerset1),
            proposedGraphPeerset2,
            peerset1,
            proposedGraphPeerset1
        );

        peerset1.proposeCrossPeersetChange(
            proposedGraphPeerset1, proposedGraphPeerset2, peerset2
        );

        // when peerset2 rejects the change

        // then events about update are emitted from both peersets
        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphChangeRejected(
            ADDRESS_PEER_1, proposedGraphPeerset1
        );

        vm.expectEmit(true, true, true, true);
        emit PeerSetPermissionGraphChangeRejected(
            address(peerset1), proposedGraphPeerset2
        );

        vm.prank(ADDRESS_PEER_3);
        peerset2.submitPeerVote(proposedGraphPeerset2, false);
    }

    function testRevertingTransactionWhenCrossPeersetTransactionIsInProgress()
        public
    {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraphPeerset1 = "https://ipfs.io/p1-cid-2";
        string memory proposedGraphPeerset2 = "https://ipfs.io/p2-cid-2";
        peerset1.proposeCrossPeersetChange(
            proposedGraphPeerset1, proposedGraphPeerset2, peerset2
        );

        // when a change is proposed to peerset2

        // then new proposition is rejected
        vm.expectRevert();
        string memory anotherChange = "https://ipfs.io/p2-cid-2a";
        peerset2.proposePermissionGraphChange(anotherChange);
    }

    function testProposingChangeAfterCrossPeersetTransactionIsFinished()
        public
    {
        // given a proposed and accepted change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraphPeerset1 = "https://ipfs.io/p1-cid-2";
        string memory proposedGraphPeerset2 = "https://ipfs.io/p2-cid-2";
        peerset1.proposeCrossPeersetChange(
            proposedGraphPeerset1, proposedGraphPeerset2, peerset2
        );

        vm.prank(ADDRESS_PEER_2);
        peerset1.submitPeerVote(proposedGraphPeerset1, true);

        vm.prank(ADDRESS_PEER_3);
        peerset2.submitPeerVote(proposedGraphPeerset2, true);

        vm.prank(ADDRESS_PEER_4);
        peerset2.submitPeerVote(proposedGraphPeerset2, true);

        // then it is possible to propose another change
        vm.prank(ADDRESS_PEER_1);
        string memory nextChange = "https://ipfs.io/p1-cid-3";
        peerset1.proposePermissionGraphChange(nextChange);

        vm.prank(ADDRESS_PEER_2);
        peerset1.submitPeerVote(nextChange, true);

        // another change is approved
        string memory latestGraphPeerset1 =
            peerset1.currentPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraphPeerset1, nextChange);
    }
}
