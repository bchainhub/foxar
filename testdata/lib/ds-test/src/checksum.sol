// SPDX-License-Identifier: MIT

pragma solidity ^1.1.0;

library Checksum {

    function _getChainId() internal view returns (uint8) {
        return uint8(block.chainid);
    }

    function _getChainPrefix() internal view returns (uint8) {
        uint8 chainId = _getChainId();
        if (chainId == 1) { // mainnet - 'cb'
                return 203;
        } else if (chainId == 3) { // devin network - 'ab'
                return 171;
        }
        return 206; // private network - 'ce'
    }

    function toIcan(uint160 rawAddress) internal view returns (address) {
        uint8 prefix = _getChainPrefix();
        uint176 value = uint176(rawAddress);
        value = (value << 16) + (uint176(prefix) << 8);
        uint176 v = value;
        uint256 s = 0;
        uint256 x = 1;
        for (uint i = 0; i < 44; i++) {
            uint256 t = v & 0x0f;
            s = s + t * x;
            x *= 10 + 90 * (t / 10);
            v >>= 4;
        }
        s = s % 97;
        s = 98 - s;
        s = (s % 10) + (s / 10) * 16;
        uint176 result;
        result = uint176(rawAddress) + (uint176(s) << 160) + (uint176(prefix) << 168);
        return address(result);
    }
}