// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract CoinbaseTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS());

    function testCoinbase() public {
        cheats.coinbase(0xce86ea674fdde714fd979de3edf0f56aa9716b898ec8);
        assertEq(block.coinbase, 0xce86ea674fdde714fd979de3edf0f56aa9716b898ec8, "coinbase failed");
    }

    function testCoinbaseFuzzed(address who) public {
        cheats.coinbase(who);
        assertEq(block.coinbase, who, "coinbase failed");
    }
}
