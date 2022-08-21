fn main() {
    let b = 0b10; // integer literal with binary base
    let c = 0o10; // integer literal with octal base
    let d = 0x10; // integer literal with hexadecimal base
    let e = 1_000_000; // integer literal with underscores (readability only - ignored by compiler)
    println!("{} {} {} {}", b, c, d, e);

    // numeric literals with type specified
    let f: f64 = 0.1;
    // vs
    let g = 0.1f32;
    // or
    let h = 0.1_f32; // with an underscore to separate the type from the value
    
    println!("{} {} {}", f, g, h);

    // boleans can be casted to integers (preferably u8)
    let i = true;
    println!("{} {}", i, i as u8);
}
