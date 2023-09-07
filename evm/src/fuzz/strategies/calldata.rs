use super::fuzz_param;
use corebc::{
    abi::Function,
    types::{Bytes, Network},
};
use proptest::prelude::{BoxedStrategy, Strategy};

/// Given a function, it returns a strategy which generates valid calldata
/// for that function's input types.
pub fn fuzz_calldata(func: Function, network: &Network) -> BoxedStrategy<Bytes> {
    // We need to compose all the strategies generated for each parameter in all
    // possible combinations
    let strats =
        func.inputs.iter().map(|input| fuzz_param(&input.kind, *network)).collect::<Vec<_>>();

    strats
        .prop_map(move |tokens| {
            trace!(input = ?tokens);
            func.encode_input(&tokens).unwrap().into()
        })
        .boxed()
}
