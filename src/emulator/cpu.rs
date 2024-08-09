use crate::{
    lexer::{
        cpu_op::CpuOperation,
        expression::{barrel_shifter::BarrelShifterExpression, Expression},
        is_logical_arithmatic_op, is_move_op,
    },
    token::{instruction::Instruction, instruction_name::InstructionName},
    utils::negate_u32,
};

use super::{instruction_mem::InstructionMemory, memory::Heap, regs::CpuRegisters};

pub struct Flags {
    pub negative: bool,
    pub zero: bool,
    pub carry: bool,
    pub overflow: bool,
}

#[derive(Debug)]
pub struct Cpu {
    pub regs: CpuRegisters,
    pub memory: Heap,
    pub instruction_mem: InstructionMemory,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            regs: CpuRegisters::new(),
            memory: Heap::new(),
            instruction_mem: InstructionMemory::new(vec![]),
        }
    }

    pub fn load_program(&mut self, program: Vec<CpuOperation>) {
        self.instruction_mem = InstructionMemory::new(program);
    }

    fn next_instruction(&self) -> &CpuOperation {
        self.instruction_mem.read(self.regs.get(15) as usize)
    }

    pub fn run(&mut self) {
        loop {
            self.cycle();
            println!("{:?}", self.regs);
        }
    }

    fn cycle(&mut self) {
        let op = self.next_instruction();

        let istr = op.instruction;

        let expr = op.expression.clone();

        self.perform_op(istr, expr)
    }

    fn perform_logic_op(&mut self, istr: Instruction, expr: Expression) {
        match expr {
            Expression::ThreeRegs(expr) => {
                let rn = self.regs.get(expr.reg_n.to_num());
                let rm = self.regs.get(expr.reg_m.to_num());
                let barrel_shifter = expr.barrel_shifter;
                let result = calculate_logical_expr(istr.value, rn, rm, barrel_shifter, &self.regs);
                self.regs.set(expr.reg_d.to_num(), result.0);

                if istr.save_register {
                    self.regs.update_flags(result.0, &result.1);
                }
            }
            Expression::TwoRegsLiteral(expr) => {
                let rn = self.regs.get(expr.reg_m.to_num());
                let rm = expr.literal.number as u32;
                let result = calculate_logical_expr(istr.value, rn, rm, None, &self.regs);
                self.regs.set(expr.reg_d.to_num(), result.0);

                if istr.save_register {
                    self.regs.update_flags(result.0, &result.1);
                }
            }
            _ => {
                panic!("Invalid expression")
            }
        };
        self.regs.set(15, self.regs.get(15) + 4);
    }

    fn perform_op(&mut self, istr: Instruction, expr: Expression) {
        if is_logical_arithmatic_op(&istr.value) {
            self.perform_logic_op(istr, expr)
        } else if is_move_op(&istr.value) {
            match expr {
                Expression::TwoRegs(expr) => {
                    let rm = self.regs.get(expr.reg_m.to_num());
                    match istr.value {
                        InstructionName::MOV => {
                            self.regs.set(expr.reg_d.to_num(), rm);
                        }
                        InstructionName::MVN => {
                            self.regs.set(expr.reg_d.to_num(), negate_u32(rm));
                        }
                        InstructionName::CMP => {}
                        _ => {
                            panic!("Invalid instruction")
                        }
                    }
                }
                Expression::RegLiteral(expr) => {
                    let rm = expr.literal.number;
                    match istr.value {
                        InstructionName::MOV => {
                            self.regs.set(expr.register.to_num(), rm);
                        }
                        InstructionName::MVN => {
                            self.regs.set(expr.register.to_num(), -(rm as i32) as u32);
                        }
                        _ => {
                            panic!("Invalid instruction")
                        }
                    }
                }
                _ => {
                    panic!("Invalid expression")
                }
            }
        }
    }
}

fn barrel_shifted_value(
    rm: u32,
    barrel_shifter: BarrelShifterExpression,
    regs: &CpuRegisters,
) -> u32 {
    barrel_shifter.apply(rm, regs)
}

fn calculate_logical_expr(
    instruction_type: InstructionName,
    rn: u32,
    rm: u32,
    barrel_shifter: Option<BarrelShifterExpression>,
    regs: &CpuRegisters,
) -> (u32, Flags) {
    let rm = match barrel_shifter {
        Some(barrel_shifter) => barrel_shifted_value(rm, barrel_shifter, regs),
        None => rm,
    };

    let result = match instruction_type {
        InstructionName::AND => rn & rm,
        InstructionName::EOR => rn ^ rm,
        InstructionName::SUB => rn.wrapping_sub(rm),
        InstructionName::RSB => rm.wrapping_sub(rn),
        InstructionName::ADD => rn.wrapping_add(rm),
        InstructionName::ADC => rn.wrapping_add(rm),
        InstructionName::SBC => rn.wrapping_sub(rm),
        InstructionName::RSC => rm.wrapping_sub(rn),
        InstructionName::TST => rn & rm,
        InstructionName::TEQ => rn ^ rm,
        InstructionName::ORR => rn | rm,
        InstructionName::BIC => rn & !rm,
        _ => panic!("Invalid instruction type"),
    };

    let flags = Flags {
        negative: (result >> 31) == 1,
        zero: result == 0,
        carry: false,
        overflow: false,
    };

    (result, flags)
}
