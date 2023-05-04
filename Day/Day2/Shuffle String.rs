// Input: s = "codeleet", indices = [4,5,6,7,0,2,1,3]
// Output: "leetcode"

pub fn restore_string(s: String, indices: Vec<i32>) -> String {
    let mut result = vec![0; s.len()];
    for i in 0..indices.len() {
        let index = indices[i] as usize;
        result[index] = s.as_bytes()[i];
    }
    String::from_utf8(result).unwrap()
}

fn main() {
    let s = "codeleet".to_string();
    let indices = vec![4, 5, 6, 7, 0, 2, 1, 3];
    let shuffled = restore_string(s, indices);
    println!("{}", shuffled);
}