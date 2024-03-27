// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract RememberTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS);

    function testRememberKey() public {
        string memory mnemonic = "test test test test test test test test test test test junk";

        uint256 privateKey = cheats.deriveKey(mnemonic, 0);
        assertEq(privateKey, 0x000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001);

        address thisAddress = cheats.rememberKey(privateKey);
        assertEq(thisAddress, 0xce49e5dd06163a480c22d540ec763325a0b5860fb56c);
    }
}
