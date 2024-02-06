// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/2629
contract Issue2629Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    function testSelectFork() public {
        address coinbase = 0xcb580851379288d0c6b251af9f49988ccca9d6502948;

        uint256 f1 = vm.createSelectFork("rpcAlias", 7628371);
        vm.selectFork(f1);

        assertEq(block.number, 7628371);
        assertEq(coinbase.balance, 58668399788681000075776);

        uint256 f2 = vm.createFork("rpcAlias", 7628372);
        vm.selectFork(f2);

        assertEq(block.number, 7628372);
        assertEq(coinbase.balance, 58423096881681000079872);
    }
}
