fn main() {
    let a: i32;
    if true {
        a = 1;
    } else {
        a = 2;
    }
    println!("{}", a);

    // function inside a function
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
    println!("{}", add(1, 2));

    // closure
    let add_closure = |a, b| a + b; // types can be inferred
                                    // can also have blocks: |a: i32, b: i32| { a + b }
    println!("{}", first_class_functions(add_closure));
}

fn first_class_functions(f: fn(i32, i32) -> i32) -> i32 {
    f(1, 2)
}
