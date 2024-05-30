# `spark`

Spark is a fast and flexible Core testing framework, inspired by
[Dapp](https://github.com/dapphub/dapptools/tree/master/src/dapp).

If you are looking into how to consume the software as an end user, check the
[CLI README](../cli/README.md).

For more context on how the package works under the hood, look in the
[code docs](./src/lib.rs).

**Need help with Spark? Read the [📖 Foxar Book (Spark Guide)][foxar-book-spark-guide] (WIP)!**

[foxar-book-spark-guide]: https://foxar.dev/

## Why

### Write your tests in Ylem to minimize context switching

Writing tests in Javascript/Typescript while writing your smart contracts in
Ylem can be confusing. Spark lets you write your tests in Ylem, so you
can focus on what matters.

```solidity
contract Foo {
    uint256 public x = 1;
    function set(uint256 _x) external {
        x = _x;
    }

    function double() external {
        x = 2 * x;
    }
}

contract FooTest {
    Foo foo;

    // The state of the contract gets reset before each
    // test is run, with the `setUp()` function being called
    // each time after deployment.
    function setUp() public {
        foo = new Foo();
    }

    // A simple unit test
    function testDouble() public {
        require(foo.x() == 1);
        foo.double();
        require(foo.x() == 2);
    }
}
```

