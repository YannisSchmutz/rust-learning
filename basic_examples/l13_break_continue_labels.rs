
fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i > 5 {
            break;
        }
        if i % 2 == 0 {
            continue;
        }
        dbg!(i);
    }

    println!("");
    // Labels to break out of NESTED for loops
    let target: i8 = 5;
    let a = [[1,2], [3,4], [5,6]];
    'outer_loop: for i in 0..=2 {
        for j in 0..=1 {
            if a[i][j] == target {
                break 'outer_loop;  // Breaks out of BOTH (the outer) loops(s)
            }
            dbg!(a[i][j]);
        }

    }

}