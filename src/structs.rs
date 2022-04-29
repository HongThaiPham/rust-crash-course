struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

struct TupleColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // This is a constructor fn
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };
    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut tc = TupleColor(255, 0, 0);

    tc.0 = 199;
    println!("TupleColor: {} {} {}", tc.0, tc.1, tc.2);

    let mut person = Person::new("Leo", "Pham");
    println!("{} {}", person.first_name, person.last_name);

    person.set_last_name("Dicarlo");

    println!("{}", person.full_name());

    println!("Person to tuple {:?} ", person.to_tuple());
}
