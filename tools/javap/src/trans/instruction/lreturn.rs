use super::{Instruction, InstructionInfo};
use classfile::OpCode;

pub struct Lreturn;

impl Instruction for Lreturn {
    fn run(&self, _codes: &[u8], pc: usize) -> (InstructionInfo, usize) {
        let info = InstructionInfo {
            op_code: OpCode::lreturn,
            icp: 0,
        };

        (info, pc + 1)
    }
}
