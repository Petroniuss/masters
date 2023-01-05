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

import "../src/oracle/PermissionVerifierOracleAPI.sol";

abstract contract UsingSharedAddresses {
    address constant ADDRESS_PEER_1 = 0xd13C4379BfC9a0EA5E147B2D37F65eB2400DFD7B;
    address constant ADDRESS_PEER_2 = 0xd248e4A8407ed7fF9bdBc396ba46723B8101C86e;
    address constant ADDRESS_PEER_3 = 0x2EFdD9aac437AC8d6CAC7cAFa3887b08769Dc049;
    address constant ADDRESS_PEER_4 = 0x797Be246A8d1858716F4A269db20DE021Dc7b321;

    PermissionVerifierOracleAPI oracle = new PermissionVerifierOracle();
}

abstract contract UsingDeployedPeerSetWithTwoPeers is
    UsingSharedAddresses,
    CommonBase
{
    PeerSetSmartContractAPI peerSetContract;
    address[] peers;
    string initialGraph =
        "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu";

    constructor() {
        peers = new address[](2);
        peers[0] = ADDRESS_PEER_1;
        peers[1] = ADDRESS_PEER_2;

        peerSetContract = new PeerSetSmartContract(
            peers, oracle, initialGraph
        );
    }

    function setUp() public {
        vm.label(ADDRESS_PEER_1, "VALID_PEER_1");
        vm.label(ADDRESS_PEER_2, "VALID_PEER_2");
        vm.label(ADDRESS_PEER_3, "INVALID_PEER_3");
        vm.label(ADDRESS_PEER_4, "INVALID_PEER_4");
    }
}

contract PeerSet is Test, UsingDeployedPeerSetWithTwoPeers {
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

    // todo: test emitted events
    //    function testOracleEmittedEventAfterValidationWasRequested() public {
    //        // given proposed change
    //        vm.prank(ADDRESS_PEER_1);
    //        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
    //        peerSetContract.proposePermissionGraphChange(proposedGraph);
    //
    //        bytes32 requestId = 0x71686b49e3b73fb9bda0c3dac95d5fcaa75bbd99663c397be8393c8c5513c067;
    //
    //        vm.expectEmit(true, true, false, true);
    ////        emit PermissionVerifierOracleAPI.PermissionGraphValidationRequested(
    ////            requestId, peerSetContract, proposedGraph
    ////        );
    //    }
}
