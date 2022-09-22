// Casting types: Enables you to cast one type into another

pub fn run() {
    let int_u8: u8 = 5;
    let int2_u8: u8 = 4;
    let int3_u32: u32 = (int_u8 as u32) + (int2_u8 as u32);

    println!("u32 (casting int_u8 + in2_u8) is {}", int3_u32);
}
