



fn main() {

    // Signed integers
    let a: i8 = 127; // 8-bit signed integer. −(2^7) to 2^7 − 1 == -128 to 127
    let b: i16 = 32_767; // 16-bit signed integer. −(2^15) to 2^15 − 1 == -32_768 to 32_767
    let c: i32 = 2_147_483_647; // 32-bit signed integer. −(2^31) to 2^31 − 1 == -2_147_483_648 to 2_147_483_647
    let d: i64 = 0; // 64-bit signed integer. -(2^63) to 2^63 − 1
    let e: i128 = 0; // 128-bit signed integer. -(2^127) to 2^127 −1
    let f: isize = 0; // Platform-dependent signed integer. Same as i32 on 32-bit systems and i64 on 64-bit systems
    
    // Unsigned integers
    let g: u8 = 255; // 8-bit unsigned integer. 0 to 2^8 − 1 == 0 to 255
    let h: u16 = 65_535; // 16-bit unsigned integer. 0 to 2^16 − 1 == 0 to 65_535
    let i: u32 = 4_294_967_295; // 32-bit unsigned integer. 0 to 2^32 − 1 == 0 to 4_294_967_295
    let j: u64 = 0; // 64-bit unsigned integer. 0 to 2^64 − 1
    let k: u128 = 0; // 128-bit unsigned integer. 0 to 2^128 − 1
    let l: usize = 0; // Platform-dependent unsigned integer. Same as u32 on 32-bit systems and u64 on 64-bit systems

    // Floating-point numbers
    let m: f32 = 3.14; // 32-bit floating-point number
    let n: f64 = 2.718281828459045; // 64-bit floating-point number

    // Unicode character
    let o: char = 'A'; // A single Unicode character

    // Boolean
    let p: bool = true; // Boolean value, can be true or false

    // Print the values
    println!("Signed integers: i8 = {}, i16 = {}, i32 = {}, i64 = {}, i128 = {}, isize = {}",
             a, b, c, d, e, f);
    println!("Unsigned integers: u8 = {}, u16 = {}, u32 = {}, u64 = {}, u128 = {}, usize = {}",
             g, h, i, j, k, l);
    println!("Floating-point numbers: f32 = {}, f64 = {}",
             m, n);
    println!("Unicode character: char = {}", o);
    println!("Boolean: bool = {}", p);
}