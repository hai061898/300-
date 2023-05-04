// Input: s = "abbcccddddeeeeedcba"
// Output: 5
// Explanation: Chuỗi con dài nhất là "eeeee".

fn max_power(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut max_len = 1;
    let mut cur_len = 1;

    for i in 1..chars.len() {
        if chars[i] == chars[i - 1] {
            cur_len += 1;
        } else {
            max_len = max_len.max(cur_len);
            cur_len = 1;
        }
    }

    max_len.max(cur_len)
}

fn main() {
    let s = String::from("leetcode");
    let res = max_power(s);
    println!("{}", res);
}


