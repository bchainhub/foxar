// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "ds-test/checksum.sol";
import "../cheats/Cheats.sol";

contract CreationRevertChild {
    uint256 public value = 123;
}

contract CreationRevertFactory {
    function predicted(bool deterministic) public view returns (address) {
        bytes32 hash;
        if (deterministic) {
            hash = keccak256(
                abi.encodePacked(hex"ff", address(this), bytes32(0), keccak256(type(CreationRevertChild).creationCode))
            );
        } else {
            // Fresh factory: nonce 1, RLP with a 22-byte Core address.
            hash = keccak256(abi.encodePacked(hex"d896", address(this), hex"01"));
        }
        return Checksum.toIcan(uint160(uint256(hash)));
    }

    function createAndRevert(bool deterministic) external {
        CreationRevertChild child;
        if (deterministic) {
            child = new CreationRevertChild{salt: bytes32(0)}();
        } else {
            child = new CreationRevertChild();
        }
        require(address(child) == predicted(deterministic), "wrong child address");
        revert("rollback");
    }
}

contract CreationRevertTest is DSTest {
    Cheats vm = Cheats(HEVM_ADDRESS());

    enum Preparation {
        Untouched,
        Deal,
        Transfer,
        CodeRead
    }

    function testCreate2Untouched() public {
        checkRollback(true, Preparation.Untouched);
    }

    function testCreate2Deal() public {
        checkRollback(true, Preparation.Deal);
    }

    function testCreate2Transfer() public {
        checkRollback(true, Preparation.Transfer);
    }

    function testCreate2CodeRead() public {
        checkRollback(true, Preparation.CodeRead);
    }

    function testCreateUntouched() public {
        checkRollback(false, Preparation.Untouched);
    }

    function testCreateDeal() public {
        checkRollback(false, Preparation.Deal);
    }

    function testCreateTransfer() public {
        checkRollback(false, Preparation.Transfer);
    }

    function testCreateCodeRead() public {
        checkRollback(false, Preparation.CodeRead);
    }

    function checkRollback(bool deterministic, Preparation preparation) internal {
        CreationRevertFactory factory = new CreationRevertFactory();
        address child = factory.predicted(deterministic);
        uint256 balance;
        if (preparation == Preparation.Deal) {
            vm.deal(child, 1);
            balance = 1;
        } else if (preparation == Preparation.Transfer) {
            vm.deal(address(this), 1);
            (bool funded,) = child.call{value: 1}("");
            require(funded, "funding failed");
            balance = 1;
        } else if (preparation == Preparation.CodeRead) {
            assertEq(child.code.length, 0);
        }

        (bool success, bytes memory reason) =
            address(factory).call(abi.encodeWithSelector(factory.createAndRevert.selector, deterministic));
        assertTrue(!success);
        // Prove creation succeeded before the parent reverted.
        assertEq(reason, abi.encodeWithSignature("Error(string)", "rollback"));
        assertEq(child.balance, balance);
        assertEq(uint256(vm.getNonce(address(factory))), 1);
        assertEq(uint256(vm.getNonce(child)), 0);
        assertEq(vm.load(child, bytes32(0)), bytes32(0));
        assertEq(child.code.length, 0, "child code survived parent REVERT");

        (bool callable, bytes memory result) = child.staticcall(abi.encodeWithSignature("value()"));
        assertTrue(callable);
        assertEq(result.length, 0, "getter survived parent REVERT");
    }
}
