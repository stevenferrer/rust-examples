#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn new() {
        let mut scores = HashMap::new();

        scores.insert("alice", 10);
        scores.insert("bob", 6);
        scores.insert("carol", 9);
        scores.insert("dean", 7);

        for (name, score) in &scores {
            println!("{name}: \"{score}\"");
        }

        if !scores.contains_key("shaw") {
            println!("shaw not found")
        }

        let students = ["alice", "ben", "oliver"];
        for &student in &students {
            match scores.get(student) {
                Some(score) => println!("{student} scored {score}"),
                None => println!("{student} didn't took the test."),
            }
        }
    }

    #[test]
    fn from() {
        let search_engines = HashMap::from([
            ("solr", 10.0),
            ("elastic", 8.5),
            ("algolia", 9.5),
            ("quickwit", 8.7),
        ]);

        for (search_engine, rating) in &search_engines {
            println!("{search_engine}: \"{rating}\"");
        }
    }
}
