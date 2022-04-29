pub fn run() {
    let numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    // Get single value
    println!("{}", numbers[0]);
    println!("{}", numbers[4]);

    let mut m_numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // Re-assign value
    m_numbers[0] = 20;
    println!("{}", m_numbers[0]);

    println!("Array length: {}", m_numbers.len());

    // Vector are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&m_numbers));

    // Get slice
    let slice: &[i32] = &m_numbers[0..2];
    println!("Slice: {:?}", slice);

    // Push
    m_numbers.push(6);
    m_numbers.push(7);

    println!("{:?}", m_numbers);

    // Pop
    m_numbers.pop();

    println!("{:?}", m_numbers);

    // Loop through vector values
    for x in &m_numbers {
        println!("{}", x);
    }

    for x in m_numbers.iter() {
        println!("{}", x);
    }

    // Loop & mutate values
    for x in m_numbers.iter_mut() {
        *x *= 2;
    }
    println!("{:?}", m_numbers);
}
