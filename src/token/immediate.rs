#[derive(Debug, Clone)]
pub struct Immediate {
    pub value: String,
    pub base: ImmediateBase,
    pub number: u32,
}

#[derive(Debug, Copy, Clone)]
pub enum ImmediateBase {
    HEX,
    DEC,
    OCT,
    BIN,
}

fn determine_base(val: &str) -> Option<ImmediateBase> {
    if val.starts_with("0x") {
        return Some(ImmediateBase::HEX);
    }

    if val.starts_with("0b") {
        return Some(ImmediateBase::BIN);
    }

    if val.starts_with("0o") {
        return Some(ImmediateBase::OCT);
    }

    Some(ImmediateBase::DEC)
}

impl Immediate {
    pub fn new(value: String) -> Option<Immediate> {
        let base = determine_base(&value)?;

        let number: u32 = match base {
            ImmediateBase::HEX => u32::from_str_radix(value.trim_start_matches("0x"), 16).ok()?,
            ImmediateBase::DEC => (value.parse::<i32>().ok()?) as u32,
            ImmediateBase::OCT => u32::from_str_radix(value.trim_start_matches("0o"), 8).ok()?,
            ImmediateBase::BIN => u32::from_str_radix(value.trim_start_matches("0b"), 2).ok()?,
        };

        Some(Immediate {
            value,
            base,
            number,
        })
    }

    pub fn to_num(&self) -> u32 {
        self.number
    }
}
