fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    for arg in args {
        match arg.as_str() {
            "sum" => sum(),
            "double" => double(),
            _ => count(arg),
        }
    }
}

fn sum() {
    let mut sum = 0;
    for i in 7..=23 {
        sum += i;
    }

    println!("The sum is {}", sum);
}

fn double() {
    let mut count = 0;
    let mut x = 1;

    while x < 500 {
        x *= 2;
        count += 1;
    }

    println!(
        "You can double x {} times until x is larger than 500",
        count
    );
}

fn count(arg: String) {
    let mut count = 0;
    loop {
        print!("{} ", arg);
        count += 1;
        if count == 8 {
            break;
        }
    }
    println!();
}
