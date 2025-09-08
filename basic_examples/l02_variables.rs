

fn immutable_variables() {
    // Variables are IMMUTABLE by default
    let x: i8 = 5;
    println!("The value of x is: {x}");

    // Try reassigning x -> This would not compile!
    // x = 6; // This will cause a compile-time error because x is immutable
    // println!("The value of x is: {x}");
}

fn mutable_variables() {
    // Use mut to make a variable mutable
    let mut x: i8 = 5;
    println!("The value of x is: {x}");

    // Try reassigning x
    x = 6; // This will cause a compile-time error because x is immutable
    println!("The value of x is: {x}");
}



fn main() {
    immutable_variables();
    mutable_variables();

}