fn main() {


    let name = String::from("Álvaro");

    for letter in name.chars() {
        println!("Char: {}", letter);
    }

    name.chars().for_each(|letter| println!("Same but functional: {}", letter));
}

// Iterators
