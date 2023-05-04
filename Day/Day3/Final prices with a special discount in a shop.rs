// Input: prices = [8,4,6,2,3]
// Output: [4,2,4,2,3]

fn final_prices(prices: Vec<i32>) -> Vec<i32> {
    let n = prices.len();
    let mut res = prices.clone();
    for i in 0..n {
        for j in i+1..n {
            if prices[j] <= prices[i] {
                res[i] -= prices[j];
                break;
            }
        }
    }
    res
}

fn main() {
    let prices = vec![8, 4, 6, 2, 3];
    let result = final_prices(prices);
    println!("{:?}", result);
}