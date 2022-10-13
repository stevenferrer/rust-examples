#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

fn main() {
    let v1: Vec<i32> = vec![1, 2, 3];

    // need to be mutable if you're calling iter.next
    let mut v1_iter = v1.iter();

    for i in 0..v1.len() {
        let v = v1_iter.next().unwrap();
        println!("v1[{}] = {}", i, v);
    }

    let total: i32 = v1.iter().sum();
    println!("Sum: {}", total);

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2: {:?}", v2);

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);

    println!("in_my_size: {:?}", in_my_size);
}
