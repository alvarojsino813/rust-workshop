pub enum Option<T> {
    None,
    Some(T),
}

fn main() {
    let name = Some(String::from("Álvaro"));        

    match name {
        Some(name) => println!("Hello {}", name),
        None => ()
    }
}

// Pattern matching
// Included in std::option::Option
