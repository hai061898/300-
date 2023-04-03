// Input: ("i love eating burger") . target ("burg")
// Output: 4

fn find_prefix_word_position(sentence: String, search_word: String) -> Option<usize> {
    for (i, word) in sentence.split(' ').enumerate() {
        if word.starts_with(&search_word) {
            return Some(i+1);
        }
    }
    None
}

fn main() {
    let sentence = String::from("i love eating burger");
    let search_word = String::from("burg");

    match find_prefix_word_position(sentence, search_word) {
        Some(pos) => println!("The word starts at position {}", pos), // =4
        None => println!("The word is not found"),
    }
}
