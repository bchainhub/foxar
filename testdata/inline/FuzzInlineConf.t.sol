// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";

contract FuzzInlineConf is DSTest {
    /**
     * spark-config: default.fuzz.runs = 1024
     * spark-config: default.fuzz.max-test-rejects = 500
     */
    function testInlineConfFuzz(uint8 x) public {
        require(true, "this is not going to revert");
    }
}
