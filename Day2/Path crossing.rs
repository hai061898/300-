// Input: path = "NESWW"
// Output: true

// Input: path = "NES"
// Output: false

fn is_path_crossing(path: String) -> bool {
    let mut visited = std::collections::HashSet::new();
    let mut x = 0;
    let mut y = 0;
    visited.insert((x, y));
    for c in path.chars() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => (),
        }
        if !visited.insert((x, y)) {
            return true;
        }
    }
    false
}

fn main() {
    let path = String::from("NESWW");
    let result = is_path_crossing(path);
    println!("{}", result);
}