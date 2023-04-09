// Input: words = ["leetcode","et","code"], s = "leetcode"
// Output: ["et","code"]

fn string_matching(words: Vec<String>) -> Vec<String> {
    let mut result = Vec::new();
    
    for i in 0..words.len() {
        for j in 0..words.len() {
            if i != j && words[j].contains(&words[i]) {
                result.push(words[i].clone());
                break;
            }
        }
    }
    
    result
}

fn main() {
    let words = vec!["leetcode","et","code"]
    .iter()
    .map(|s| s.to_string())
    .collect();

    let result = string_matching(words);

    println!("{:?}", result);
}