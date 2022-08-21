trait Bite {
    fn bite(self: &mut Self);
}

#[derive(Debug)]
struct Grapes {
    num_grapes: u8,
}

impl Bite for Grapes {
    fn bite(&mut self) {
        self.num_grapes -= 1
    }
}

fn main() {
    let mut carrot = Carrot {
        percent_left: 100.0,
    };
    carrot.bite();
    println!("I take a bite: {:?}", carrot);

    let mut grapes = Grapes { num_grapes: 100 };
    grapes.bite();
    println!("Eat a grape: {:?}", grapes);

    bunny_nibbles(&mut carrot);
    println!("Bunny nibbles for awhile: {:?}", carrot);

    bunny_nibbles(&mut grapes);
    println!("Still eating grapes: {:?}", grapes);
}

fn bunny_nibbles<T: Bite>(food: &mut T) {
    for _ in 1..=10 {
        food.bite()
    }
}

#[derive(Debug)] // This enables using the debugging format string "{:?}"
struct Carrot {
    percent_left: f32,
}

impl Bite for Carrot {
    fn bite(self: &mut Self) {
        // Eat 20% of the remaining carrot. It may take awhile to eat it all...
        self.percent_left *= 0.8;
    }
}
