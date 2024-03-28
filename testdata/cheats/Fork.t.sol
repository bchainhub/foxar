// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

interface IWETH {
    function deposit() external payable;

    function balanceOf(address) external view returns (uint256);
}

contract ForkTest is DSTest {
    address constant WETH_TOKEN_ADDR =
        0xcb19c7acc4c292d2943ba23c2eaa5d9c5a6652a8710c;
    uint256 constant mainblock = 7582885;

    Cheats cheats = Cheats(HEVM_ADDRESS());
    IWETH WETH = IWETH(WETH_TOKEN_ADDR);

    uint256 forkA;
    uint256 forkB;

    uint256 testValue;

    // this will create two _different_ forks during setup
    function setUp() public {
        forkA = cheats.createFork(
            "https://xcbapi-arch-mainnet.coreblockchain.net/",
            mainblock
        );
        forkB = cheats.createFork(
            "https://xcbapi-arch-mainnet.coreblockchain.net/",
            mainblock - 1
        );
        testValue = 999;
    }

    // ensures forks use different ids
    function testForkIdDiffer() public {
        assert(forkA != forkB);
    }

    // ensures we can create and select in one step
    function testCreateSelect() public {
        uint256 fork = cheats.createSelectFork(
            "https://xcbapi-arch-mainnet.coreblockchain.net/"
        );
        assertEq(fork, cheats.activeFork());
    }

    // ensures forks use different ids
    function testCanSwitchForks() public {
        cheats.selectFork(forkA);
        cheats.selectFork(forkB);
        cheats.selectFork(forkB);
        cheats.selectFork(forkA);
    }

    function testForksHaveSeparatedStorage() public {
        cheats.selectFork(forkA);
        // read state from forkA
        assert(
            WETH.balanceOf(0xcb74429635e186bb37ceaec87f88e2acbdc0392b0b3b) != 1
        );

        cheats.selectFork(forkB);
        // read state from forkB
        uint256 forkBbalance = WETH.balanceOf(
            0xcb74429635e186bb37ceaec87f88e2acbdc0392b0b3b
        );
        assert(forkBbalance != 1);

        cheats.selectFork(forkA);

        // modify state
        bytes32 value = bytes32(uint256(1));
        // "0xdfb571817f852d91cbe632c0bb6413b44f376a24fe4b9a2fe0b9e8135565aa65" is the slot storing the balance of zero address for the weth contract
        // `probe index address uint cb74429635e186bb37ceaec87f88e2acbdc0392b0b3b 3`
        bytes32 zero_address_balance_slot = 0xdfb571817f852d91cbe632c0bb6413b44f376a24fe4b9a2fe0b9e8135565aa65;
        cheats.store(WETH_TOKEN_ADDR, zero_address_balance_slot, value);
        assertEq(
            WETH.balanceOf(0xcb74429635e186bb37ceaec87f88e2acbdc0392b0b3b),
            1,
            "Cheatcode did not change value at the storage slot."
        );

        // switch forks and ensure the balance on forkB remains untouched
        cheats.selectFork(forkB);
        assert(forkBbalance != 1);
        // balance of forkB is untouched
        assertEq(
            WETH.balanceOf(0xcb74429635e186bb37ceaec87f88e2acbdc0392b0b3b),
            forkBbalance,
            "Cheatcode did not change value at the storage slot."
        );
    }

    function testCanShareDataAcrossSwaps() public {
        assertEq(testValue, 999);

        uint256 val = 300;
        cheats.selectFork(forkA);
        assertEq(val, 300);

        testValue = 100;

        cheats.selectFork(forkB);
        assertEq(val, 300);
        assertEq(testValue, 100);

        val = 99;
        testValue = 300;

        cheats.selectFork(forkA);
        assertEq(val, 99);
        assertEq(testValue, 300);
    }

    // ensures forks use different ids
    function testCanChangeChainId() public {
        cheats.selectFork(forkA);
        uint256 newChainId = 1337;
        cheats.chainId(newChainId);
        uint256 expected = block.chainid;
        assertEq(newChainId, expected);
    }
}
