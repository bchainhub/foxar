# Anvil

### Crates to fix
- [x] Core - All tests pass except 2 where we need to figure out where to rlp encode network_id
- [x] rpc
- [x] server - No tests in this crate to fix
- [x] src
- [ ] tests


## Known problems

Right now if we call .hash() on transactions, it will always append the network_id at the end of the rlp. Fix later