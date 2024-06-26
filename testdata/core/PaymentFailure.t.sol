// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

contract Payable {
    function pay() public payable {}
}

contract PaymentFailureTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS());

    function testCantPay() public {
        Payable target = new Payable();
        cheats.prank(address(1));
        target.pay{value: 1}();
    }
}
