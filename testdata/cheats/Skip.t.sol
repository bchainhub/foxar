// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract SkipTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS);

    function testSkip() public {
        cheats.skip(true);
        revert("Should not reach this revert");
    }

    function testFailNotSkip() public {
        cheats.skip(false);
        revert("This test should fail");
    }

    function testFuzzSkip(uint256 x) public {
        cheats.skip(true);
        revert("Should not reach revert");
    }

    function testFailFuzzSkip(uint256 x) public {
        cheats.skip(false);
        revert("This test should fail");
    }

    function statefulFuzzSkip() public {
        cheats.skip(true);
        require(true == false, "Test should not reach invariant");
    }
}
