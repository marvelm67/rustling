fn animal_habitat(animal: &str) -> &str {
    // Ensure the identifier is of the same type (f64 in this case)
    let identifier = if animal == "crab" {
        1.0
    } else if animal == "gopher" {
        2.0
    } else if animal == "snake" {
        3.0
    } else {
        -1.0 // Use -1.0 to indicate an unknown animal, which is a valid f64
    };

    // Don't change the expression below!
    if identifier == 1.0 {
        "Beach"
    } else if identifier == 2.0 {
        "Burrow"
    } else if identifier == 3.0 {
        "Desert"
    } else {
        "Unknown"
    }
}

fn main() {
    // You can optionally experiment here.
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        assert_eq!(animal_habitat("gopher"), "Burrow")
    }

    #[test]
    fn snake_lives_in_desert() {
        assert_eq!(animal_habitat("snake"), "Desert")
    }

    #[test]
    fn crab_lives_on_beach() {
        assert_eq!(animal_habitat("crab"), "Beach")
    }

    #[test]
    fn unknown_animal() {
        assert_eq!(animal_habitat("dinosaur"), "Unknown")
    }
}
