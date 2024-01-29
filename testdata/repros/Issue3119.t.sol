// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3119
contract Issue3119Test is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    address public owner = 0xcb1958b39698a44bdae37f881e68dce073823a48a631;
    address public alice = 0xcb675ffbdbb79c60f695c9f5b9df2ec16fb8171ee13d;

    function testRollFork() public {
        uint256 fork = vm.createFork("rpcAlias");
        vm.selectFork(fork);

        FortressSwap fortressSwap = new FortressSwap(address(owner));
        vm.prank(owner);
        fortressSwap.updateOwner(alice);
    }
}

contract FortressSwap {
    address owner;

    constructor(address _owner) {
        owner = _owner;
    }

    function updateOwner(address new_owner) public {
        require(msg.sender == owner, "must be owner");
        owner = new_owner;
    }
}
