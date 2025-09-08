

fn main(){
    let mut loop_val: i8 = 0;

    // The loopstatement works like a while-true-loop.
    // Use it for things like servers which will serve connections forever.
    loop {
        loop_val += 1;
        dbg!(loop_val);
        if loop_val == 100{
            break;
        }
    }

    println!("{loop_val}");
}