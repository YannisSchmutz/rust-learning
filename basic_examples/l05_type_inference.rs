
fn print_i8(x: i8){
    println!("i8: {x}");
}

fn print_i16(y: i16) {
    println!("i16: {y}");
}


fn main() {

    let foo = 127;
    let bar = 128;
    print_i8(foo);
    print_i16(bar);

    // assert_eq!(foo, bar);
    // ERROR: expected `i8`, found `i16`
}