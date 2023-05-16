// SPDX-License-Identifier: MIT
pragma solidity ^0.8.17;

contract Counter {
    int public count = 0;

    function increment() public {
        count += 1;
    }

    function getCount() public view returns (int) {
        return count;
    }
}