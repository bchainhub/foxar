// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "../cheats/Cheats.sol";

contract DefaultAccessTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS());
    bytes constant FOXAR_WRITE_ERR =
        'The path "../testdata/fixtures/File/write_file.txt" is not allowed to be accessed for write operations.';

    function testReadFile() public {
        string memory path = "../testdata/fixtures/File/read.txt";
        cheats.readFile(path);

        cheats.readFileBinary(path);
    }

    function testReadLine() public {
        string memory path = "../testdata/fixtures/File/read.txt";
        cheats.readLine(path);
    }

    function testWriteFile() public {
        string memory path = "../testdata/fixtures/File/write_file.txt";
        string memory data = "hello writable world";
        cheats.expectRevert(FOXAR_WRITE_ERR);
        cheats.writeFile(path, data);

        cheats.writeFileBinary(path, bytes(data));
    }

    function testWriteLine() public {
        string memory path = "../testdata/fixtures/File/write_file.txt";
        string memory data = "hello writable world";
        cheats.expectRevert(FOXAR_WRITE_ERR);
        cheats.writeLine(path, data);
    }

    function testRemoveFile() public {
        string memory path = "../testdata/fixtures/File/write_file.txt";
        cheats.expectRevert(FOXAR_WRITE_ERR);
        cheats.removeFile(path);
    }
}
