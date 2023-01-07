// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../peer-set/PeerSetSmartContractAPI.sol";
import "./PeerBroadcastAPI.sol";

// this is used as a main entry point for the system.
// based on this contract we can get all the other contracts.
// for now anyone can register as a peer!
contract PeerBroadcast is UsingPeerBroadcastEvents, PeerBroadcastAPI {
    // todo: come up with a better way to do this.
    function registerPeer(string calldata peerIPFSPointer) external {
        emit PeerRegistered(msg.sender, peerIPFSPointer);
    }

    // todo :this should check that it's an actual instance of the peer set smart contract.
    function registerPeerSet(PeerSetSmartContractAPI peerSetSmartContract)
        external
    {
        emit PeerSetRegistered(peerSetSmartContract);
    }
}
