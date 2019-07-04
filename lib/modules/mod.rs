//! Modules are optional enhancements to analysis.

use crate::error::*;
use crate::ir;

pub trait Module {
    /// Apply modifications to functions before analysis takes place
    fn pre_analysis_function(&self, function: &mut ir::Function<ir::Constant>) -> Result<()>;
}

pub struct MipsT9 {}

impl Module for MipsT9 {
    fn pre_analysis_function(&self, function: &mut ir::Function<ir::Constant>) -> Result<()> {
        // Get the entry for this function
        let entry_index = match function.control_flow_graph().entry() {
            Some(index) => index,
            None => return Ok(()),
        };

        let function_address = function.address();

        // We need the block for this entry index
        let block = function.block_mut(entry_index)?;

        // Prepend an assignment to t9 which sets it equal to the function entry
        let operation = ir::Operation::Assign {
            dst: ir::scalar("$t9", 32).into(),
            src: ir::Constant::new(function_address, 32).into(),
        };
        block.prepend_operation(operation);

        Ok(())
    }
}