#![allow(non_camel_case_types)]
use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Astore_2;

impl Instruction for Astore_2 {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            pc,
            op_code: OpCode::astore_2,
            icp: 0,
            wide: false,
        };

        (info, pc + 1)
    }
}
