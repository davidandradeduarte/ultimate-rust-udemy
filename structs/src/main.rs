struct Person {
    name: String,
    age: u8,
}

impl Person {
    // an associated function (doesn't have a self parameter)
    fn new(name: &str, age: u8) -> Self {
        Self {
            name: name.to_string(),
            age,
        }
    }
    // a method
    fn print(&self) {
        println!("{} is {} years old", self.name, self.age);
    }
    // mutable method
    fn change_age(&mut self, age: u8) {
        self.age = age;
    }
}

fn main() {
    // instantiate a Person struct
    // (all fields are mandatory)
    let p = Person {
        name: "John".to_string(),
        age: 30,
    };

    // access the fields of the struct
    println!("{}, {}", p.name, p.age);

    // instantiate a new Person struct using the `new` function
    let p = Person::new("Mary", 32);

    // print using the `print` method
    p.print();

    // mutable struct
    let mut p = Person::new("David", 27);
    p.age = 28;
    p.print();
    p.change_age(30);
    p.print();
}
