// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";

contract FailingTestAfterFailedSetupTest is DSTest {
    function setUp() public {
        assertTrue(false);
    }

    function testAssertSuccess() public {
        assertTrue(true);
    }

    function testAssertFailure() public {
        assertTrue(false);
    }
}
