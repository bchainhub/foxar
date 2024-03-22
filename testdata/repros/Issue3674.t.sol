// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3674
contract Issue3674Test is DSTest {
    Cheats vm = Cheats(HEVM_ADDRESS);

    function testNonceCreateSelect() public {
        vm.createSelectFork(
            "rpcAlias", 
            7627763
        );

        vm.createSelectFork("rpcAlias", 7627762);
        assertEq(vm.getNonce(msg.sender), 22871);
    }
}
