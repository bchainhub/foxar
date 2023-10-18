use bytes::Bytes;
use revm::{
    inspectors::EnergyInspector,
    interpreter::{opcode, CallInputs, CreateInputs, Energy, InstructionResult, Interpreter},
    primitives::B176,
    Database, EVMData, Inspector,
};

#[derive(Clone, Default)]
pub struct TracePrinter {
    gas_inspector: EnergyInspector,
}

impl<DB: Database> Inspector<DB> for TracePrinter {
    fn initialize_interp(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        is_static: bool,
    ) -> InstructionResult {
        self.gas_inspector.initialize_interp(interp, data, is_static);
        InstructionResult::Continue
    }

    // get opcode by calling `interp.contract.opcode(interp.program_counter())`.
    // all other information can be obtained from interp.
    fn step(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        is_static: bool,
    ) -> InstructionResult {
        let opcode = interp.current_opcode();
        let opcode_str = opcode::OPCODE_JUMPMAP[opcode as usize];

        let gas_remaining = self.gas_inspector.energy_remaining();

        println!(
            "depth:{}, PC:{}, gas:{:#x}({}), OPCODE: {:?}({:?})  refund:{:#x}({}) Stack:{:?}, Data size:{}, Data: 0x{}",
            data.journaled_state.depth(),
            interp.program_counter(),
            gas_remaining,
            gas_remaining,
            opcode_str.unwrap(),
            opcode,
            interp.energy.refunded(),
            interp.energy.refunded(),
            interp.stack.data(),
            interp.memory.data().len(),
            hex::encode(interp.memory.data()),
        );

        self.gas_inspector.step(interp, data, is_static);

        InstructionResult::Continue
    }

    fn step_end(
        &mut self,
        interp: &mut Interpreter,
        data: &mut EVMData<'_, DB>,
        is_static: bool,
        eval: InstructionResult,
    ) -> InstructionResult {
        self.gas_inspector.step_end(interp, data, is_static, eval);
        InstructionResult::Continue
    }

    fn call(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        inputs: &mut CallInputs,
        is_static: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        println!(
            "SM CALL:   {:?},context:{:?}, is_static:{:?}, transfer:{:?}, input_size:{:?}",
            inputs.contract,
            inputs.context,
            is_static,
            inputs.transfer,
            inputs.input.len(),
        );
        (InstructionResult::Continue, Energy::new(0), Bytes::new())
    }

    fn call_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &CallInputs,
        remaining_gas: Energy,
        ret: InstructionResult,
        out: Bytes,
        is_static: bool,
    ) -> (InstructionResult, Energy, Bytes) {
        self.gas_inspector.call_end(data, inputs, remaining_gas, ret, out.clone(), is_static);
        (ret, remaining_gas, out)
    }

    fn create(
        &mut self,
        _data: &mut EVMData<'_, DB>,
        inputs: &mut CreateInputs,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        println!(
            "CREATE CALL: caller:{:?}, scheme:{:?}, value:{:?}, init_code:{:?}, gas:{:?}",
            inputs.caller,
            inputs.scheme,
            inputs.value,
            hex::encode(&inputs.init_code),
            inputs.energy_limit
        );
        (InstructionResult::Continue, None, Energy::new(0), Bytes::new())
    }

    fn create_end(
        &mut self,
        data: &mut EVMData<'_, DB>,
        inputs: &CreateInputs,
        ret: InstructionResult,
        address: Option<B176>,
        remaining_gas: Energy,
        out: Bytes,
    ) -> (InstructionResult, Option<B176>, Energy, Bytes) {
        self.gas_inspector.create_end(data, inputs, ret, address, remaining_gas, out.clone());
        (ret, address, remaining_gas, out)
    }
}
