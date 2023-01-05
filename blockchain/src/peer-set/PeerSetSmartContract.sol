// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "./PeerSetSmartContractAPI.sol";
import "../oracle/PermissionVerifierOracleAPI.sol";

contract PeerSetSmartContract is PeerSetSmartContractAPI {
    mapping(address => bool) public peers;
    PermissionVerifierOracleAPI public oracle;
    string public peerSetPermissionGraphIPFSPointer;

    // todo: wrap this into a nice struct.
    bytes32 public pendingRequestId;
    string public pendingGraphIPFSPointer;
    address public peerRequestingChange;

    constructor(
        address[] memory _peers,
        PermissionVerifierOracleAPI _oracle,
        string memory _peerSetPermissionGraphIPFSPointer
    ) {
        oracle = _oracle;
        peerSetPermissionGraphIPFSPointer = _peerSetPermissionGraphIPFSPointer;

        for (uint256 i = 0; i < _peers.length; i++) {
            peers[_peers[i]] = true;
        }
    }

    function latestPeerSetPermissionGraphIPFSPointer()
        external
        view
        returns (string memory)
    {
        return peerSetPermissionGraphIPFSPointer;
    }

    function proposePermissionGraphChange(
        string calldata proposedGraphIPFSPointer
    ) external {
        require(pendingRequestId == 0, "There is already a pending request");

        address _peerRequestingChange = msg.sender;
        require(isPeer(_peerRequestingChange), "Caller is not a peer");

        emit PeerSetPermissionGraphChangeRequest(
            msg.sender, proposedGraphIPFSPointer
            );

        bytes32 requestId =
            oracle.validatePermissionGraphChange(proposedGraphIPFSPointer);
        pendingRequestId = requestId;
        pendingGraphIPFSPointer = proposedGraphIPFSPointer;
        peerRequestingChange = _peerRequestingChange;
    }

    function __callback(
        bytes32 requestId,
        bool result,
        address peerValidatingChange
    ) external {
        require(msg.sender == address(oracle), "Caller is not the oracle");
        require(pendingRequestId == requestId, "RequestId is not valid");

        if (result) {
            peerSetPermissionGraphIPFSPointer = pendingGraphIPFSPointer;
            emit PeerSetPermissionGraphUpdated(
                peerRequestingChange,
                peerValidatingChange,
                pendingGraphIPFSPointer
                );
        } else {
            emit PeerSetPermissionGraphChangeRejected(
                peerRequestingChange,
                peerValidatingChange,
                pendingGraphIPFSPointer
                );
        }

        pendingRequestId = 0;
        pendingGraphIPFSPointer = "";
        peerRequestingChange = address(0);
    }

    function isPeer(address _peer) public view returns (bool) {
        return peers[_peer];
    }
}
