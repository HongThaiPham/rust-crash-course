pub fn run() {
    let hello = String::from("Hello");

    println!("{} Leo", hello);
    // Get length
    println!("Length: {}", hello.len());

    let mut hellomut = String::from("Hello");

    hellomut.push('L');

    hellomut.push_str(" Leo!");

    // Capacity in bytes
    println!("Capacity: {}", hellomut.capacity());

    // Check if empty
    println!("Is empty: {}", hellomut.is_empty());

    // Contains
    println!("Contains 'Hello': {}", hellomut.contains("Hello"));

    // Replace
    println!("Replace: {}", hellomut.replace("Hello", "Hi guys"));

    // Loop through string by whitespace
    for word in hellomut.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('s');
    s.push_str("omething good");

    println!("s = {}", s);

    // Assertion testing
    assert_eq!(14, s.len());
    assert_eq!(20, s.capacity());

    println!("{}", hellomut);
}
