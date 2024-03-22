// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3223
contract Issue3223Test is DSTest {
    Cheats vm = Cheats(HEVM_ADDRESS);
    uint256 fork1;
    uint256 fork2;

    function setUp() public {
        fork1 = vm.createFork(
            "rpcAlias",
            3813881
        );
        fork2 = vm.createFork(
            "rpcAlias",
            7627763
        );
    }

    function testForkNonce() public {
        address user = address(0xcb1958b39698a44bdae37f881e68dce073823a48a631);
        assertEq(user, msg.sender);

        vm.selectFork(fork2);
        assertEq(vm.getNonce(user), 22872);
        vm.prank(user);
        new Counter();

        vm.selectFork(fork1);
        assertEq(vm.getNonce(user), 0);
        vm.prank(user);
        new Counter();
    }
}

contract Counter {}
