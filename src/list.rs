use std::rc::Rc;

pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

pub enum ListV2<T> {
    Cons(T, Rc<ListV2<T>>),
    Nil,
}
