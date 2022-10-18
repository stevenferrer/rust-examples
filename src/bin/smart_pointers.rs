use std::mem::drop;
use std::ops::Deref;
use std::rc::Rc;

use rust_examples::list::{List, ListV2};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> MyBox<T> {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let l = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");

    let d = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    drop(d);
    println!("CustomSmartPointer dropped before the end of main.");

    let a = Rc::new(ListV2::Cons(
        5,
        Rc::new(ListV2::Cons(10, Rc::new(ListV2::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ListV2::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ListV2::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a))
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
