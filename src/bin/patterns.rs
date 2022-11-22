fn main() {
    // match arms
    let v = Some(1);
    let x: i32 = match v {
        None => 0,
        Some(v) => v,
    };
    println!("match -> x = {x}");

    // if let
    if let Some(x) = v {
        println!("if let -> x = {x}");
    }

    // while let
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // for loops
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // let statements
    let (x, y, z) = (1, 2, 3);
    println!("x, y, z = {x}, {y}, {z}");

    // function params
    let point = (3, 5);
    print_coordinates(&point);
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
