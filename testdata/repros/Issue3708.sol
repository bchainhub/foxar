// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

// https://github.com/foxar-rs/foxar/issues/3708
contract Issue3708Test is DSTest {
    // https://optimistic.etherscan.io/address/0xcb063edadf999cb7b8b3ebc71f5e97783176d289d640#code
    address constant CREATE2_DEPLOYER =
        0xcb063edadf999cb7b8b3ebc71f5e97783176d289d640;
    Cheats vm = Cheats(HEVM_ADDRESS());

    function setUp() public {
        string memory RPC_URL = "rpcAlias";
        uint256 forkId = vm.createSelectFork(RPC_URL);

        bytes
            memory code = hex"7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3";
        assertEq(CREATE2_DEPLOYER.code, code);
    }

    function test_deployer() public {
        bytes
            memory code = hex"7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe03601600081602082378035828234f58015156039578182fd5b8082525050506014600cf3";
        assertEq(CREATE2_DEPLOYER.code, code);
    }
}
