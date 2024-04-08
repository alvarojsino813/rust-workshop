enum Season {
    Summer,
    Winter,
    Autumn,
    Spring
}

fn main() {

    let season = Season::Autumn;

    match season {
        Season::Summer => println!("Summer"),
        Season::Winter => println!("Winter"),
        _ => println!("Other season"),
    }
}

// Pattern matching
