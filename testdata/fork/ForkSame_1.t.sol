// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

contract ForkTest is DSTest {
    address constant WETH_TOKEN_ADDR = 0xcb19c7acc4c292d2943ba23c2eaa5d9c5a6652a8710c;
    Cheats constant cheats = Cheats(HEVM_ADDRESS);
    uint256 forkA;

    // this will create two _different_ forks during setup
    function setUp() public {
        forkA = cheats.createFork("https://xcbapi-arch-mainnet.coreblockchain.net/", 7_582_801);
    }

    function testDummy() public {
        uint256 balance = WETH_TOKEN_ADDR.balance;
    }
}
