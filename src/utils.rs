pub fn negate_u32(value: u32) -> u32 {
    // Convert to i32, negate, then convert back to u32
    let negated = -(value as i32);
    negated as u32
}

pub fn debug_u32(num: u32) {
    for i in (0..32).rev() {
        print!("{}", (num >> i) & 1);
    }
    println!();
}
