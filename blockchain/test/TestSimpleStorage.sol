// SPDX-License-Identifier: MIT
pragma solidity >=0.8.0 <0.9.0;

import "truffle/Assert.sol";
import "truffle/DeployedAddresses.sol";
import "../contracts/SimpleStorage.sol";

contract TestSimpleStorage {
    function testSimpleStorageInitialState() public {
        SimpleStorage simpleStorage = SimpleStorage(DeployedAddresses.SimpleStorage());

        uint expected = 0;
        Assert.equal(simpleStorage.get(), expected, "Initially the balance should be 0");
    }

    function testSetGetSequence() public {
        SimpleStorage simpleStorage = SimpleStorage(DeployedAddresses.SimpleStorage());

        simpleStorage.set(1);
        Assert.equal(simpleStorage.get(), 1,
            "Initial value should be overwritten by set operation");
    }
}
