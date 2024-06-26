// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract LabelTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS());

    function testLabel() public {
        cheats.label(address(1), "Sir Address the 1st");
    }
}
