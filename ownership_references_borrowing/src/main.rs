fn main() {
    let a = String::from("hello");
    let _b = a;
    // println!("{}", a); // compile error
    let s1 = String::from("hello");
    let _s2 = s1.clone(); // clone the stack and heap memory of s1
    println!("{} {:p}", s1, &s1);
    println!("{} {:p}", _s2, &_s2);

    let s = String::from("hello");
    do_stuff(s);
    // println!("{}", s); // compile error: s is moved to do_stuff

    // if we want to keep using s
    // we can make it mutable, change do_stuff to return a String and reassign it to s
    // or we can use a reference
    let s = String::from("hello");
    do_stuff_ref(&s);
    // println!("{}", s);

    // mutable reference
    let mut s = String::from("hello");
    do_stuff_ref_mut(&mut s);
    // println!("{}", s);
}

fn do_stuff(s: String) {
    println!("{} {:p}", s, &s);
}

fn do_stuff_ref(s: &String) {
    println!("{} {:p}", s, &s);
}

fn do_stuff_ref_mut(s: &mut String) {
    s.insert_str(0, "foo"); // auto dereference by the dot operator
    *s = String::from("hello"); // reassign s to a new String. we need to manually dereference it
    println!("{} {:p}", s, &s);
}
