


fn main(){

    let foo : i8 = 8;  // 127 max
    let bar : i8 = 100;
    let mut blub: i8 = foo * bar;
    println!("This value would overflow, but rust should detect it or panic: {blub}");
    // OIUTPUT: attempt to compute `8_i8 * 100_i8`, which would overflow

}