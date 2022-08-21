fn main() {
    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let leet = ("L", 3, 3, 7.0);
    println!("{:?} {:?}", tup, leet);

    // accessing tuple elements
    // by index
    println!("{} {} {}", tup.0, tup.1, tup.2);
    // destructor syntax
    let (x, y, z) = tup;
    println!("{} {} {}", x, y, z);

    // array
    let arr = [1, 2, 3, 4, 5];
    let arr2 = [3; 5]; // array with 5 elements all initialized to 3
    println!("{:?} {:?}", arr, arr2);
    // array type anotation
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // always needs the size of the array to be specified explicitly
    let b = [3; 50];
    println!("{:?} {:?}", a, b);
}
