const HOURS_IN_A_WEEK: u32 = 24 * 7;

fn main() {
    let x = 1;
    let mut y = 2;

    println!("x = {x}");
    println!("y = {y}");

    {
        let x = x * 5;
        println!("x (inner) = {x}");
    }

    y = 3;

    println!("y (new) = {y}");

    println!("HOURS_IN_A_WEEK = {HOURS_IN_A_WEEK}");
}
