use std::collections::HashMap;
use std::vec::Vec;

fn main() {
    // creating a vector
    let mut v: Vec<i32> = Vec::new();
    // pushing values to a vector
    for i in 1..=10 {
        v.push(i);
    }
    // get
    println!("{:?}", v.get(0));
    // None
    println!("{:?}", v.get(99));
    // pop
    println!("{:?}", v.pop());
    // accessing a vector element by index
    println!("{}", v[0]);
    // println!("{}", v[99]); // panic
    // iterating over a vector
    for i in &v {
        println!("{}", i)
    }
    // using vec! macro
    let v2 = vec![1, 2, 3, 4, 5]; // or vec![1; 5] for [1, 1, 1, 1, 1]
    println!("{:?}", v2);
    // pop on empty vector
    let mut v3: Vec<&str> = vec![];
    println!("{:?}", v3.pop());

    // creating a hashmap
    let mut map: HashMap<u8, bool> = HashMap::new();
    // inserting a key-value pair
    map.insert(1, true);
    // get
    println!("{:?}", map.get(&1));
    // None
    println!("{:?}", map.get(&99));
    // pop(remove)
    println!("{:?}", map.remove(&1));
    // None
    println!("{:?}", map.get(&1));

    // unraping a value
    println!("{}", v.get(0).unwrap());
}
