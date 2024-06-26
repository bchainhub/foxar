# Configuration

Foxar's configuration system allows you to configure it's tools the way _you_ want while also providing with a
sensible set of defaults.

## Profiles

Configurations can be arbitrarily namespaced by profiles. Foxar's default config is also named `default`, but can
arbitrarily name and configure profiles as you like and set the `FOXAR_PROFILE` environment variable to the selected
profile's name. This results in foxar's tools (spark) preferring the values in the profile with the named that's set
in `FOXAR_PROFILE`. But all custom profiles inherit from the `default` profile.

## foxar.toml

Foxar's tools search for a `foxar.toml` or the filename in a `FOXAR_CONFIG` environment variable starting at the
current working directory. If it is not found, the parent directory, its parent directory, and so on are searched until
the file is found or the root is reached. But the typical location for the global `foxar.toml` would
be `~/.foxar/foxar.toml`, which is also checked. If the path set in `FOXAR_CONFIG` is absolute, no such search
takes place and the absolute path is used directly.

In `foxar.toml` you can define multiple profiles, therefore the file is assumed to be _nested_, so each top-level key
declares a profile and its values configure the profile.

The following is an example of what such a file might look like. This can also be obtained with `spark config`

```toml
## defaults for _all_ profiles
[profile.default]
src = "src"
out = "out"
libs = ["lib"]
ylem = "1.1.2" # to use a specific local ylem install set the path as `ylem = "<path to ylem>/ylem"`

## set only when the `spells` profile is selected
[profile.spells]
## --snip-- more settings
```

## Default profile

When determining the profile to use, `Config` considers the following sources in ascending priority order to read from
and merge, at the per-key level:

1. [`Config::default()`], which provides default values for all parameters.
2. `foxar.toml` _or_ TOML file path in `FOXAR_CONFIG` environment variable.
3. `FOXAR_` or `DAPP_` prefixed environment variables.

The selected profile is the value of the `FOXAR_PROFILE` environment variable, or if it is not set, "default".

### All Options

The following is a foxar.toml file with all configuration options set. See also [/config/src/lib.rs](/config/src/lib.rs) and [/cli/tests/it/config.rs](/cli/tests/it/config.rs).

```toml
## defaults for _all_ profiles
[profile.default]
src = 'src'
test = 'test'
script = 'script'
out = 'out'
libs = ['lib']
remappings = []
# list of libraries to link in the form of `<path to lib>:<lib name>:<address>`: `"src/MyLib.sol:MyLib:0x8De6DDbCd5053d32292AAA0D2105A32d108484a6"`
# the <path to lib> supports remappings
libraries = []
cache = true
cache_path = 'cache'
broadcast = 'broadcast'
# additional ylem allow paths
allow_paths = []
# additional ylem include paths
include_paths = []
force = false
energy_reports = ['*']
energy_reports_ignore = []
## Sets the concrete ylem version to use, this overrides the `auto_detect_ylem` value
# ylem = '1.1.2'
auto_detect_ylem = true
offline = false
optimizer = true
optimizer_runs = 200
model_checker = { contracts = { 'a.sol' = [
    'A1',
    'A2',
], 'b.sol' = [
    'B1',
    'B2',
] }, engine = 'chc', targets = [
    'assert',
    'outOfBounds',
], timeout = 10000 }
verbosity = 0
eth_rpc_url = "https://example.com/"
# Setting this option enables decoding of error traces from mainnet deployed / verfied contracts via etherscan
# ignore ylem warnings for missing license and exceeded contract size
# known error codes are: ["unreachable", "unused-return", "unused-param", "unused-var", "code-size", "shadowing", "func-mutability", "license", "pragma-solidity", "virtual-interfaces", "same-varname"]
# additional warnings can be added using their numeric error code: ["license", 1337]
ignored_error_codes = ["license", "code-size"]
deny_warnings = false
match_test = "Foo"
no_match_test = "Bar"
match_contract = "Foo"
no_match_contract = "Bar"
match_path = "*/Foo*"
no_match_path = "*/Bar*"
ffi = false
sender = '0xce591804c8ab1f12e6bbf3894d4083f33e07309d1f38'
tx_origin = '0xce591804c8ab1f12e6bbf3894d4083f33e07309d1f38'
initial_balance = '0xffffffffffffffffffffffff'
block_number = 0
fork_block_number = 0
chain_id = 1
# NOTE due to a toml-rs limitation, this value needs to be a string if the desired energy limit exceeds `i64::MAX` (9223372036854775807)
# `energy_limit = "Max"` is equivalent to `energy_limit = "18446744073709551615"`
energy_limit = 9223372036854775807
energy_price = 0
block_coinbase = '0x00000000000000000000000000000000000000000000'
block_timestamp = 0
block_difficulty = 0
block_energy_limit = 30000000
memory_limit = 33554432
extra_output = ["metadata"]
extra_output_files = []
names = false
sizes = false
via_ir = false
# caches storage retrieved locally for certain chains and endpoints
# can also be restricted to `chains = ["optimism", "mainnet"]`
# by default all endpoints will be cached, alternative options are "remote" for only caching non localhost endpoints and "<regex>"
# to disable storage caching entirely set `no_storage_caching = true`
rpc_storage_caching = { chains = "all", endpoints = "all" }
# this overrides `rpc_storage_caching` entirely
no_storage_caching = false
# Whether to store the referenced sources in the metadata as literal data.
use_literal_content = false
# use ipfs method to generate the metadata hash, ylem's default.
# To not include the metadata hash, to allow for deterministic code: https://docs.soliditylang.org/en/latest/metadata.html, use "none"
bytecode_hash = "ipfs"
# Whether to append the metadata hash to the bytecode
cbor_metadata = true
# How to treat revert (and require) reason strings.
# Possible values are: "default", "strip", "debug" and "verboseDebug".
#  "default" does not inject compiler-generated revert strings and keeps user-supplied ones.
# "strip" removes all revert strings (if possible, i.e. if literals are used) keeping side-effects
# "debug" injects strings for compiler-generated internal reverts, implemented for ABI encoders V1 and V2 for now.
# "verboseDebug" even appends further information to user-supplied revert strings (not yet implemented)
revert_strings = "default"
# If this option is enabled, Ylem is instructed to generate output (bytecode) only for the required contracts
# this can reduce compile time for `spark test` a bit but is considered experimental at this point.
sparse_mode = false
build_info = true
build_info_path = "build-info"
root = "root"
# Configures permissions for cheatcodes that touch the filesystem like `vm.writeFile`
# `access` restricts how the `path` can be accessed via cheatcodes
#    `read-write` | `true`   => `read` + `write` access allowed (`vm.readFile` + `vm.writeFile`)
#    `none`| `false` => no access
#    `read` => only read access (`vm.readFile`)
#    `write` => only write access (`vm.writeFile`)
# The `allowed_paths` further lists the paths that are considered, e.g. `./` represents the project root directory
# By default, only read access is granted to the project's out dir, so generated artifacts can be read by default
# following example enables read-write access for the project dir :
#       `fs_permissions = [{ access = "read-write", path = "./"}]`
fs_permissions = [{ access = "read", path = "./out"}]
[fuzz]
runs = 256
max_test_rejects = 65536
seed = '0x3e8'
dictionary_weight = 40
include_storage = true
include_push_bytes = true

[invariant]
runs = 256
depth = 15
fail_on_revert = false
call_override = false
dictionary_weight = 80
include_storage = true
include_push_bytes = true
shrink_sequence = true

[fmt]
line_length = 100
tab_width = 2
bracket_spacing = true
```

#### RPC-Endpoints settings

The `rpc_endpoints` value accepts a list of `alias = "<url|env var>"` pairs.

Environment variables need to be wrapped in `${}`

```toml
[rpc_endpoints]
mainnet = "${RPC_MAINNET}"
devin = "{RPC_DEVIN}"
```
