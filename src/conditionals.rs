pub fn run() {
    let age: u8 = 18;
    let check_id: bool = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("You can drink!");
    } else if age >= 18 && check_id {
        println!("You can vote!");
    } else {
        println!("You are too young to do anything!");
    }

    // Shorthand if
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is of age: {}", is_of_age);
}
