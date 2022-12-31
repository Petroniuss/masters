// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;

import "forge-std/Test.sol";
import "../src/PermissionGraph.sol";

contract PermissionGraphTest is Test {
    function testInitialState() public {
        PermissionGraph permissionGraphContract = new PermissionGraph();

        string memory expected = "";
        assertEq(permissionGraphContract.getLatestPermissionGraphIPFSPointer(), expected);
    }

    function testProposingChange() public {
        PermissionGraph permissionGraphContract = new PermissionGraph();

        string memory organisationName = "ORG_1";
        string memory newPermissionGraphIPFSPointer = "QmZ1";
        permissionGraphContract.proposePermissionGraphChange(organisationName, newPermissionGraphIPFSPointer);

        string memory actual = permissionGraphContract.getLatestPermissionGraphIPFSPointer();
        assertEq(actual, newPermissionGraphIPFSPointer);
    }

    // todo - test the event emission - could be done using javascript tests.
}
