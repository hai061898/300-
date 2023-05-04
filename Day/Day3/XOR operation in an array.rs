
fn xor_operation(n: i32, start: i32) -> i32 {
    let mut result = start;
    for i in 1..n {
        result ^= start + 2 * i;
    }
    result
}

fn main() {
    let n = 5;
    let start = 0;
    let result = xor_operation(n, start);
    println!("{}", result);
}