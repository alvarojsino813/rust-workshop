fn main() {
    let mut alvaro : String = String::from("Álvaro");
    let david : String = String::from("David");

    let borrow = &alvaro;
    let borrow_mut = &mut alvaro;

    *borrow_mut = String::from("Jaime");
    
    println!("{}", wave(&alvaro));
    println!("{}", wave(&david));
}

fn wave(x : &String) -> String {
    let wave : String = String::from(format!("Hello {}", x));
    return wave;
}

// Shadowing
// Borrowing rules
