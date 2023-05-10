// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "forge-std/Vm.sol";
import "forge-std/Base.sol";
import "forge-std/console.sol";

import "../src/peer-set/PeerSetSmartContractAPI.sol";
import "../src/peer-set/PeerSetSmartContract.sol";
import "./UsingSharedSetup.t.sol";

contract PeerSetBenchmarkTest is Test, UsingDeployedPeerSetWithTenPeersTest {
    function testBenchmarkSuccessfulGraphChange() public {
        // given proposed change
        vm.prank(ADDRESS_PEER_1);
        string memory proposedGraph = "https://ipfs.io/ipfs/QmZ1";
        peerSetContract.proposePermissionGraphChange(proposedGraph);

        // when change is validated
        vm.prank(ADDRESS_PEER_10);
        peerSetContract.submitPeerVote(proposedGraph, true);

//        // when change is validated
//        vm.prank(ADDRESS_PEER_3);
//        peerSetContract.submitPeerVote(proposedGraph, true);

        // then change is applied
//        string memory latestGraph =
//            peerSetContract.currentPeerSetPermissionGraphIPFSPointer();
//        assertEq(latestGraph, proposedGraph);
    }
}

