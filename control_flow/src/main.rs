fn main() {
    // if's are expressions
    let x = if true { 1 } else { 2 };
    println!("x = {}", x);

    // infinite loop
    // loop {
    //     println!("infinite loop");
    // }

    'outer: loop {
        loop {
            break 'outer; // break out of the outer loop by using its label
        }
    }

    // while loop (syntax sugar for loop/if)
    while false {
        println!("This won't print");
    }

    // there's no do-while loop in rust, but we can use loop with the condition at the end
    let mut i = 0;
    loop {
        i += 1;
        if i == 10 {
            break;
        }
    }

    // for loop (using a range)
    for i in 0..10 {
        // 0 to 9
        println!("{}", i);
    }

    // for loop (using a inclusive range)
    for i in 0..=10 {
        // 0 to 10
        println!("{}", i);
    }

    // for loop over a collection
    for i in [1, 2, 3, 4, 5].iter() {
        println!("{}", i);
    }

    // looping through a tuple and deconstructing it
    for (x, y) in [(1, 2), (3, 4), (5, 6)].iter() {
        println!("{} {}", x, y);
    }
}
