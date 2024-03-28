// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract ChainIdTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS());

    function testChainId() public {
        cheats.chainId(10);
        assertEq(block.chainid, 10, "chainId switch failed");
    }

    function testChainIdFuzzed(uint64 chainId) public {
        cheats.chainId(chainId);
        assertEq(block.chainid, chainId, "chainId switch failed");
    }
}
