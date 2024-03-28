// SPDX-License-Identifier: Unlicense
pragma solidity ^1.1.0;

import "ds-test/test.sol";
import "./DssExecLib.sol";

interface Cheats {
    function store(address account, bytes32 slot, bytes32 value) external;

    function activeFork() external returns (uint256);
}

interface IWETH {
    function deposit() external payable;

    function balanceOf(address) external view returns (uint256);
}

// A minimal contract. We test if it is deployed correctly
contract DummyContract {
    address public deployer;

    constructor() public {
        deployer = msg.sender;
    }
}

contract ForkTest is DSTest {
    address constant DAI_TOKEN_ADDR =
        0xcb19c7acc4c292d2943ba23c2eaa5d9c5a6652a8710c;
    address constant WETH_TOKEN_ADDR =
        0xcb19c7acc4c292d2943ba23c2eaa5d9c5a6652a8710c;

    // checks that we can retrieve the fork we launched with
    function testActiveFork() public {
        Cheats cheatvm = Cheats(HEVM_ADDRESS());
        uint256 activeFork = cheatvm.activeFork();
        // launch fork has id `0`
        assertEq(activeFork, 0);
    }

    function testReadState() public {
        ERC20 DAI = ERC20(DAI_TOKEN_ADDR);
        assertEq(
            uint256(DAI.decimals()),
            uint256(18),
            "Failed to read DAI token decimals."
        );
    }

    function testDeployContract() public {
        DummyContract dummy = new DummyContract();
        uint256 size;
        address DummyAddress = address(dummy);
        assembly {
            size := extcodesize(DummyAddress)
        }
        assertGt(
            size,
            0,
            "Deploying dummy contract failed. Deployed size of zero"
        );
        assertEq(
            dummy.deployer(),
            address(this),
            "Calling the Dummy contract failed to return expected value"
        );
    }
    function testCheatcode() public {
        Cheats cheatvm = Cheats(HEVM_ADDRESS());
        IWETH WETH = IWETH(WETH_TOKEN_ADDR);
        bytes32 value = bytes32(uint256(1));
        // "0xdfb571817f852d91cbe632c0bb6413b44f376a24fe4b9a2fe0b9e8135565aa65" is the slot storing the balance of zero address for the weth contract
        // `probe index address uint cb74429635e186bb37ceaec87f88e2acbdc0392b0b3b 3`
        bytes32 zero_address_balance_slot = 0xdfb571817f852d91cbe632c0bb6413b44f376a24fe4b9a2fe0b9e8135565aa65;
        cheatvm.store(WETH_TOKEN_ADDR, zero_address_balance_slot, value);
        assertEq(
            WETH.balanceOf(0xcb74429635e186bb37ceaec87f88e2acbdc0392b0b3b),
            1,
            "Cheatcode did not change value at the storage slot."
        );
    }

    /* todo:error2215 currenty we do not have SC that has deposit function so it fails
    function testDepositWeth() public {
        IWETH WETH = IWETH(WETH_TOKEN_ADDR);
        WETH.deposit{value: 1000}();
        assertEq(
            WETH.balanceOf(address(this)),
            1000,
            "WETH balance is not equal to deposited amount."
        );
    }
    */
}
