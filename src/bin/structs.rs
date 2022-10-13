#[derive(Debug)]
struct Rect {
    width: u32,
    height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rect) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rect {
        Rect {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rect {
        width: 30,
        height: 50,
    };

    println!("rect1: {:?}, area: {}", rect1, rect1.area());

    let scale = 2;
    let rect2 = Rect {
        width: dbg!(10 * scale),
        height: 45,
    };

    dbg!(&rect2);

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));

    let square = Rect::square(10);
    println!("square: {:?}", square);
}
