// Type alias
pub type Kilometers = i32;

// Use type alias to shorten a verbose type
type Thunk = Box<dyn Fn() + Send + 'static>;

// The "never type"
fn bar() -> ! {
    // --snip--
    panic!("never type!")
}
