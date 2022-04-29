pub fn run() {
    let x = 1;
    let y = 3.5;

    // Explicit type
    let z: i64 = 3423;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean

    let is_active: bool = false;

    println!("is_active = {}", is_active);

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    println!("10 is_greater 5 = {}", is_greater);

    println!("{:?}", (x, y, z, is_active, is_greater));

    // Char
    let a1 = 'a';
    let face = '\u{1F600}';

    println!("{:?}", (a1, face));
}
