#[cfg(test)]
mod test {
    #[test]
    fn new() {
        let mut vec = Vec::new();
        vec.push(1);
        vec.push(2);

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);
    }

    #[test]
    fn macro_res() {
        let mut vec = vec![1, 2];

        assert_eq!(vec.len(), 2);
        assert_eq!(vec[0], 1);

        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.len(), 1);

        let vec = vec![0; 5];
        assert_eq!(vec, [0, 0, 0, 0, 0]);
    }
}
