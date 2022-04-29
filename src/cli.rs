use std::env;

pub fn run() {
    let agrs: Vec<String> = env::args().collect();
    let command = agrs[1].clone();
    let status = "100%";
    let name = "Leo";

    println!("Args {:?}", agrs);

    println!("Command: {}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}!", status);
    } else {
        println!("That is not a valid command");
    }
}
