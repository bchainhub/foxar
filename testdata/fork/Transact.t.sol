// SPDX-License-Identifier: Unlicense
pragma solidity 1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";
import "../logs/console.sol";

interface IERC20 {
    function transfer(address to, uint256 amount) external returns (bool);

    function balanceOf(address account) external view returns (uint256);
}

contract TransactOnForkTest is DSTest {
    Cheats constant vm = Cheats(HEVM_ADDRESS);

    IERC20 constant USDT = IERC20(0xcb19c7acc4c292d2943ba23c2eaa5d9c5a6652a8710c);

    event Transfer(address indexed from, address indexed to, uint256 value);

    function testTransact() public {
        // A random block https://blockindex.net/block/7591477
        uint256 fork = vm.createFork("rpcAlias", 7591477);
        vm.selectFork(fork);
        // a random transfer transaction in the next block: https://blockindex.net/tx/0x43f35862462e01cfe10f849bb7ed05a492a8283b43ec86e574c00714d283c29f
        bytes32 tx = 0x43f35862462e01cfe10f849bb7ed05a492a8283b43ec86e574c00714d283c29f;

        address sender = address(0xcb423001ff40e3b13d2f6504341d7c9390678b3b0109);
        address recipient = address(0xcb1857f0c7de7b016b00a777d27f7685e033d52601e0);

        assertEq(sender.balance, 74640493951869914049536);
        assertEq(recipient.balance, 95710415892000000000);

        // transfer amount: 4499.9995 XCB
        uint256 transferAmount = 4499999500000000000000;
        uint256 expectedRecipientBalance = recipient.balance + transferAmount;
        uint256 expectedSenderBalance = sender.balance - transferAmount;

        // execute the transaction
        vm.transact(tx);

        // recipient received transfer
        assertEq(recipient.balance, expectedRecipientBalance);

        // decreased by transferAmount and gas
        assert(sender.balance < expectedSenderBalance);
    }

    function testTransactCooperatesWithCheatcodes() public {
        // A random block https://blockindex.net/block/7591437
        uint256 fork = vm.createFork("rpcAlias", 7591437);
        vm.selectFork(fork);

        // a random CRC20 CTN transfer transaction in the next block: https://blockindex.net/tx/0x31677254f2d9e82dc01a234befa46daa630a30620bd36682cdfc1d607614165e
        bytes32 tx = 0x31677254f2d9e82dc01a234befa46daa630a30620bd36682cdfc1d607614165e;

        address sender = address(0xcb8882600fca3cde87799f01e2570d7d3d1c1c0d1dd4);
        address recipient = address(0xcb580851379288d0c6b251af9f49988ccca9d6502948);

        uint256 senderBalance = USDT.balanceOf(sender);
        uint256 recipientBalance = USDT.balanceOf(recipient);

        assertEq(senderBalance, 73622256750103146242167552);
        assertEq(recipientBalance, 1616556078400002920448);

        // transfer amount: 499 999 CTN
        uint256 transferAmount = 499999999999999991611392;
        uint256 expectedRecipientBalance = recipientBalance + transferAmount;
        uint256 expectedSenderBalance = senderBalance - transferAmount;

        // expect a call to USDT's transfer
        vm.expectCall(address(USDT), abi.encodeWithSelector(IERC20.transfer.selector, recipient, transferAmount));

        // expect a Transfer event to be emitted
        vm.expectEmit(true, true, false, true, address(USDT));
        emit Transfer(address(sender), address(recipient), transferAmount);

        // start recording logs
        vm.recordLogs();

        // execute the transaction
        vm.transact(tx);

        // extract recorded logs
        Cheats.Log[] memory logs = vm.getRecordedLogs();

        senderBalance = USDT.balanceOf(sender);
        recipientBalance = USDT.balanceOf(recipient);

        // recipient received transfer
        assertEq(recipientBalance, expectedRecipientBalance);

        // decreased by transferAmount
        assertEq(senderBalance, expectedSenderBalance);

        // recorded a `Transfer` log
        assertEq(logs.length, 1);
    }
}
