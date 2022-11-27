use std::fmt;
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

// Overloading add (+) operator
impl Add for Point {
    // associated type
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// newtype pattern
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

pub trait Pilot {
    fn fly(&self);
}

pub trait Wizard {
    fn fly(&self);
}

pub struct Human;

impl Human {
    pub fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

pub trait Animal {
    fn baby_name() -> String;
}

pub struct Dog;

impl Dog {
    pub fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

// Supertrait
pub trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}

// Newtype pattern
pub struct VecWrapper(Vec<String>);

impl VecWrapper {
    pub fn new(v: Vec<String>) -> VecWrapper {
        VecWrapper(v)
    }
}

impl fmt::Display for VecWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
