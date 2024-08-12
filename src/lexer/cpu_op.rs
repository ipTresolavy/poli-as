use crate::token::{
    immediate::Immediate, instruction::Instruction, instruction_name::InstructionName,
};

use super::{
    expression::{
        ls_imm_index::{IndexMode, LoadStoreImmediateExpression},
        ls_multiple::LoadStoreMultipleExpression,
        ls_reg_index::LoadStoreRegisterExpression,
        Expression,
    },
    machine_code_builder::MachineCodeInstruction,
};

#[derive(Debug)]
pub struct CpuOperation {
    pub instruction: Instruction,
    pub expression: Expression,
}

impl CpuOperation {
    pub fn new(instruction: Instruction, expression: Expression) -> Self {
        CpuOperation {
            instruction,
            expression,
        }
    }

    pub fn to_machine_code(&self) -> MachineCodeInstruction {
        let mut code = MachineCodeInstruction::new();
        let condition_mask = self.instruction.condition.to_machine_code();
        code.push_mask((15 << 28) as u32, condition_mask);

        let machine_code = self.gen_machine_code();

        code.push_mask(0x0fffffff, machine_code);

        code
    }

    fn gen_machine_code(&self) -> u32 {
        if is_load_store_multiple(&self.instruction) {
            return CpuOperation::generate_load_store_multiple(
                &self.instruction,
                match &self.expression {
                    Expression::LoadStoreMultiple(expr) => expr,
                    _ => panic!("Expected load store multiple expression"),
                },
            );
        };

        if is_load_store(&self.instruction) {
            return self.generate_load_store();
        };

        if is_proc(&self.instruction) {
            return self.generate_proc();
        };

        if is_bx(&self.instruction) {
            return self.generate_bx();
        };

        if is_branch(&self.instruction) {
            return self.generate_b();
        };

        panic!("Invalid Instruction");
    }

    fn generate_bx(&self) -> u32 {
        let base: u32 = 0x012fff10;

        let reg = match self.expression {
            Expression::Register(ref reg) => reg.to_machine_code(),
            _ => panic!("Expected register"),
        };

        base | reg
    }

    fn generate_b(&self) -> u32 {
        let base: u32 = 0x0e000000;

        let link: u32 = match self.instruction.value {
            InstructionName::BL => 0x01000000,
            InstructionName::B => 0,
            _ => panic!("Expected branch instruction"),
        };

        let base = base | link;

        let imm = match self.expression {
            Expression::Immediate(ref imm) => imm.to_machine_code(),
            _ => panic!("Expected register"),
        };

        base | imm
    }

    fn generate_proc(&self) -> u32 {
        let base: u32 = 0x00000000;
        let save = match self.instruction.save_register {
            true => 1 << 20,
            false => 0,
        };

        let base = base | save;

        let proc_opcode = get_proc_opcode(&self.instruction.value);

        let base = base | (proc_opcode << 21);

        let expression = get_proc_expression(&self.expression);

        base | expression
    }

    fn generate_load_store(&self) -> u32 {
        let mut mask = 1 << 26;

        let istr = match self.instruction.value {
            InstructionName::LDR => 1 << 20,
            InstructionName::STR => 0,
            _ => panic!("Expected load store instruction"),
        };

        mask |= istr;

        let expression = match self.expression {
            Expression::LoadStoreImmediate(ref expr) => CpuOperation::generate_load_store_imm(expr),
            Expression::LoadStoreRegister(ref expr) => CpuOperation::generate_load_store_reg(expr),
            _ => panic!("Expected load store immediate expression"),
        };

        mask |= expression;

        mask
    }

    fn generate_load_store_imm(expr: &LoadStoreImmediateExpression) -> u32 {
        let mut istr: u32 = 0;
        let base: u32 = (expr.base.to_num() as u32) << 16;
        let destination = (expr.destination.to_num() as u32) << 12;

        let index = match expr.index_mode {
            IndexMode::Pre(index) => {
                let mask = 1 << 24;
                if index.write_back {
                    mask | 1 << 21
                } else {
                    mask
                }
            }
            IndexMode::Post => 0,
            IndexMode::None => 0,
        };

        let imm = expr
            .clone()
            .offset
            .unwrap_or(Immediate::new("0x0".to_owned()).unwrap())
            .to_num() as i32;

        let imm_u32: u32 = if imm < 0 {
            imm.unsigned_abs()
        } else {
            istr |= 1 << 23;
            imm as u32
        };

        let imm = imm_u32 & 0xfff;

        istr | base | destination | index | imm
    }

    fn generate_load_store_reg(expr: &LoadStoreRegisterExpression) -> u32 {
        let istr: u32 = 1 << 25;
        let base: u32 = (expr.base.to_num() as u32) << 16;
        let destination = (expr.destination.to_num() as u32) << 12;
        let offset = expr.offset.to_num() as u32;
        let barrel_shifter = expr
            .barrel_shifter
            .map(|bs| bs.to_machine_code())
            .unwrap_or(0);

        let negative = if expr.negative { 0 } else { 1 << 23 };

        let index = match expr.index_mode {
            IndexMode::Pre(index) => {
                let mask = 1 << 24;
                if index.write_back {
                    mask | 1 << 21
                } else {
                    mask
                }
            }
            IndexMode::Post => 0,
            IndexMode::None => 0,
        };

        istr | base | destination | index | barrel_shifter | offset | negative
    }

    fn generate_load_store_multiple(
        instruction: &Instruction,
        expr: &LoadStoreMultipleExpression,
    ) -> u32 {
        let istr: u32 = 1 << 27 | 1 << 21;
        let base: u32 = (expr.base.to_num() as u32) << 16;
        let write_back = if expr.write_back { 1 << 21 } else { 0 };
        let save_spsr = if instruction.save_register {
            1 << 22
        } else {
            0
        };

        let before_or_after = if after_istr(instruction) { 0 } else { 1 << 24 };

        let load = if load_istr(instruction) { 1 << 20 } else { 0 };

        let increment = if increment_istr(instruction) {
            1 << 23
        } else {
            0
        };

        let mut reg_to_transfer = 0;
        for reg in &expr.registers {
            let reg_num = reg.to_num();
            reg_to_transfer |= 1 << reg_num;
        }

        istr | base | write_back | save_spsr | increment | reg_to_transfer | load | before_or_after
    }
}

fn get_proc_opcode(operation: &InstructionName) -> u32 {
    match operation {
        InstructionName::AND => 0,
        InstructionName::EOR => 1,
        InstructionName::SUB => 2,
        InstructionName::RSB => 3,
        InstructionName::ADD => 4,
        InstructionName::ADC => 5,
        InstructionName::SBC => 6,
        InstructionName::RSC => 7,
        InstructionName::TST => 8,
        InstructionName::TEQ => 9,
        InstructionName::CMP => 10,
        InstructionName::CMN => 11,
        InstructionName::ORR => 12,
        InstructionName::MOV => 13,
        InstructionName::BIC => 14,
        InstructionName::MVN => 15,
        _ => panic!("Invalid operation"),
    }
}

fn load_istr(instruction: &Instruction) -> bool {
    use InstructionName::*;
    matches!(
        instruction.value,
        LDMIA | LDMIB | LDMDA | LDMDB | LDR | LDRB | LDRH | LDRSB | LDRSH
    )
}

fn increment_istr(instruction: &Instruction) -> bool {
    matches!(
        instruction.value,
        InstructionName::LDMIA
            | InstructionName::STMIA
            | InstructionName::LDMIB
            | InstructionName::STMIB
    )
}

fn after_istr(instruction: &Instruction) -> bool {
    matches!(
        instruction.value,
        InstructionName::LDMIA
            | InstructionName::STMIA
            | InstructionName::LDMDA
            | InstructionName::STMDA
    )
}

fn is_load_store_multiple(instruction: &Instruction) -> bool {
    use InstructionName::*;
    matches!(
        instruction.value,
        LDMIA | LDMIB | LDMDA | LDMDB | STMIA | STMIB | STMDA | STMDB
    )
}

fn is_branch(instruction: &Instruction) -> bool {
    use InstructionName::*;
    matches!(instruction.value, B | BL)
}

fn is_bx(instruction: &Instruction) -> bool {
    use InstructionName::*;
    matches!(instruction.value, BX | BLX)
}

fn is_proc(instruction: &Instruction) -> bool {
    use InstructionName::*;
    matches!(
        instruction.value,
        AND | EOR
            | SUB
            | RSB
            | ADD
            | ADC
            | SBC
            | RSC
            | TST
            | TEQ
            | CMP
            | CMN
            | ORR
            | MOV
            | BIC
            | MVN
    )
}

fn is_load_store(instruction: &Instruction) -> bool {
    use InstructionName::*;
    matches!(
        instruction.value,
        LDR | LDRB | LDRH | LDRSB | LDRSH | STR | STRB | STRH
    )
}

fn get_proc_expression(expression: &Expression) -> u32 {
    match expression {
        Expression::ThreeRegs(expr) => expr.to_machine_code(),
        Expression::TwoRegs(expr) => expr.to_machine_code(),
        Expression::TwoRegsLiteral(expr) => expr.to_machine_code(),
        Expression::RegLiteral(expr) => expr.to_machine_code(),
        _ => panic!("Invalid expression"),
    }
}
