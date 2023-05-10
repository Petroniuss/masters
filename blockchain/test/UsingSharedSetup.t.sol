// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "forge-std/Base.sol";
import "../src/peer-set/PeerSetSmartContractAPI.sol";
import "../src/peer-set/PeerSetSmartContract.sol";

abstract contract UsingSharedAddressesTest {
    address constant ADDRESS_PEER_1 = 0xd13C4379BfC9a0EA5E147B2D37F65eB2400DFD7B;
    address constant ADDRESS_PEER_2 = 0xd248e4A8407ed7fF9bdBc396ba46723B8101C86e;
    address constant ADDRESS_PEER_3 = 0x2EFdD9aac437AC8d6CAC7cAFa3887b08769Dc049;
    address constant ADDRESS_PEER_4 = 0x797Be246A8d1858716F4A269db20DE021Dc7b321;
    address constant ADDRESS_PEER_5 = 0xC644e5B3A55ddFCe8a84cd0556303fA0Be23a6B8;
    address constant ADDRESS_PEER_6 = 0xd767583CB2bB4FFE8931d7B423B89F653a9F4D54;
    address constant ADDRESS_PEER_7 = 0x3b7Ef1EcD33B5a700C988c4b3f577d8d24126260;
    address constant ADDRESS_PEER_8 = 0x0e21E148871CbD109A8C645d336139A6a4d5ca86;
    address constant ADDRESS_PEER_9 = 0x17335596284Fe3522878c193F85b61b8d5cF1507;
    address constant ADDRESS_PEER_10 = 0x3213CFD7d0d6a00CA192b5157D1Cf54143bC0e19;
}

abstract contract UsingDeployedPeerSetWithTwoPeersTest is
    UsingSharedAddressesTest,
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
            peers, initialGraph
        );
    }

    function setUp() public {
        vm.label(ADDRESS_PEER_1, "VALID_PEER_1");
        vm.label(ADDRESS_PEER_2, "VALID_PEER_2");
        vm.label(ADDRESS_PEER_3, "INVALID_PEER_3");
        vm.label(ADDRESS_PEER_4, "INVALID_PEER_4");
    }
}

abstract contract UsingDeployedPeerSetWithForPeersTest is
    UsingSharedAddressesTest,
    UsingPeerSetEvents,
    CommonBase
{
    PeerSetSmartContractAPI peerSetContract;
    address[] peers;
    string initialGraph =
    "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu";

    constructor() {
        peers = new address[](4);
        peers[0] = ADDRESS_PEER_1;
        peers[1] = ADDRESS_PEER_2;
        peers[2] = ADDRESS_PEER_3;
        peers[3] = ADDRESS_PEER_4;

        peerSetContract = new PeerSetSmartContract(
            peers, initialGraph
        );
    }

    function setUp() public {
        vm.label(ADDRESS_PEER_1, "VALID_PEER_1");
        vm.label(ADDRESS_PEER_2, "VALID_PEER_2");
        vm.label(ADDRESS_PEER_3, "VALID_PEER_3");
        vm.label(ADDRESS_PEER_4, "VALID_PEER_4");
    }
}

abstract contract UsingDeployedPeerSetWithTenPeersTest is
UsingSharedAddressesTest,
UsingPeerSetEvents,
CommonBase
{
    PeerSetSmartContractAPI peerSetContract;
    address[] peers;
    string initialGraph =
    "https://ipfs.io/ipfs/Qme7ss3ARVgxv6rXqVPiikMJ8u2NLgmgszg13pYrDKEoiu";

    constructor() {
        peers = new address[](10);
        peers[0] = ADDRESS_PEER_1;
        peers[1] = ADDRESS_PEER_2;
        peers[2] = ADDRESS_PEER_3;
        peers[3] = ADDRESS_PEER_4;
        peers[4] = ADDRESS_PEER_5;
        peers[5] = ADDRESS_PEER_6;
        peers[6] = ADDRESS_PEER_7;
        peers[7] = ADDRESS_PEER_8;
        peers[8] = ADDRESS_PEER_9;
        peers[9] = ADDRESS_PEER_10;

        peerSetContract = new PeerSetSmartContract(
            peers, initialGraph
        );
    }

    function setUp() public {
        vm.label(ADDRESS_PEER_1, "VALID_PEER_1");
        vm.label(ADDRESS_PEER_2, "VALID_PEER_2");
        vm.label(ADDRESS_PEER_3, "VALID_PEER_3");
        vm.label(ADDRESS_PEER_4, "VALID_PEER_4");
        vm.label(ADDRESS_PEER_5, "VALID_PEER_5");
        vm.label(ADDRESS_PEER_6, "VALID_PEER_6");
        vm.label(ADDRESS_PEER_7, "VALID_PEER_7");
        vm.label(ADDRESS_PEER_8, "VALID_PEER_8");
        vm.label(ADDRESS_PEER_9, "VALID_PEER_9");
        vm.label(ADDRESS_PEER_10, "VALID_PEER_10");
    }
}

abstract contract UsingTwoPeersetsWithTwoPeersTest is
    UsingSharedAddressesTest,
    UsingPeerSetEvents,
    CommonBase
{
    PeerSetSmartContractAPI peerset1;
    address[] peers1;
    string initialCIDPeerset1 = "https://ipfs.io/p1-cid-1";

    PeerSetSmartContractAPI peerset2;
    address[] peers2;
    string initialCIDPeerset2 = "https://ipfs.io/p2-cid-1";

    constructor() {
        peers1 = new address[](2);
        peers1[0] = ADDRESS_PEER_1;
        peers1[1] = ADDRESS_PEER_2;
        peerset1 = new PeerSetSmartContract(
            peers1, initialCIDPeerset1
        );

        peers2 = new address[](2);
        peers2[0] = ADDRESS_PEER_3;
        peers2[1] = ADDRESS_PEER_4;
        peerset2 = new PeerSetSmartContract(
            peers2, initialCIDPeerset2
        );
    }

    function setUp() public {
        vm.label(ADDRESS_PEER_1, "PEERSET_1_PEER_1");
        vm.label(ADDRESS_PEER_2, "PEERSET_1_PEER_2");
        vm.label(ADDRESS_PEER_3, "PEERSET_2_PEER_1");
        vm.label(ADDRESS_PEER_4, "PEERSET_2_PEER_2");
    }
}
