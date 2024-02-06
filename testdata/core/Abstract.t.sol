// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

contract TestFixture {
    function something() public pure returns (string memory) {
        return "something";
    }
}

abstract contract AbstractTestBase {
    TestFixture fixture;

    function testSomething() public {
        fixture.something();
    }
}

contract AbstractTest is AbstractTestBase {
    function setUp() public {
        fixture = new TestFixture();
    }
}
