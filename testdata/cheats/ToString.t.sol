// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract ToStringTest is DSTest {
    Cheats constant cheats = Cheats(HEVM_ADDRESS);

    function testAddressToString() public {
        address testAddress = 0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8;
        string memory stringAddress = cheats.toString(testAddress);
        assertEq("0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8", stringAddress);
    }

    function testBytes32ToString() public {
        bytes32 testBytes = "test";
        string memory stringBytes = cheats.toString(testBytes);
        assertEq("0x7465737400000000000000000000000000000000000000000000000000000000", stringBytes);
    }

    function testBoolToString() public {
        bool testBool = true;
        string memory stringBool = cheats.toString(testBool);
        assertEq("true", stringBool);
    }

    function testBytesToString() public {
        bytes memory testBytes = hex"7109709ECfa91a80626fF3989D68f67F5b1DD12D";
        string memory stringBytes = cheats.toString(testBytes);
        assertEq("0x7109709ecfa91a80626ff3989d68f67f5b1dd12d", stringBytes);
    }

    function testUintToString() public {
        uint256 testUint = 420;
        string memory stringUint = cheats.toString(testUint);
        assertEq("420", stringUint);
    }

    function testIntToString() public {
        int256 testInt = 420;
        string memory stringInt = cheats.toString(testInt);
        assertEq("420", stringInt);
    }
}
