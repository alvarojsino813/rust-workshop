fn main() {
    let juan : String = String::from("Juan");

    wave(juan);
}

fn wave(person : String) {
    println!("Hello {}", person);
}

// Ownership

// str
// &str
// String 
// &String 
// [u8]
// &[u8]
