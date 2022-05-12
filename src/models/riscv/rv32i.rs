use crate::{traits::bit_vector::BitVector, ScfiaStdlib};
use std::rc::Rc;
use std::cell::RefCell;

use crate::ActiveAst;
use crate::expressions::bv_add_expression::BVAddExpression;
use crate::expressions::bv_slice_expression::BVSliceExpression;
use crate::values::bit_vector_concrete::BitVectorConcrete;

pub struct SystemState {
    pub x0: Rc<RefCell<dyn ActiveAst>>,
    pub x1: Rc<RefCell<dyn ActiveAst>>,
    pub x2: Rc<RefCell<dyn ActiveAst>>,
    pub x3: Rc<RefCell<dyn ActiveAst>>,
    pub x4: Rc<RefCell<dyn ActiveAst>>,
    pub x5: Rc<RefCell<dyn ActiveAst>>,
    pub x6: Rc<RefCell<dyn ActiveAst>>,
    pub x7: Rc<RefCell<dyn ActiveAst>>,
    pub x8: Rc<RefCell<dyn ActiveAst>>,
    pub x9: Rc<RefCell<dyn ActiveAst>>,
    pub x10: Rc<RefCell<dyn ActiveAst>>,
    pub x11: Rc<RefCell<dyn ActiveAst>>,
    pub x12: Rc<RefCell<dyn ActiveAst>>,
    pub x13: Rc<RefCell<dyn ActiveAst>>,
    pub x14: Rc<RefCell<dyn ActiveAst>>,
    pub x15: Rc<RefCell<dyn ActiveAst>>,
    pub x16: Rc<RefCell<dyn ActiveAst>>,
    pub x17: Rc<RefCell<dyn ActiveAst>>,
    pub x18: Rc<RefCell<dyn ActiveAst>>,
    pub x19: Rc<RefCell<dyn ActiveAst>>,
    pub x20: Rc<RefCell<dyn ActiveAst>>,
    pub x21: Rc<RefCell<dyn ActiveAst>>,
    pub x22: Rc<RefCell<dyn ActiveAst>>,
    pub x23: Rc<RefCell<dyn ActiveAst>>,
    pub x24: Rc<RefCell<dyn ActiveAst>>,
    pub x25: Rc<RefCell<dyn ActiveAst>>,
    pub x26: Rc<RefCell<dyn ActiveAst>>,
    pub x27: Rc<RefCell<dyn ActiveAst>>,
    pub x28: Rc<RefCell<dyn ActiveAst>>,
    pub x29: Rc<RefCell<dyn ActiveAst>>,
    pub x30: Rc<RefCell<dyn ActiveAst>>,
    pub x31: Rc<RefCell<dyn ActiveAst>>,
    pub pc: Rc<RefCell<dyn ActiveAst>>,
}

pub struct ComplexSpecialStruct {
    pub some_flag: Rc<RefCell<dyn ActiveAst>>,
}

pub unsafe fn reset(state: *mut SystemState, stdlib: &mut ScfiaStdlib) {
    (*state).x0 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x1 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x2 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x3 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x4 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x5 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x6 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x7 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x8 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x9 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x10 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x11 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x12 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x13 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x14 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x15 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x16 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x17 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x18 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x19 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x20 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x21 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x22 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x23 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x24 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x25 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x26 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x27 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x28 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x29 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x30 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
    (*state).x31 = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
}

pub unsafe fn sum(state: *mut SystemState, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn ActiveAst>> {
    return Rc::new(RefCell::new(BVAddExpression::new((*state).x1.clone(), (*state).x2.clone(), stdlib)));
}

pub unsafe fn test(state: *mut SystemState, stdlib: &mut ScfiaStdlib) {
    (*state).x3 = Rc::new(RefCell::new(BVAddExpression::new(sum(state, stdlib), sum(state, stdlib), stdlib)));
}

pub unsafe fn step(state: *mut SystemState, stdlib: &mut ScfiaStdlib) {
    let instruction_32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinPhysicalMemoryReadFunctionDeclaration");;
    let opcode: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 6, 0, stdlib)));
    if true {
        let funct3: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, stdlib)));
        if true {
            let rd: Rc<RefCell<dyn ActiveAst>> = extract_rd_32(instruction_32.clone(), stdlib);
            let rs1: Rc<RefCell<dyn ActiveAst>> = extract_rs1_32(instruction_32.clone(), stdlib);
            let imm: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, stdlib)));
            let imm32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinSignExtendFunctionDeclaration");;
            let base_address: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, rs1.clone(), stdlib);
            let address: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new(base_address.clone(), imm32.clone(), stdlib)));
            let value: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinPhysicalMemoryReadFunctionDeclaration");;
            register_write_BV32(state, rd.clone(), value.clone(), stdlib);
            progress_pc_4(state, stdlib);
        }
        else {
            unimplemented!();
        }
    }
    else if true {
        let funct3: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, stdlib)));
        if true {
            let rd: Rc<RefCell<dyn ActiveAst>> = extract_rd_32(instruction_32.clone(), stdlib);
            let rs1: Rc<RefCell<dyn ActiveAst>> = extract_rs1_32(instruction_32.clone(), stdlib);
            let offset: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, stdlib)));
            let offset_32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinSignExtendFunctionDeclaration");;
            let result: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new(offset_32.clone(), register_read_BV32(state, rs1.clone(), stdlib), stdlib)));
            register_write_BV32(state, rd.clone(), result.clone(), stdlib);
            progress_pc_4(state, stdlib);
        }
        else if true {
            let rd: Rc<RefCell<dyn ActiveAst>> = extract_rd_32(instruction_32.clone(), stdlib);
            let rs1: Rc<RefCell<dyn ActiveAst>> = extract_rs1_32(instruction_32.clone(), stdlib);
            let imm: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, stdlib)));
            let imm_32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinSignExtendFunctionDeclaration");;
            let result: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinOrFunctionDeclaration");;
            register_write_BV32(state, rd.clone(), result.clone(), stdlib);
            progress_pc_4(state, stdlib);
        }
        else {
            unimplemented!();
        }
    }
    else if true {
        let funct3: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, stdlib)));
        if true {
            let rs1: Rc<RefCell<dyn ActiveAst>> = extract_rs1_32(instruction_32.clone(), stdlib);
            let rs2: Rc<RefCell<dyn ActiveAst>> = extract_rs2_32(instruction_32.clone(), stdlib);
            let offset_11_5: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 25, stdlib)));
            let offset_4_0: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 11, 7, stdlib)));
            let offset: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
            let offset_32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinSignExtendFunctionDeclaration");;
            let base_address: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, rs1.clone(), stdlib);
            let address: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new(base_address.clone(), offset_32.clone(), stdlib)));
            let value: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, rs2.clone(), stdlib);
            unimplemented!("SISALBuiltinPhysicalMemoryWriteFunctionDeclaration");;
            progress_pc_4(state, stdlib);
        }
        else {
            unimplemented!();
        }
    }
    else if true {
        let rd: Rc<RefCell<dyn ActiveAst>> = extract_rd_32(instruction_32.clone(), stdlib);
        let rs: Rc<RefCell<dyn ActiveAst>> = extract_rs1_32(instruction_32.clone(), stdlib);
        let imm: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 12, stdlib)));
        let value: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
        register_write_BV32(state, rd.clone(), value.clone(), stdlib);
        progress_pc_4(state, stdlib);
    }
    else if true {
        let dst: Rc<RefCell<dyn ActiveAst>> = extract_rd_32(instruction_32.clone(), stdlib);
        let imm: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 12, stdlib)));
        let imm32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
        let sum: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new(imm32.clone(), (*state).pc.clone(), stdlib)));
        register_write_BV32(state, dst.clone(), sum.clone(), stdlib);
        progress_pc_4(state, stdlib);
    }
    else if true {
        let funct3: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, stdlib)));
        let funct7: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 25, stdlib)));
        let rd: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 11, 7, stdlib)));
        let rs1: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 19, 15, stdlib)));
        let rs2: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 24, 20, stdlib)));
        if true {
            if true {
                execute_add32(state, rd.clone(), rs2.clone(), rs2.clone(), stdlib);
                progress_pc_4(state, stdlib);
            }
            else if true {
                (*state).pc = Rc::new(RefCell::new(BitVectorConcrete::new(0b0, 32, stdlib)));
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if true {
            if true {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if true {
            if true {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if true {
            if true {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if true {
            if true {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if true {
            if true {
                unimplemented!();
            }
            else if true {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if true {
            if true {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else if true {
            if true {
                unimplemented!();
            }
            else {
                unimplemented!();
            }
        }
        else {
            unimplemented!();
        }
    }
    else if true {
        let funct3: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, stdlib)));
        if true {
            unimplemented!();
        }
        else if true {
            let lhs: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, extract_rs1_32(instruction_32.clone(), stdlib), stdlib);
            let rhs: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, extract_rs2_32(instruction_32.clone(), stdlib), stdlib);
            if true {
                let imm_4_1: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 11, 8, stdlib)));
                let imm_10_5: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 30, 25, stdlib)));
                let imm11: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 7, 7, stdlib)));
                let imm12: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 31, stdlib)));
                let imm_4_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
                let imm_10_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
                let imm_11_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
                let imm_12_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
                let offset: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinSignExtendFunctionDeclaration");;
                let _address: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), offset.clone(), stdlib)));
                (*state).pc = _address.clone();
            }
            else {
                progress_pc_4(state, stdlib);
            }
        }
        else {
            unimplemented!();
        }
    }
    else if true {
        let funct3: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 14, 12, stdlib)));
        if true {
            let dst: Rc<RefCell<dyn ActiveAst>> = extract_rd_32(instruction_32.clone(), stdlib);
            let rs1: Rc<RefCell<dyn ActiveAst>> = extract_rs1_32(instruction_32.clone(), stdlib);
            let s1: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, rs1.clone(), stdlib);
            let offset: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 20, stdlib)));
            let offset_32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinSignExtendFunctionDeclaration");;
            let address: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new(s1.clone(), offset_32.clone(), stdlib)));
            let return_address: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 32, stdlib))), stdlib)));
            (*state).pc = address.clone();
            register_write_BV32(state, dst.clone(), return_address.clone(), stdlib);
        }
        else {
            unimplemented!();
        }
    }
    else if true {
        let imm_20: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 31, 31, stdlib)));
        let imm_10_1: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 30, 21, stdlib)));
        let imm_11: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 20, 20, stdlib)));
        let imm_19_12: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVSliceExpression::new(instruction_32.clone(), 19, 12, stdlib)));
        let offset_10_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
        let offset_11_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
        let offset_19_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
        let offset_20_0: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinConcatFunctionDeclaration");;
        let offset_32: Rc<RefCell<dyn ActiveAst>> = unimplemented!("SISALBuiltinSignExtendFunctionDeclaration");;
        let address: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), offset_32.clone(), stdlib)));
        let return_address: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new((*state).pc.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 32, stdlib))), stdlib)));
        let dst: Rc<RefCell<dyn ActiveAst>> = extract_rd_32(instruction_32.clone(), stdlib);
        register_write_BV32(state, dst.clone(), return_address.clone(), stdlib);
        (*state).pc = address.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn extract_rd_32(op: Rc<RefCell<dyn ActiveAst>>, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn ActiveAst>> {
    return Rc::new(RefCell::new(BVSliceExpression::new(op.clone(), 11, 7, stdlib)));
}

pub unsafe fn extract_rs1_32(op: Rc<RefCell<dyn ActiveAst>>, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn ActiveAst>> {
    return Rc::new(RefCell::new(BVSliceExpression::new(op.clone(), 19, 15, stdlib)));
}

pub unsafe fn extract_rs2_32(op: Rc<RefCell<dyn ActiveAst>>, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn ActiveAst>> {
    return Rc::new(RefCell::new(BVSliceExpression::new(op.clone(), 24, 20, stdlib)));
}

pub unsafe fn progress_pc_4(state: *mut SystemState, stdlib: &mut ScfiaStdlib) {
    let old_pc: Rc<RefCell<dyn ActiveAst>> = (*state).pc.clone();
    let new_pc: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new(old_pc.clone(), Rc::new(RefCell::new(BitVectorConcrete::new(0b100, 32, stdlib))), stdlib)));
    (*state).pc = new_pc.clone();
}

pub unsafe fn register_write_BV32(state: *mut SystemState, register_id: Rc<RefCell<dyn ActiveAst>>, value: Rc<RefCell<dyn ActiveAst>>, stdlib: &mut ScfiaStdlib) {
    if true {
        (*state).x0 = value.clone();
    }
    else if true {
        (*state).x1 = value.clone();
    }
    else if true {
        (*state).x2 = value.clone();
    }
    else if true {
        (*state).x3 = value.clone();
    }
    else if true {
        (*state).x4 = value.clone();
    }
    else if true {
        (*state).x5 = value.clone();
    }
    else if true {
        (*state).x6 = value.clone();
    }
    else if true {
        (*state).x7 = value.clone();
    }
    else if true {
        (*state).x8 = value.clone();
    }
    else if true {
        (*state).x9 = value.clone();
    }
    else if true {
        (*state).x10 = value.clone();
    }
    else if true {
        (*state).x11 = value.clone();
    }
    else if true {
        (*state).x12 = value.clone();
    }
    else if true {
        (*state).x13 = value.clone();
    }
    else if true {
        (*state).x14 = value.clone();
    }
    else if true {
        (*state).x15 = value.clone();
    }
    else if true {
        (*state).x16 = value.clone();
    }
    else if true {
        (*state).x17 = value.clone();
    }
    else if true {
        (*state).x18 = value.clone();
    }
    else if true {
        (*state).x19 = value.clone();
    }
    else if true {
        (*state).x20 = value.clone();
    }
    else if true {
        (*state).x21 = value.clone();
    }
    else if true {
        (*state).x22 = value.clone();
    }
    else if true {
        (*state).x23 = value.clone();
    }
    else if true {
        (*state).x24 = value.clone();
    }
    else if true {
        (*state).x25 = value.clone();
    }
    else if true {
        (*state).x26 = value.clone();
    }
    else if true {
        (*state).x27 = value.clone();
    }
    else if true {
        (*state).x28 = value.clone();
    }
    else if true {
        (*state).x29 = value.clone();
    }
    else if true {
        (*state).x30 = value.clone();
    }
    else if true {
        (*state).x31 = value.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn register_read_BV32(state: *mut SystemState, register_id: Rc<RefCell<dyn ActiveAst>>, stdlib: &mut ScfiaStdlib) -> Rc<RefCell<dyn ActiveAst>> {
    if true {
        return (*state).x0.clone();
    }
    else if true {
        return (*state).x1.clone();
    }
    else if true {
        return (*state).x2.clone();
    }
    else if true {
        return (*state).x3.clone();
    }
    else if true {
        return (*state).x4.clone();
    }
    else if true {
        return (*state).x5.clone();
    }
    else if true {
        return (*state).x6.clone();
    }
    else if true {
        return (*state).x7.clone();
    }
    else if true {
        return (*state).x8.clone();
    }
    else if true {
        return (*state).x9.clone();
    }
    else if true {
        return (*state).x10.clone();
    }
    else if true {
        return (*state).x11.clone();
    }
    else if true {
        return (*state).x12.clone();
    }
    else if true {
        return (*state).x13.clone();
    }
    else if true {
        return (*state).x14.clone();
    }
    else if true {
        return (*state).x15.clone();
    }
    else if true {
        return (*state).x16.clone();
    }
    else if true {
        return (*state).x17.clone();
    }
    else if true {
        return (*state).x18.clone();
    }
    else if true {
        return (*state).x19.clone();
    }
    else if true {
        return (*state).x20.clone();
    }
    else if true {
        return (*state).x21.clone();
    }
    else if true {
        return (*state).x22.clone();
    }
    else if true {
        return (*state).x23.clone();
    }
    else if true {
        return (*state).x24.clone();
    }
    else if true {
        return (*state).x25.clone();
    }
    else if true {
        return (*state).x26.clone();
    }
    else if true {
        return (*state).x27.clone();
    }
    else if true {
        return (*state).x28.clone();
    }
    else if true {
        return (*state).x29.clone();
    }
    else if true {
        return (*state).x30.clone();
    }
    else if true {
        return (*state).x31.clone();
    }
    else {
        unimplemented!();
    }
}

pub unsafe fn execute_add32(state: *mut SystemState, destination_id: Rc<RefCell<dyn ActiveAst>>, source1_id: Rc<RefCell<dyn ActiveAst>>, source2_id: Rc<RefCell<dyn ActiveAst>>, stdlib: &mut ScfiaStdlib) {
    let s1: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, source1_id.clone(), stdlib);
    let s2: Rc<RefCell<dyn ActiveAst>> = register_read_BV32(state, source2_id.clone(), stdlib);
    let sum: Rc<RefCell<dyn ActiveAst>> = Rc::new(RefCell::new(BVAddExpression::new(s1.clone(), s2.clone(), stdlib)));
    register_write_BV32(state, destination_id.clone(), sum.clone(), stdlib);
}
