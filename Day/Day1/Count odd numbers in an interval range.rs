// Input: low = 3, high = 7
// Output: 3
// Explanation: Các số lẻ trong đoạn [3,7] là 3, 5 và 7.

fn even_count(low: i32, high: i32) -> i32 {
    let even_count = (high / 2) - (low / 2);
    return even_count;
}

fn count_odds(low: i32, high: i32) -> i32 {
    let even_count = (high / 2) - (low / 2);
    return high - low - even_count + 1;
}

fn main() {
    let low = 3;
    let high = 7;
    let count = count_odds(low, high);
    let count2 = count_odds(low, high);
    println!("The number of odd numbers between {} and {} is {}", low, high, count);
    println!("The number of even numbers between {} and {} is {}", low, high, count2);
}