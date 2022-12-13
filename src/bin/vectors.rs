#![allow(warnings)]

use std::vec;

fn main() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    let mut v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    for e in &mut v {
        *e += 50;
    }
}
