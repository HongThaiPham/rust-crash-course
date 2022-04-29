pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);
    println!("{}", numbers[4]);

    let mut m_numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    m_numbers[0] = 20;
    println!("{}", m_numbers[0]);

    println!("Array length: {}", m_numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&m_numbers));

    // Get slice
    let slice: &[i32] = &m_numbers[0..2];
    println!("Slice: {:?}", slice);
}
