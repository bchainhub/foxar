// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract AddrTest is DSTest {
    Cheats constant cheats = Cheats(HEVM_ADDRESS);

    function testFailPrivKeyZero() public {
        cheats.addr(0);
    }

    function testAddr() public {
        uint256 pk = 77814517325470205911140941194401928579557062014761831930645393041380819009408;
        address expected = 0xcb58e5dd06163a480c22d540ec763325a0b5860fb56c;

        assertEq(cheats.addr(pk), expected, "expected address did not match");
    }
}
