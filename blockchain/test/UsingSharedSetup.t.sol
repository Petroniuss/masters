// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Base.sol";
import "../src/peer-set/PeerSetSmartContractAPI.sol";
import "../src/oracle/PermissionVerifierOracleAPI.sol";
import "../src/oracle/PermissionVerifierOracle.sol";
import "../src/peer-set/PeerSetSmartContract.sol";
import "../src/peer-broadcast/PeerBroadcastAPI.sol";
import "../src/peer-broadcast/PeerBroadcast.sol";

abstract contract UsingSharedAddressesTest {
    address constant ADDRESS_PEER_1 = 0xd13C4379BfC9a0EA5E147B2D37F65eB2400DFD7B;
    address constant ADDRESS_PEER_2 = 0xd248e4A8407ed7fF9bdBc396ba46723B8101C86e;
    address constant ADDRESS_PEER_3 = 0x2EFdD9aac437AC8d6CAC7cAFa3887b08769Dc049;
    address constant ADDRESS_PEER_4 = 0x797Be246A8d1858716F4A269db20DE021Dc7b321;

    PermissionVerifierOracleAPI oracle = new PermissionVerifierOracle();
}

abstract contract UsingDeployedPeerSetWithTwoPeersTest is
    UsingSharedAddressesTest,
    UsingPermissionVerifierOracleEvents,
    UsingPeerSetEvents,
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

abstract contract UsingDeployedPeerBroadcastContractTest is
    UsingSharedAddressesTest,
    UsingPeerBroadcastEvents,
    CommonBase
{
    PeerBroadcastAPI peerBroadCastContract;

    constructor() {
        peerBroadCastContract = new PeerBroadcast();
    }
}
