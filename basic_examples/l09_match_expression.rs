


fn main(){

    let x: u8 = 7;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Other"),
    }

    let c: char = 'A';
    let n = match c {
        'A' => 10,
        'B' => 11,
        'D' => 12,
        _ => -1,
    };
    println!("{n}");
}