struct Solution {}

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut total = num_bottles;
        let mut empty = num_bottles;
        
        while empty >= num_exchange {
            let new_bottles = empty / num_exchange;
            total += new_bottles;
            empty = new_bottles + empty % num_exchange;
        }
        
        return total;
    }
}

fn main() {
    let num_bottles = 15;
    let num_exchange = 4;
    let num_drinks = Solution::num_water_bottles(num_bottles, num_exchange);
    println!("Number of drinks: {}", num_drinks);
}