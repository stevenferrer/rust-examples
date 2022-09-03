use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::{File, OpenOptions};
    use std::str;

    #[test]
    fn struct_json() {
        let a = Move { x: 1, y: 1 };
        println!("{:?}", a);

        let f1 = OpenOptions::new()
            .write(true)
            .create_new(false)
            .open("testdata/move.json")
            .unwrap();

        serde_json::to_writer(f1, &a).unwrap();

        let f2 = File::open("testdata/move.json").unwrap();

        let b: Move = serde_json::from_reader(f2).unwrap();
        println!("{:?}", b);
    }

    #[test]
    fn struct_ron() {
        let a = Move { x: 1, y: 1 };

        let v1 = serde_json::to_vec(&a).unwrap();
        println!("{:}", str::from_utf8(&v1).unwrap());
        println!("{:}", ron::to_string(&a).unwrap())
    }
}
