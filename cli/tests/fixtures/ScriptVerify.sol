// SPDX-License-Identifier: UNLICENSED
pragma solidity >=1.1.0;

import {Unique} from "./unique.sol";

interface HEVM {
    function startBroadcast() external;
}

library F {
    function f() public pure returns (uint256) {
        return 1;
    }
}

library C {
    function c() public pure returns (uint256) {
        return 2;
    }
}

contract Hello {
    function world() public {
        F.f();
        C.c();
    }
}

contract CC1 is Unique {
    uint256 a;

    constructor(uint256 _a) {
        a = _a;
    }
}

contract CC2 is Unique {
    uint8 b;

    constructor(uint256 _b) {
        b = uint8(_b);
        new CC3("hello");
    }
}

contract CC3 is Unique {
    string c;

    constructor(string memory _c) {
        c = _c;
    }
}

contract InnerContracts is Unique {
    constructor(uint256 _a) public {
        CC1 c1 = new CC1(_a);
    }

    function c2(uint256 _b) public {
        CC2 c2 = new CC2{salt: bytes32(uint256(1))}(_b);
    }
}

contract ScriptVerify {
    function run() public {
        address vm = address(0xcb69fc06a12b7a6f30e2a3c16a3b5d502cd71c20f2f8);
        HEVM(vm).startBroadcast();
        new Hello();
        InnerContracts contracts = new InnerContracts(1);
        contracts.c2(3);
    }
}
