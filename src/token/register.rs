#[derive(Debug)]
pub enum RegisterNumbers {
    ZERO,
    ONE,
    TWO,
    THREE,
    FOUR,
    FIVE,
    SIX,
    SEVEN,
    EIGHT,
    NINE,
    TEN,
    ELEVEN,
    TWELVE,
    THIRTEEN,
    FOURTEEN,
    FIFTEEN,
}

impl RegisterNumbers {
    pub fn to_num(&self) -> u8 {
        match self {
            RegisterNumbers::ZERO => 0,
            RegisterNumbers::ONE => 1,
            RegisterNumbers::TWO => 2,
            RegisterNumbers::THREE => 3,
            RegisterNumbers::FOUR => 4,
            RegisterNumbers::FIVE => 5,
            RegisterNumbers::SIX => 6,
            RegisterNumbers::SEVEN => 7,
            RegisterNumbers::EIGHT => 8,
            RegisterNumbers::NINE => 9,
            RegisterNumbers::TEN => 10,
            RegisterNumbers::ELEVEN => 11,
            RegisterNumbers::TWELVE => 12,
            RegisterNumbers::THIRTEEN => 13,
            RegisterNumbers::FOURTEEN => 14,
            RegisterNumbers::FIFTEEN => 15,
        }
    }

    pub fn from_num(num: u32) -> Option<RegisterNumbers> {
        match num {
            0 => Some(RegisterNumbers::ZERO),
            1 => Some(RegisterNumbers::ONE),
            2 => Some(RegisterNumbers::TWO),
            3 => Some(RegisterNumbers::THREE),
            4 => Some(RegisterNumbers::FOUR),
            5 => Some(RegisterNumbers::FIVE),
            6 => Some(RegisterNumbers::SIX),
            7 => Some(RegisterNumbers::SEVEN),
            8 => Some(RegisterNumbers::EIGHT),
            9 => Some(RegisterNumbers::NINE),
            10 => Some(RegisterNumbers::TEN),
            11 => Some(RegisterNumbers::ELEVEN),
            12 => Some(RegisterNumbers::TWELVE),
            13 => Some(RegisterNumbers::THIRTEEN),
            14 => Some(RegisterNumbers::FOURTEEN),
            15 => Some(RegisterNumbers::FIFTEEN),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Register {
    pub register: RegisterNumbers,
}

impl Register {
    pub fn new(register: RegisterNumbers) -> Register {
        Register { register }
    }
}
