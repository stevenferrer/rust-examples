fn main() {
    let data = "lorem ipsum dolor";
    let s = data.to_string();
    let s = String::from(data);

    let _ = String::from("السلام عليكم");
    let _ = String::from("Dobrý den");
    let _ = String::from("Hello");
    let _ = String::from("שָׁלוֹם");
    let _ = String::from("नमस्ते");
    let _ = String::from("こんにちは");
    let _ = String::from("안녕하세요");
    let _ = String::from("你好");
    let _ = String::from("Olá");
    let _ = String::from("Здравствуйте");
    let _ = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s is {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2);

    let mut s = String::from("lo");
    s.push('l');

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("s3 is {}", s3);

    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s is {}", s);

    for c in hello.chars() {
        println!("c is {}", c);
    }
}
