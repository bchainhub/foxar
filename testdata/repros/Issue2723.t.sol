// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/2723
contract Issue2723Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function testRollFork() public {
        address coinbase = 0xcb580851379288d0c6b251af9f49988ccca9d6502948;

        vm.createSelectFork("rpcAlias", 7628371);

        assertEq(block.number, 7628371);
        assertEq(coinbase.balance, 58668399788681000075776);

        vm.rollFork(7628372);

        assertEq(block.number, 7628372);
        assertEq(coinbase.balance, 58423096881681000079872);
    }
}
