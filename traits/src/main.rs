const DOG_NAME: &'static str = "Dog";
const DUCK_NAME: &'static str = "Duck";

trait Animal {
    fn new(num_legs: u8) -> Self;
    fn get_name(&self) -> String;
    fn num_legs(&self) -> u8;
    // default implementation
    fn print_name_in_uppercase(&self) {
        println!("{}", self.get_name().to_uppercase());
    }
}

struct Dog {
    name: String,
    num_legs: u8,
}

// Dog implements Animal trait
impl Animal for Dog {
    fn new(legs: u8) -> Self {
        Self {
            name: DOG_NAME.to_string(),
            num_legs: legs,
        }
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn num_legs(&self) -> u8 {
        self.num_legs
    }
    // not need to implement `print_name_in_uppercase`
    // altough we can override the default implementation
    fn print_name_in_uppercase(&self) {
        println!("{} woof woof", self.get_name().to_uppercase());
    }
}

struct Duck {
    name: String,
    num_legs: u8,
}

// Duck doesn't implement Animal trait
impl Duck {
    fn new(legs: u8) -> Self {
        Self {
            name: DUCK_NAME.to_string(),
            num_legs: legs,
        }
    }
}

fn main() {
    let dog = Dog::new(4);
    print_animal_num_legs(&dog);
    let duck = Duck::new(2);
    // print_animal_num_legs(&duck); // compile error (Duck doesn't implement Animal trait)
    dog.print_name_in_uppercase();
}

fn print_animal_num_legs<T: Animal>(animal: &T) {
    println!("{} has {} legs", animal.get_name(), animal.num_legs());
}
