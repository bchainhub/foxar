// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";

contract ContractEnvironmentTest is DSTest {
    function chainId() internal view returns (uint256 id) {
        assembly {
            id := chainid()
        }
    }

    function testAddresses() public {
        assertEq(msg.sender, 0xce591804c8ab1f12e6bbf3894d4083f33e07309d1f38, "sender account is incorrect");
        assertEq(tx.origin, 0xce591804c8ab1f12e6bbf3894d4083f33e07309d1f38, "origin account is incorrect");
        assertEq(address(this), 0xce37791ef0be909181dd92373777e9ea118e63c1f68a, "test contract address is incorrect");
    }

    function testEnvironment() public {
        assertEq(chainId(), 1337, "chainid is incorrect");
        assertEq(block.number, 1, "block number is incorrect");
        assertEq(blockhash(block.number), 0x0, "blockhash is incorrect");
        assertEq(block.coinbase, address(0xce450000000000000000000000000000000000000000), "coinbase is incorrect");
        assertEq(block.timestamp, 1, "timestamp is incorrect");
    }
}
