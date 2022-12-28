// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0 <0.9.0;

contract PermissionGraph {
    string public PermissionGraphIPFSPointer;

    event PermissionGraphChangeRequest(
        string organisationName,
        string _PermissionGraphIPFSPointer
    );

    event PermissionGraphUpdated(
        string organisationName,
        string PermissionGraphIPFSPointer
    );

    // we need to have some notion of organisations.
    constructor() {
        PermissionGraphIPFSPointer = "";
    }

    // todo: we need to handle races.
    // the way this should work
    // propose a change
    // wait for validation
    // if validated then change
    // if a new graph change comes in
    // then either invalidate the old one
    function proposePermissionGraphChange(
        string calldata organisationName,
        string calldata _PermissionGraphIPFSPointer) external {
        emit PermissionGraphChangeRequest(organisationName, _PermissionGraphIPFSPointer);

        // todo: we need to validate the change via an oracle.
        // for now let's blindly accept the change.
        PermissionGraphIPFSPointer = _PermissionGraphIPFSPointer;
        emit PermissionGraphUpdated(organisationName, PermissionGraphIPFSPointer);
    }

    function getLatestPermissionGraphIPFSPointer() public view returns (string memory) {
        return PermissionGraphIPFSPointer;
    }
}
