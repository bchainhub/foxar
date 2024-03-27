// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3221
contract Issue3221Test is DSTest {
    Cheats vm = Cheats(HEVM_ADDRESS);
    uint256 fork1;
    uint256 fork2;

    function setUp() public {
        fork1 = vm.createFork(
            "rpcAlias",
            7306987
        );
        fork2 = vm.createFork(
            "rpcAlias",
            7654413
        );
    }

    function testForkNonce() public {
        address user = address(0xcb1958b39698a44bdae37f881e68dce073823a48a631);

        // Loads but doesn't touch
        assertEq(vm.getNonce(user), 0);

        vm.selectFork(fork2);
        assertEq(vm.getNonce(user), 23940);
        vm.prank(user);
        new Counter();

        vm.selectFork(fork1);
        assertEq(vm.getNonce(user), 10090);
        vm.prank(user);
        new Counter();
    }
}

contract Counter {}
