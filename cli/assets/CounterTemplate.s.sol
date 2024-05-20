// SPDX-License-Identifier: UNLICENSED
pragma solidity ^1.1.0;

import "spark-std/Script.sol";

contract CounterScript is Script {
    function setUp() public {}

    function run() public {
        vm.broadcast();
    }
}
