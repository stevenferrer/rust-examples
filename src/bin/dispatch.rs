trait Foo {
    fn method(&self) -> String;
}

impl Foo for u8 {
    fn method(&self) -> String {
        format!("u8: {}", *self)
    }
}

impl Foo for String {
    fn method(&self) -> String {
        format!("string: {}", *self)
    }
}

fn do_something<T: Foo>(x: T) -> String {
    x.method()
}

fn do_something2(x: &dyn Foo) -> String {
    x.method()
}

fn main() {
    let x = 5u8;
    let y = "Hello".to_string();

    println!("{}", do_something(x));
    println!("{}", do_something(y.clone()));

    println!("{}", do_something2(&x));
    println!("{}", do_something2(&y));
}
