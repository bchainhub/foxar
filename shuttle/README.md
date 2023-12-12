# Anvil

### Crates to fix
- [x] Core - All tests pass except 2 where we need to figure out where to rlp encode network_id
- [x] rpc
- [x] server - No tests in this crate to fix
- [x] src
- [ ] tests


## Known problems

Right now if we call .hash() on transactions, it will always append the network_id at the end of the rlp. Fix later


Currently the default network_id for testing is 1. In original shuttle this is 31337. We should probably change this in the future but for now it is ok to use it with this chain id.

When we requeset network id through the rpc it returns it in hexadecimal but it acts as decimal
