use super::instruction_name::InstructionName;

#[derive(Debug)]
pub struct Instruction {
    pub value: InstructionName,
    pub save_register: bool,
    pub condition: ConditionCode,
}

impl Instruction {
    pub fn new(
        operation: &str,
        save_reg: Option<&str>,
        condition: Option<&str>,
    ) -> Option<Instruction> {
        let value = InstructionName::from_name(operation)?;

        let save_register = matches!(save_reg, Some("s"));

        let condition = match condition {
            Some(c) => ConditionCode::from_name(c)?,
            None => ConditionCode::Al,
        };

        Some(Instruction {
            value,
            save_register,
            condition,
        })
    }
}

#[derive(Debug)]
pub enum ConditionCode {
    Eq, // Equal
    Ne, // Not equal
    Cs, // Carry set, Unsigned higher or same
    Hs, // Carry set, Unsigned higher or same
    Cc, // Carry clear, Unsigned lower
    Lo, // Carry clear, Unsigned lower
    Mi, // Minus, Negative
    Pl, // Plus, Positive or zero
    Vs, // Overflow
    Vc, // No overflow
    Hi, // Unsigned higher
    Ls, // Unsigned lower or same
    Ge, // Signed greater or equal
    Lt, // Signed less
    Gt, // Signed greater
    Le, // Signed less or equal
    Al, // Always
}

impl ConditionCode {
    pub fn from_name(s: &str) -> Option<Self> {
        match s {
            "eq" => Some(Self::Eq),
            "ne" => Some(Self::Ne),
            "cs" => Some(Self::Cs),
            "hs" => Some(Self::Hs),
            "cc" => Some(Self::Cc),
            "lo" => Some(Self::Lo),
            "mi" => Some(Self::Mi),
            "pl" => Some(Self::Pl),
            "vs" => Some(Self::Vs),
            "vc" => Some(Self::Vc),
            "hi" => Some(Self::Hi),
            "ls" => Some(Self::Ls),
            "ge" => Some(Self::Ge),
            "lt" => Some(Self::Lt),
            "gt" => Some(Self::Gt),
            "le" => Some(Self::Le),
            "al" => Some(Self::Al),
            "" => Some(Self::Al),
            _ => None,
        }
    }
}
