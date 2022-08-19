fn main() {
    // scope
    let x = 5;
    {
        let y = 99;
        println!("{} {}", x, y);
    }
    // println!("{} {}", x, y); // compile error: y is not in scope

    // shadow variable value
    let x = 5;
    {
        let x = 99;
        println!("{}", x);
    }
    println!("{}", x); // x is still 5

    // shadow imutable variable
    let mut a = 5;
    let a = a; // a is now immutable

    // shadow variable type (from string to i32)
    let b = "string";
    let b = 1;
}