use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum List<T> {
    Cons(Rc<RefCell<T>>, Rc<List<T>>),
    Nil,
}

#[cfg(test)]
mod tests {
    use super::List::{Cons, Nil};
    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn test_list() {
        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);
    }
}
