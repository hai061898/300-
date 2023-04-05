// Input: s = "covid2019"
// Output: "c2o0v1i9d"

fn reformat(s: String) -> String {
    let mut chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    
    let mut letters = Vec::new();
    let mut digits = Vec::new();
    
    for c in chars {
        if c.is_digit(10) {
            digits.push(c);
        } else {
            letters.push(c);
        }
    }
    
    if (letters.len() as i32 - digits.len() as i32).abs() > 1 {
        return "".to_string();
    }
    
    let mut result = String::new();
    let mut i = 0;
    let mut j = 0;
    
    if letters.len() > digits.len() {
        result.push(letters[i]);
        i += 1;
    } else if letters.len() < digits.len() {
        result.push(digits[j]);
        j += 1;
    }
    
    while i < letters.len() && j < digits.len() {
        if letters.len() > digits.len() {
            result.push(digits[j]);
            result.push(letters[i]);
        } else {
            result.push(letters[i]);
            result.push(digits[j]);
        }
        i += 1;
        j += 1;
    }
    // if i < letters.len() {
    //     result.push(letters[i]);
    // }
    
    // if j < digits.len() {
    //     result.push(digits[j]);
    // } 
    
    result
}
fn main() {
    let s1 = String::from("a0b1c2");
    let s2 = String::from("leetcode");
    let s3 = String::from("1229857369");
    let s4 = String::from("covid2019");

    println!("{}", reformat(s1));
    println!("{}", reformat(s2));
    println!("{}", reformat(s3));
    println!("{}", reformat(s4));
}