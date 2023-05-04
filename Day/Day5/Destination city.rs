// Input: [["London","New York"],["New York","Lima"],["Lima","Sao Paulo"]]
// Output: "Sao Paulo"

// "London" -> "New York" -> "Lima" -> "Sao Paulo"
use std::collections::HashMap;

fn dest_city(paths: Vec<Vec<String>>) -> String {
    let mut map = std::collections::HashMap::new();
    for path in &paths {
        map.insert(&path[0], &path[1]);
    }
    let mut current_city = &paths[0][0];
    while map.contains_key(current_city) {
        current_city = map.get(current_city).unwrap();
    }
    current_city.to_string()
}

fn main() {
    let paths = vec![
        vec!["London", "New York"],
        vec!["New York", "Lima"],
        vec!["Lima", "Sao Paulo"],
    ];
    let result = dest_city(paths);
    println!("{}", result);
}







