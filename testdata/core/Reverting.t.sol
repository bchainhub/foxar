// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

contract RevertingTest {
    function testFailRevert() public pure {
        require(false, "should revert here");
    }
}
