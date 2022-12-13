#![allow(warnings)]

use rust_examples::advanced::fns::{add_one, do_twice, Status};

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    // transform numbers to string using closures
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // transform numbers to string using fns
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
