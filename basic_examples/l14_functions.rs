
fn greathest_common_divider(a: u16, b: u16) -> u16 {
    println!("a: {a}, b: {b}");
    if b > 0{
        greathest_common_divider(b, a % b) // Return value
    } else {
        a // Return value
    }
}


fn main(){
    dbg!(greathest_common_divider(12, 54));
}