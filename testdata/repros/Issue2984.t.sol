// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/2984
contract Issue2984Test is DSTest {
    Cheats vm = Cheats(HEVM_ADDRESS);
    uint256 fork;
    uint256 snapshot;

    function setUp() public {
        fork = vm.createSelectFork(
            "rpcAlias",
            7627763
        );
        snapshot = vm.snapshot();
    }

    function testForkRevertSnapshot() public {
        vm.revertTo(snapshot);
    }

    function testForkSelectSnapshot() public {
        uint256 fork2 = vm.createSelectFork(
            "rpcAlias",
            7627765
        );
    }
}
