fn max_score(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect(); // convert string to vector of chars
    let n = s.len();

    // compute prefix sum of 0's and 1's
    let mut zeros: Vec<i32> = vec![0; n];
    let mut ones: Vec<i32> = vec![0; n];
    for i in 0..n {
        if s[i] == '0' {
            zeros[i] = if i == 0 { 0 } else { zeros[i - 1] } + 1;
            ones[i] = if i == 0 { 0 } else { ones[i - 1] };
        } else {
            zeros[i] = if i == 0 { 0 } else { zeros[i - 1] };
            ones[i] = if i == 0 { 0 } else { ones[i - 1] } + 1;
        }
    }

    // compute maximum score
    let mut max_score = 0;
    for i in 0..n-1 {
        let score = zeros[i] + ones[n-1] - ones[i];
        max_score = max_score.max(score);
    }

    return max_score;
}
fn main() {
    let s = "011101".to_string();
    let max_score = max_score(s);
    println!("{}", max_score);
}


