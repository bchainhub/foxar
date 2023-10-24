use super::{
    inspector::{Cheatcodes, Fuzzer, InspectorStackConfig},
    Executor,
};
use crate::{
    executor::{backend::Backend, inspector::CheatsConfig},
    fuzz::{invariant::RandomCallGenerator, strategies::EvmFuzzState},
    utils::ru256_to_u256,
};
use corebc::types::U256;
use revm::primitives::{Env, SpecId};

/// The builder that allows to configure an evm [`Executor`] which a stack of optional
/// [`revm::Inspector`]s, such as [`Cheatcodes`]
///
/// By default, the [`Executor`] will be configured with an empty [`InspectorStack`]
#[derive(Default, Debug)]
pub struct ExecutorBuilder {
    /// The execution environment configuration.
    env: Env,
    /// The configuration used to build an [InspectorStack].
    inspector_config: InspectorStackConfig,
    energy_limit: Option<U256>,
}

// === impl ExecutorBuilder ===

impl ExecutorBuilder {
    /// Enables cheatcodes on the executor.
    #[must_use]
    pub fn with_cheatcodes(mut self, config: CheatsConfig) -> Self {
        self.inspector_config.cheatcodes = Some(Cheatcodes::new(
            self.env.block.clone(),
            ru256_to_u256(self.env.tx.energy_price),
            config,
        ));
        self
    }

    /// Enables or disables tracing
    #[must_use]
    pub fn set_tracing(mut self, enable: bool) -> Self {
        self.inspector_config.tracing = enable;
        self
    }

    /// Enables or disables the debugger
    #[must_use]
    pub fn set_debugger(mut self, enable: bool) -> Self {
        self.inspector_config.debugger = enable;
        self
    }

    /// Enables or disables coverage collection
    #[must_use]
    pub fn set_coverage(mut self, enable: bool) -> Self {
        self.inspector_config.coverage = enable;
        self
    }

    /// Enables or disabled trace printer.
    #[must_use]
    pub fn set_trace_printer(mut self, enable: bool) -> Self {
        self.inspector_config.trace_printer = enable;
        self
    }

    /// Enables the fuzzer for data collection and maybe call overriding
    #[must_use]
    pub fn with_fuzzer(
        mut self,
        call_generator: Option<RandomCallGenerator>,
        fuzz_state: EvmFuzzState,
    ) -> Self {
        self.inspector_config.fuzzer = Some(Fuzzer { call_generator, fuzz_state, collect: false });
        self
    }

    /// Sets the EVM spec to use
    #[must_use]
    pub fn with_spec(mut self, spec: SpecId) -> Self {
        self.env.cfg.spec_id = spec;
        self
    }

    /// Sets the executor energy limit.
    ///
    /// See [Executor::energy_limit] for more info on why you might want to set this.
    #[must_use]
    pub fn with_energy_limit(mut self, energy_limit: U256) -> Self {
        self.energy_limit = Some(energy_limit);
        self
    }

    /// Configure the execution environment (energy limit, chain spec, ...)
    #[must_use]
    pub fn with_config(mut self, env: Env) -> Self {
        self.inspector_config.block = env.block.clone();
        self.inspector_config.energy_price = ru256_to_u256(env.tx.energy_price);
        self.env = env;
        self
    }

    /// Enable the chisel state inspector
    #[must_use]
    pub fn with_chisel_state(mut self, final_pc: usize) -> Self {
        self.inspector_config.chisel_state = Some(final_pc);
        self
    }

    /// Builds the executor as configured.
    pub fn build(self, db: Backend) -> Executor {
        let energy_limit = self.energy_limit.unwrap_or(ru256_to_u256(self.env.block.energy_limit));
        Executor::new(db, self.env, self.inspector_config, energy_limit)
    }
}
