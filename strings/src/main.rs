fn main() {
    // borrowed string (&str)
    let s = "hello";

    // iterate over unicode characters
    for c in s.chars() {
        println!("{}", c);
    }
    // iterate over bytes
    for c in s.bytes() {
        println!("{}", c);
    }

    // String
    let s = String::from("hello");
    println!("{}", s);

    // or to_string
    let s = "hello".to_string();
    println!("{}", s);
}
