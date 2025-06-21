// This is a quiz for the following sections:
// - Variables
// - Functions
// - If
//
// Mary is buying apples. The price of an apple is calculated as follows:
// - An apple costs 2 rustbucks.
// - However, if Mary buys more than 40 apples, the price of each apple in the
// entire order is reduced to only 1 rustbuck!

// TODO: Write a function that calculates the price of an order of apples given
// the quantity bought.
// fn calculate_price_of_apples(???) -> ??? { ??? }

fn calculate_price_of_apples(num_apples: u32) -> u32 {
    let cost = if num_apples > 40 { 1 } else { 2 }; // price per apple
    num_apples * cost // total cost
}

fn main(){
    
}

// Don't change the tests!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verify_test() {
        assert_eq!(calculate_price_of_apples(35), 70);  // 35 apples * 2 rustbucks
        assert_eq!(calculate_price_of_apples(40), 80);  // 40 apples * 2 rustbucks
        assert_eq!(calculate_price_of_apples(41), 41);  // 41 apples * 1 rustbuck
        assert_eq!(calculate_price_of_apples(65), 65);  // 65 apples * 1 rustbuck
    }
}