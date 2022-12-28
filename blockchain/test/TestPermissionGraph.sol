// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0 <0.9.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/PermissionGraph.sol";

contract TestPermissionGraph {
    function testInitialState() public {
        PermissionGraph permissionGraphContract = PermissionGraph(DeployedAddresses.PermissionGraph());

        string memory expected = "";
        Assert.equal(permissionGraphContract.getLatestPermissionGraphIPFSPointer(), expected,
                     "Initially the pointer is empty/null");
    }

    function testProposingChange() public {
        PermissionGraph permissionGraphContract = PermissionGraph(DeployedAddresses.PermissionGraph());

        string memory organisationName = "ORG_1";
        string memory newPermissionGraphIPFSPointer = "QmZ1";
        permissionGraphContract.proposePermissionGraphChange(organisationName, newPermissionGraphIPFSPointer);

        string memory actual = permissionGraphContract.getLatestPermissionGraphIPFSPointer();
        Assert.equal(
            actual, newPermissionGraphIPFSPointer,
            "The new pointer should be set");
    }

    // todo - test the event emission - could be done using javascript tests.
}
