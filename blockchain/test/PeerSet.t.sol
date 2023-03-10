// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "forge-std/Vm.sol";
import "forge-std/Base.sol";
import "forge-std/console.sol";

import "../src/peer-set/PeerSetSmartContractAPI.sol";
import "../src/peer-set/PeerSetSmartContract.sol";
import "../src/oracle/PermissionVerifierOracle.sol";
import "../src/oracle/PermissionVerifierOracleAPI.sol";
import "./UsingSharedSetup.t.sol";

contract PeerSetTest is Test, UsingDeployedPeerSetWithTwoPeersTest {
    function testSuccessfulGraphChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated
        vm.prank(ADDRESS_PEER_2);
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;
        oracle.submitPeerValidation(requestId, true);

        // then change is applied
        string memory latestGraph =
            peerSetContract.latestPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, proposedGraph);
    }

    function testPeetSetSmartContractEmittedEventAfterNewGraphWasValidated()
        public
    {
        // given a proposed change
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;
        vm.prank(ADDRESS_PEER_1);
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // then oracle event is emitted
        vm.expectEmit(true, true, true, true);
        emit PermissionGraphChangeValidated(requestId, true);

        // then peet set event is emitted
        vm.expectEmit(true, true, true, true);
        address peerRequestingChange = ADDRESS_PEER_1;
        address peerValidatingChange = ADDRESS_PEER_2;
        string memory updatedPeerSetPermissionGraphIPFSPointer = proposedGraph;
        emit PeerSetPermissionGraphUpdated(
            peerRequestingChange,
            peerValidatingChange,
            updatedPeerSetPermissionGraphIPFSPointer
            );

        // when validation is submitted
        vm.prank(ADDRESS_PEER_2);
        oracle.submitPeerValidation(requestId, true);
    }

    function testRejectedGraphChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated
        vm.prank(ADDRESS_PEER_2);
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;
        oracle.submitPeerValidation(requestId, false);

        // then change is rejected
        string memory latestGraph =
            peerSetContract.latestPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, initialGraph);
    }

    function testValidationByTheSamePeerWhoSubmittedTheChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated by the same peer
        vm.prank(ADDRESS_PEER_1);
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;
        oracle.submitPeerValidation(requestId, true);

        // then change is applied
        string memory latestGraph =
            peerSetContract.latestPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, proposedGraph);
    }

    function testValidationByInvalidPeer() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated by invalid peer
        vm.prank(ADDRESS_PEER_3);
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;

        // then validation is rejected.
        vm.expectRevert();
        oracle.submitPeerValidation(requestId, true);
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
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;

        //  validation is rejected
        vm.expectRevert();
        oracle.submitPeerValidation(requestId, true);

        //  change is validated by valid peer.
        vm.prank(ADDRESS_PEER_2);
        oracle.submitPeerValidation(requestId, true);

        // change is applied.
        string memory latestGraph =
            peerSetContract.latestPeerSetPermissionGraphIPFSPointer();
        assertEq(latestGraph, proposedGraph);
    }

    function testOracleEmittedEventAfterValidationWasRequested() public {
        // when a change is proposed
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;

        // then expected event is emitted
        vm.expectEmit(true, true, true, true);
        emit PermissionGraphValidationRequested(
            requestId, peerSetContract, proposedGraph
            );

        vm.prank(ADDRESS_PEER_1);
        peerSetContract.proposePermissionGraphChange(proposedGraph);
    }

    function testOracleEmittedEventAfterValidationWasSubmitted() public {
        // given a proposed change
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        bytes32 requestId =
            0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;
        vm.prank(ADDRESS_PEER_1);
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // then expected event is emitted
        vm.expectEmit(true, true, true, true);
        emit PermissionGraphChangeValidated(requestId, true);

        // when validation is submitted
        vm.prank(ADDRESS_PEER_2);
        oracle.submitPeerValidation(requestId, true);
    }
}
