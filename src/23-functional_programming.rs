fn main() {

    let name = String::from("Juan");

    name.chars()
        .map(|letter| letter.to_uppercase())
        .for_each(|letter| print!("{}", letter));

    println!("");

    name.chars()
        .enumerate()
        .filter(|(_, letter)| letter.is_uppercase())
        .for_each(|(index, letter)| print!("[{}]: {}", index, letter));

    println!("");
}
