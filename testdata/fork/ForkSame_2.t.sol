// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

contract ForkTest is DSTest {
    address constant WETH_TOKEN_ADDR = 0xcb37c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2;
    Cheats constant cheats = Cheats(HEVM_ADDRESS);
    uint256 forkA;

    // this will create two _different_ forks during setup
    function setUp() public {
        forkA = cheats.createFork("https://eth-mainnet.alchemyapi.io/v2/Lc7oIGYeL_QvInzI0Wiu_pOZZDEKBrdf", 15_977_624);
    }

    function testDummy() public {
        uint256 balance = WETH_TOKEN_ADDR.balance;
    }
}
