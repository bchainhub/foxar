// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

struct MyStruct {
    uint256 value;
}

contract MyContract {
    uint256 forkId;
    bytes32 blockHash;

    constructor(uint256 _forkId) public {
        forkId = _forkId;
        blockHash = blockhash(block.number - 1);
    }

    function ensureForkId(uint256 _forkId) public view {
        require(forkId == _forkId, "ForkId does not match");
    }

    function ensureBlockHash() public view {
        require(blockhash(block.number - 1) == blockHash, "Block Hash does not match");
    }
}

contract ForkTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS());

    uint256 mainnetFork;
    uint256 mainnetDiffFork;
    uint256 mainnetDiff2Fork;

    // this will create two _different_ forks during setup
    function setUp() public {
        mainnetFork = cheats.createFork("rpcAlias");
        mainnetDiffFork = cheats.createFork("rpcAlias", block.number - 1);

        mainnetDiff2Fork = cheats.createFork("https://xcbapi-arch-mainnet.coreblockchain.net/", block.number - 5);
    }

    // ensures forks use different ids
    function testForkIdDiffer() public {
        assert(mainnetFork != mainnetDiff2Fork);
    }

    // ensures forks use different ids
    function testCanSwitchForks() public {
        cheats.selectFork(mainnetFork);
        assertEq(mainnetFork, cheats.activeFork());
        cheats.selectFork(mainnetDiff2Fork);
        assertEq(mainnetDiff2Fork, cheats.activeFork());
        cheats.selectFork(mainnetDiff2Fork);
        assertEq(mainnetDiff2Fork, cheats.activeFork());
        cheats.selectFork(mainnetFork);
        assertEq(mainnetFork, cheats.activeFork());
    }

    function testCanCreateSelect() public {
        uint256 anotherFork = cheats.createSelectFork("rpcAlias");
        assertEq(anotherFork, cheats.activeFork());
    }

    // ensures forks have different block hashes
    function testBlockNumbersMimatch() public {
        cheats.selectFork(mainnetFork);
        uint256 num = block.number;
        bytes32 mainHash = blockhash(block.number - 1);
        cheats.selectFork(mainnetDiff2Fork);
        uint256 num2 = block.number;
        bytes32 mainnetDiff2Hash = blockhash(block.number - 1);
        assert(mainHash != mainnetDiff2Hash);
    }

    // test that we can switch between forks, and "roll" blocks
    function testCanRollFork() public {
        cheats.selectFork(mainnetFork);
        uint256 otherMain = cheats.createFork("rpcAlias", block.number - 1);
        cheats.selectFork(otherMain);
        uint256 mainBlock = block.number;

        uint256 forkedBlock = 7582918;
        uint256 otherFork = cheats.createFork("rpcAlias", forkedBlock);
        cheats.selectFork(otherFork);
        assertEq(block.number, forkedBlock);

        cheats.rollFork(forkedBlock + 1);
        assertEq(block.number, forkedBlock + 1);

        // can also roll by id
        cheats.rollFork(otherMain, mainBlock + 1);
        assertEq(block.number, forkedBlock + 1);

        cheats.selectFork(otherMain);
        assertEq(block.number, mainBlock + 1);
    }

    // test that we can "roll" blocks until a transaction
    function testCanRollForkUntilTransaction() public {
        // block to run transactions from
        uint256 block = 	7582986;

        // fork until previous block
        uint256 fork = cheats.createSelectFork("rpcAlias", block - 1);

        // block transactions in order: https://blockindex.net/block/7582986?page=1
        // run transactions from current block until tx
        bytes32 tx = 0x876c408048ffafdd25a5418986a3485874cefdc2351c3fa40ff373e86b7c808b;

        // account that sends ether in 3 transaction before tx
        address account = 0xcb917b0d24cadf52466ce86d28ecb9a8fd4694cc078d;

        assertEq(account.balance, 1969656649323000000000);

        // transfer: 26.9644 xcb (11.3964 + 11.4365 + 4.1315)
        // transaction 1: https://blockindex.net/tx/0x90a9212d766af332e323740b1129bf46a2182fc6f14e634ce18640a25fa26051
        // transaction 2: https://blockindex.net/tx/0xcde98a0cf64001efe75f9686e1036aea262b539182388bede1989f6f656f53e3
        // transaction 3: https://blockindex.net/tx/0x9565f9cdc5b237bd6ef10d8a41f1b2bca2888d6263614c54b92e09c6e34ab519

        uint256 transferAmount = 11396402873000000000 + 11436466201000000000 + 4131465242000000000;
        uint256 newBalance = account.balance - transferAmount;

        // execute transactions in block until tx
        cheats.rollFork(tx);

        // balance must be less than newBalance due to gas spent
        assert(account.balance < newBalance);
    }

    /// checks that marking as persistent works
    function testMarkPersistent() public {
        assert(cheats.isPersistent(address(this)));

        cheats.selectFork(mainnetFork);

        DummyContract dummy = new DummyContract();
        assert(!cheats.isPersistent(address(dummy)));

        uint256 expectedValue = 99;
        dummy.set(expectedValue);

        cheats.selectFork(mainnetDiff2Fork);

        cheats.selectFork(mainnetFork);
        assertEq(dummy.val(), expectedValue);
        cheats.makePersistent(address(dummy));
        assert(cheats.isPersistent(address(dummy)));

        cheats.selectFork(mainnetDiff2Fork);
        // the account is now marked as persistent and the contract is persistent across swaps
        dummy.hello();
        assertEq(dummy.val(), expectedValue);
    }

    // checks diagnostic
    function testNonExistingContractRevert() public {
        cheats.selectFork(mainnetFork);
        DummyContract dummy = new DummyContract();

        // this will succeed since `dummy` is deployed on the currently active fork
        string memory msg = dummy.hello();

        address dummyAddress = address(dummy);

        cheats.selectFork(mainnetDiffFork);
        assertEq(dummyAddress, address(dummy));

        // We emulate successful call to non-existent address (that will return with opcode=STOP
        // because the code of non-existent contract is zero (which is opcode of STOP) and then
        // reverting manually (in new ylem version this will be done on the contract level)

        dummyAddress.staticcall(hex"");
        revert();
    }
}

contract DummyContract {
    uint256 public val;

    function hello() external view returns (string memory) {
        return "hello";
    }

    function set(uint256 _val) public {
        val = _val;
    }
}
