use std::cmp;

struct Point<T> {
    x: T,
    y: T,
}

// impl type param doesn't have to be the same with struct
impl<T1> Point<T1> {
    fn x(&self) -> &T1 {
        &self.x
    }

    fn y(&self) -> &T1 {
        &self.y
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let p = Point { x: 5.2, y: 10.1 };
    println!("x: {}, y: {}", p.x(), p.y());
}

fn largest<T: cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
