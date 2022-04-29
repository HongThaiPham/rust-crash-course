pub fn run() {
    // Print to console
    println!("Hello, world! I'm Leo Pham");
    // Print to console with formatting
    println!("I'm a {} and I'm {} years old", "Vietnamese", 31);
    println!("{} is a number", 1);

    // Positional arguments
    println!("My name is {0} and i love {1}", "Leo Pham", "Rust");

    // Named arguments
    println!(
        "My name is {name} and i love {language}",
        name = "Leo Pham",
        language = "Javascript"
    );

    // Placeholder traits
    println!("{} of {:b} people know binary, the other half don't", 1, 2);
    println!("Binary is {:b}, Hex is {:x},  Octal is {:o} ", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("{} + {} = {}", 10, 12, 10 + 12);
}
