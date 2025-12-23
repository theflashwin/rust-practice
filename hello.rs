use std::fmt;

/*
Should be unprintable
*/
struct UnprintableUser {
    id: u64,
    name: String, 
    active: bool
}

/*
Should be printable
*/
#[derive(Debug)]
struct PrintableUser {
    id: u64,
    name: String,
    active: bool
}

impl fmt::Display for PrintableUser {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} (id: {}) is currently {} ", self.name, self.id, self.active)
    }
}

fn main() {

    let peter = UnprintableUser {
        id: 1,
        name: "Peter".to_string(),
        active: true,
    };

    let lois = PrintableUser {
        id: 2,
        name: "Lois".to_string(),
        active: false,
    };

    // println!("Peter: {:?}", peter);
    println!("Lois: {}", lois); // display
    println!("Lois: {:?}", lois); // debug

}