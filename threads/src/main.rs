// https://doc.rust-lang.org/rust-by-example/std_misc/threads.html
use std::thread;

const NTHREADS: u32 = 10;

// This is the `main` thread
fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(move || {
            println!("this is thread number {}", i);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3, 4, 5];
//     let mut result = vec![];
//     for i in v {
//         result.push(thread::spawn(move || i + 1));
//     }
//     for i in result {
//         // waits for the thread to finish
//         println!("{:?}", i.join().unwrap());
//     }
// }
