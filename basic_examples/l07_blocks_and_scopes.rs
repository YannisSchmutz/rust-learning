



fn main(){

    let x: i8 = 10;
    let y = {
        let z: i8 = 15;
        dbg!(x);
        // dbg!(y); // ERROR: cannot find value `y` in this scope
        dbg!(z);
        z - x
    };
    println!("x: {x}, y: {y}");
    // println!("z: {z}")  // ERROR: cannot find value `z` in this scope
}