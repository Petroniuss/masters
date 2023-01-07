// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.17;

import "./UsingSharedSetup.t.sol";
import "forge-std/Test.sol";

contract PeerBroadcastTest is Test, UsingDeployedPeerBroadcastContractTest {
    function testRegisterPeerEmitsPeerRegisteredEvent() public {
        // given
        string memory peer1Meta = "https://ipfs.io/ipfs/QmZ1";

        // then expected event is emitted
        vm.expectEmit(false, false, false, true);
        emit PeerRegistered(ADDRESS_PEER_1, peer1Meta);

        // when a peer registers
        // meh passing calldata is really annoying :/
        vm.prank(ADDRESS_PEER_1);

        (bool _success, bytes memory _data) = address(peerBroadCastContract)
            .call(abi.encodeCall(peerBroadCastContract.registerPeer, peer1Meta));

        assertTrue(_success);
    }

    function testRegisterPeerSetEmitsPeerSetRegisteredEvent() public {
        //given
        PeerSetSmartContractAPI peerSetSmartContract =
            PeerSetSmartContractAPI(address(1));

        // then expected event is emitted
        vm.expectEmit(false, false, false, true);
        emit PeerSetRegistered(peerSetSmartContract);

        // when peerset is registered
        (bool _success, bytes memory _data) = address(peerBroadCastContract)
            .call(
            abi.encodeCall(
                peerBroadCastContract.registerPeerSet, peerSetSmartContract
            )
        );

        assertTrue(_success);
    }
}
