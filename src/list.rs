pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};

    #[test]
    fn test_list() {
        let l = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    }
}
