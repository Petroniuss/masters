// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "../peer-set/PeerSetSmartContractAPI.sol";

interface UsingPeerBroadcastEvents {
    event PeerRegistered(address peerAddress, string peerMetadataIPFSPointer);

    event PeerSetRegistered(
        PeerSetSmartContractAPI peerSetSmartContractAddress
    );
}

// for now anyone can register as a peer!
interface PeerBroadcastAPI is UsingPeerBroadcastEvents {
    // todo: come up with a better way to do this.
    function registerPeer(string calldata peerIPFSPointer) external;

    // todo: this should check that it's an actual instance of the peer set smart contract.
    function registerPeerSet(PeerSetSmartContractAPI peerSetSmartContract)
        external;
}
