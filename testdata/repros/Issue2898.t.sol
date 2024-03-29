// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";
import "../logs/console.sol";
import {Checksum} from "ds-test/checksum.sol";

// https://github.com/foxar-rs/foxar/issues/2898
contract Issue2898Test is DSTest {
    address private BRIDGE = Checksum.toIcan(uint160(bytes20(hex"0000000000000000000000000000000000000010")));
    address private BENEFICIARY = Checksum.toIcan(uint160(bytes20(hex"0000000000000000000000000000000000000011")));
    Cheats vm = Cheats(HEVM_ADDRESS());

    function setUp() public {
        vm.deal(BRIDGE, 100);
        vm.deal(BENEFICIARY, 99);

        vm.setNonce(BRIDGE, 10);
    }

    function testDealBalance() public {
        assertEq(BRIDGE.balance, 100);
        assertEq(BENEFICIARY.balance, 99);
    }

    function testSetNonce() public {
        assertEq(vm.getNonce(BRIDGE), 10);
    }
}
