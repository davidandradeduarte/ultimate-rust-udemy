#[derive(Debug)]
enum Animal {
    Dog,
    Cat,
    Bird(String), // enums can receive values
}

// we can implement methods for enums
impl Animal {
    fn print(&self) {
        println!("I'm a {:?}", self);
    }
}

fn main() {
    // using enum value
    let dog = Animal::Dog;
    dog.print();

    // checking enum value
    if let Animal::Cat = dog {
        println!("This is a dog");
    }

    // use Option<T> enum to represent an optional (nullable) value
    // let mut animal: Option<Animal> = Some(Animal::Cat);
    let mut animal = Some(Animal::Cat); // type inference for Option<Animal>
    check_animal(animal);
    animal = None;
    check_animal(animal);

    // use Result<T, E> enum to represent a result of an operation that can either succeed or fail
    let result = divide(10, 0);
    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error: {}", error),
    }

    // or using is_ok() and is_err() methods
    let result = divide(10, 2);
    if result.is_ok() || !result.is_err() {
        println!("Result: {}", result.unwrap());
    } else {
        println!("Error: {}", result.unwrap_err());
    }
}

fn divide(dividend: i32, divisor: i32) -> Result<i32, String> {
    if divisor == 0 {
        return Err(String::from("Division by zero"));
    }

    Ok(dividend / divisor)
}

fn check_animal(animal: Option<Animal>) {
    match animal {
        Some(animal) => animal.print(),
        None => println!("No value"),
    }
}
