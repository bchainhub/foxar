// SPDX-License-Identifier: Unlicense
pragma solidity >=1.1.0;

import "ds-test/test.sol";
import "./Cheats.sol";

contract RpcUrlTest is DSTest {
    Cheats cheats = Cheats(HEVM_ADDRESS());

    // returns the correct url
    function testCanGetRpcUrl() public {
        string memory url = cheats.rpcUrl("rpcAlias"); // note: this alias is pre-configured in the test runner
        assertEq(url, "https://xcbapi-arch-mainnet.coreblockchain.net/");
    }

    // returns an error if env alias does not exist
    function testRevertsOnMissingEnv() public {
        cheats.expectRevert(
            "Failed to resolve env var `RPC_ENV_ALIAS` in `${RPC_ENV_ALIAS}`: environment variable not found"
        );
        string memory url = cheats.rpcUrl("rpcEnvAlias");
    }

    // can set env and return correct url
    function testCanSetAndGetURLAndAllUrls() public {
        // this will fail because alias is not set
        cheats.expectRevert(
            "Failed to resolve env var `RPC_ENV_ALIAS` in `${RPC_ENV_ALIAS}`: environment variable not found"
        );
        string[2][] memory _urls = cheats.rpcUrls();

        string memory url = cheats.rpcUrl("rpcAlias");
        cheats.setEnv("RPC_ENV_ALIAS", url);
        string memory envUrl = cheats.rpcUrl("rpcEnvAlias");
        assertEq(url, envUrl);

        string[2][] memory allUrls = cheats.rpcUrls();
        assertEq(allUrls.length, 2);

        string[2] memory val = allUrls[0];
        assertEq(val[0], "rpcAlias");

        string[2] memory env = allUrls[1];
        assertEq(env[0], "rpcEnvAlias");
    }
}
