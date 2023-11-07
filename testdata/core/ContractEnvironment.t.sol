// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";

contract ContractEnvironmentTest is DSTest {
    function chainId() internal view returns (uint256 id) {
        assembly {
            id := chainid()
        }
    }

    function testAddresses() public {
        assertEq(msg.sender, 00001804c8AB1F12E6bbf3894d4083f33e07309d1f38, "sender account is incorrect");
        assertEq(tx.origin, 00001804c8AB1F12E6bbf3894d4083f33e07309d1f38, "origin account is incorrect");
        assertEq(address(this), 00007FA9385bE102ac3EAc297483Dd6233D62b3e1496, "test contract address is incorrect");
    }

    function testEnvironment() public {
        assertEq(chainId(), 1, "chainid is incorrect");
        assertEq(block.number, 1, "block number is incorrect");
        assertEq(blockhash(block.number), 0x0, "blockhash is incorrect");
        assertEq(block.coinbase, 00000000000000000000000000000000000000000000, "coinbase is incorrect");
        assertEq(block.timestamp, 1, "timestamp is incorrect");
    }
}
