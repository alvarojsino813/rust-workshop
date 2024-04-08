enum Season {
    Summer(f64),
    Winter(f64),
    Autumn(String, f64),
}

fn main() {

    let season = Season::Autumn(String::from("Halloween!"), 18.43);

    match season {
        Season::Summer(t) => println!("Summer: {}", t),
        Season::Winter(t) => println!("Winter: {}", t),
        Season::Autumn(s, t) => {
            println!("Other season: {}", t);
            println!("Note: {}", s);
        }
    }
}

// Pattern matching
