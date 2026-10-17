## Installation
1. Run
```console
curl -L https://up.foxar.dev/ | bash
```

2. Then just run foxarup
```console
foxarup
```

Foxarup will download the latest binaries from github. After foxarup finishes you can use all binaries.

For more information and more custom instalation of foxar, check out the README in [foxarup/](https://github.com/bchainhub/foxar/tree/master/foxarup) folder.


Here are the references to the original foundry names:
forge: spark
anvil: shuttle
chisel: pilot
cast: probe

## Creation revert regression

The workspace pins CVM to
[`1eda574`](https://github.com/core-coin/cvm/commit/1eda5741bc5ac599d724f42c8c572aefc45f6e58).
The fix journals the previous bytecode and code hash,
so reverting a parent call removes code deployed by a successful child CREATE
or CREATE2. The regression tests cover untouched addresses, prefunding via
`deal` and native transfer, and an earlier EXTCODESIZE read.

Cargo fetches the corrected CVM revision directly from GitHub during the build.

Build and run the focused regression with Rust 1.87.0 and Ylem 1.1.2:

```bash
RUSTC_BOOTSTRAP=1 cargo build --locked -p foxar-cli --bin spark
FOXAR_TEST=testdata/repros/CreationRevert.t.sol target/debug/spark test \
  --offline --use 1.1.2 --contracts testdata/repros/CreationRevert.t.sol \
  --remappings ds-test/=testdata/lib/ds-test/src/ \
  --cache-path target/creation-revert-cache --match-contract CreationRevertTest -vv
```

`RUSTC_BOOTSTRAP` enables the feature gates used by this CVM version. The build
creates `target/debug/spark`; it does not replace an installed Spark binary.
