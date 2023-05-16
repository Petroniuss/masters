// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

import "forge-std/Test.sol";
import "forge-std/Vm.sol";
import "forge-std/Base.sol";
import "../src/counter/Counter.sol";

contract CounterTest is Test {
    function testIncrement() public {
        Counter counter = new Counter();
        counter.increment();
        assertEq(counter.getCount(), 1);
    }
}