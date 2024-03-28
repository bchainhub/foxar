// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "./console.sol";

contract HardhatLogsTest {
    constructor() {
        console.log("constructor");
    }

    string testStr;
    int256 testInt;
    uint256 testUint;
    bool testBool;
    address testAddr;
    bytes testBytes;
    bytes1 testBytes1;

    function setUp() public {
        testStr = "test";
        testInt = -31337;
        testUint = 1;
        testBool = false;
        testAddr = address(0xce180000000000000000000000000000000000000001);
        testBytes = "a";
        testBytes1 = "a";
    }

    function testInts() public view {
        console.log(0);
        console.log(1);
        console.log(2);
        console.log(3);
    }

    function testStrings() public view {
        console.log("testStrings");
    }

    function testMisc() public view {
        console.log("testMisc", address(1));
        console.log("testMisc", 42);
    }

    function testConsoleLog() public view {
        console.log(testStr);
    }

    function testLogInt() public view {
        console.logInt(testInt);
    }

    function testLogUint() public view {
        console.logUint(testUint);
    }

    function testLogString() public view {
        console.logString(testStr);
    }

    function testLogBool() public view {
        console.logBool(testBool);
    }

    function testLogAddress() public view {
        console.logAddress(testAddr);
    }

    function testLogBytes() public view {
        console.logBytes(testBytes);
    }

    function testLogBytes1() public view {
        console.logBytes1(bytes1(testBytes1));
    }

    function testLogBytes2() public view {
        console.logBytes2(bytes2(testBytes1));
    }

    function testLogBytes3() public view {
        console.logBytes3(bytes3(testBytes1));
    }

    function testLogBytes4() public view {
        console.logBytes4(bytes4(testBytes1));
    }

    function testLogBytes5() public view {
        console.logBytes5(bytes5(testBytes1));
    }

    function testLogBytes6() public view {
        console.logBytes6(bytes6(testBytes1));
    }

    function testLogBytes7() public view {
        console.logBytes7(bytes7(testBytes1));
    }

    function testLogBytes8() public view {
        console.logBytes8(bytes8(testBytes1));
    }

    function testLogBytes9() public view {
        console.logBytes9(bytes9(testBytes1));
    }

    function testLogBytes10() public view {
        console.logBytes10(bytes10(testBytes1));
    }

    function testLogBytes11() public view {
        console.logBytes11(bytes11(testBytes1));
    }

    function testLogBytes12() public view {
        console.logBytes12(bytes12(testBytes1));
    }

    function testLogBytes13() public view {
        console.logBytes13(bytes13(testBytes1));
    }

    function testLogBytes14() public view {
        console.logBytes14(bytes14(testBytes1));
    }

    function testLogBytes15() public view {
        console.logBytes15(bytes15(testBytes1));
    }

    function testLogBytes16() public view {
        console.logBytes16(bytes16(testBytes1));
    }

    function testLogBytes17() public view {
        console.logBytes17(bytes17(testBytes1));
    }

    function testLogBytes18() public view {
        console.logBytes18(bytes18(testBytes1));
    }

    function testLogBytes19() public view {
        console.logBytes19(bytes19(testBytes1));
    }

    function testLogBytes20() public view {
        console.logBytes20(bytes20(testBytes1));
    }

    function testLogBytes21() public view {
        console.logBytes21(bytes21(testBytes1));
    }

    function testLogBytes22() public view {
        console.logBytes22(bytes22(testBytes1));
    }

    function testLogBytes23() public view {
        console.logBytes23(bytes23(testBytes1));
    }

    function testLogBytes24() public view {
        console.logBytes24(bytes24(testBytes1));
    }

    function testLogBytes25() public view {
        console.logBytes25(bytes25(testBytes1));
    }

    function testLogBytes26() public view {
        console.logBytes26(bytes26(testBytes1));
    }

    function testLogBytes27() public view {
        console.logBytes27(bytes27(testBytes1));
    }

    function testLogBytes28() public view {
        console.logBytes28(bytes28(testBytes1));
    }

    function testLogBytes29() public view {
        console.logBytes29(bytes29(testBytes1));
    }

    function testLogBytes30() public view {
        console.logBytes30(bytes30(testBytes1));
    }

    function testLogBytes31() public view {
        console.logBytes31(bytes31(testBytes1));
    }

    function testLogBytes32() public view {
        console.logBytes32(bytes32(testBytes1));
    }

    function testConsoleLogUint() public view {
        console.log(testUint);
    }

    function testConsoleLogString() public view {
        console.log(testStr);
    }

    function testConsoleLogBool() public view {
        console.log(testBool);
    }

    function testConsoleLogAddress() public view {
        console.log(testAddr);
    }

    function testConsoleLogFormatString() public view {
        console.log("formatted log str=%s", testStr);
    }

    function testConsoleLogFormatUint() public view {
        console.log("formatted log uint=%s", testUint);
    }

    function testConsoleLogFormatAddress() public view {
        console.log("formatted log addr=%s", testAddr);
    }

    function testConsoleLogFormatMulti() public view {
        console.log("formatted log str=%s uint=%d", testStr, testUint);
    }

    function testConsoleLogFormatEscape() public view {
        console.log("formatted log %% %s", testStr);
    }

    function testConsoleLogFormatSpill() public view {
        console.log("formatted log %s", testStr, testUint);
    }
}