pub fn run() {
    let name = "Leo";
    let mut age = 31;
    println!("My name is {} and i'm {}", name, age);
    age = age + 1;
    println!("My name is {} and next year i'm {}", name, age);

    // Define constant
    const ID: i32 = 1;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Leo", 31);
    println!("{} is {}", my_name, my_age);
}
