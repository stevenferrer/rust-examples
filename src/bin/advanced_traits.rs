use rust_examples::advanced::traits::{
    Animal, Dog, Human, OutlinePrint, Pilot, Point, VecWrapper, Wizard,
};

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    let person = Human;
    person.fly();

    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();

    println!("A baby dog is called a {}", Dog::baby_name());
    // fully-qualified syntax
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    let p = Point { x: 1, y: 2 };
    p.outline_print();

    let w = VecWrapper::new(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
