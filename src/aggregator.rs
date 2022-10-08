use std::fmt::Display;

pub trait Summary {
    // Required methods
    // fn summarize(&self) -> String;

    // Optional
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    // Comment-out to see default mpl
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// "impl Trait" syntax and multi-trait bounds
pub fn notify1(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item.summarize());
}

// Trait-bound syntax and multi-trait bounds
pub fn notify2<T: Summary + Display>(item: T) {
    println!("Breaking news: {}", item.summarize());
}

// Trait-bounds with where clause
pub fn notify3<T>(item: &T)
where
    T: Summary + Display,
{
    println!("Breaking news: {}", item.summarize());
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Conditionally implement methods for specific type (e.g. Display + PartialOrd)
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
