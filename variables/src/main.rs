fn main() {
    let a = 1; // compile infers type of x to be i32
    let mut b: i32 = 2; // explicit type annotation
    let (c, d) = (3, 4); // tuple pattern
    // a = 2; // compile error: cannot assign to a (immutable)
    b = 3; // ok
    println!("{} {} {} {}", a, b, c, d);
}
