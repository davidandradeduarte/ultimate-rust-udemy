fn main() {
    // a simple closure
    let add = |x, y| x + y;
    println!("{}", add(1, 2));

    // a lot of closures are available in the standard library
    let numbers = vec![1, 2, 3, 4, 5];
    numbers
        .iter()
        .map(|x| x + 1)
        .filter(|x| *x % 2 == 0)
        .for_each(|x| println!("{}", x));
}
