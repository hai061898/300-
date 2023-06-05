// Đề bài: Isomorphic Strings

// Cho hai chuỗi không rỗng s và t, hãy xác định xem chúng có là đồng dạng (isomorphic) hay không.

// Hai chuỗi được gọi là đồng dạng nếu có thể thay thế các ký tự trong chuỗi s để tạo ra chuỗi t.

// Mỗi ký tự trong s có thể được ánh xạ đến một ký tự duy nhất trong t, và mỗi ký tự trong t có thể được ánh xạ đến một ký tự duy nhất trong s.

// Ví dụ 1:

// Input: s = "egg", t = "add"

// Output: true

// Ví dụ 2:

// Input: s = "foo", t = "bar"

// Output: false

fn isomorphic_strings(s: String, t: String) -> bool {
    // Kiểm tra nếu độ dài của hai chuỗi không bằng nhau, chúng không thể là đồng dạng
    if s.len() != t.len() {
        return false;
    }

    let mut s_map = std::collections::HashMap::new();
    let mut t_map = std::collections::HashMap::new();

    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    for i in 0..s_chars.len() {
        let s_char = s_chars[i];
        let t_char = t_chars[i];

        // Kiểm tra nếu đã tồn tại một ánh xạ trong cả hai hướng
        if s_map.contains_key(&s_char) && t_map.contains_key(&t_char) {
            if *s_map.get(&s_char).unwrap() != t_char || *t_map.get(&t_char).unwrap() != s_char {
                return false;
            }
        } else if s_map.contains_key(&s_char) || t_map.contains_key(&t_char) {
            // Một ánh xạ chỉ tồn tại trong một hướng
            return false;
        } else {
            // Thêm ánh xạ mới vào cả hai bản đồ
            s_map.insert(s_char, t_char);
            t_map.insert(t_char, s_char);
        }
    }

    true
}

fn main() {
    let s = String::from("egg");
    let t = String::from("add");
    let result = isomorphic_strings(s, t);
    println!("Are the strings isomorphic? {}", result);
}
