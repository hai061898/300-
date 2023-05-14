// Đề bài: Loại bỏ các dấu ngoặc không hợp lệ khỏi một biểu thức chứa các dấu ngoặc đơn và đôi.

// Input:
// Một chuỗi biểu thức chứa các dấu ngoặc đơn và đôi, ví dụ: "())()"

// Output:
// Một vector chứa các chuỗi biểu thức hợp lệ sau khi loại bỏ các dấu ngoặc không hợp lệ, ví dụ: ["()()"]

use std::collections::{HashSet, VecDeque};

fn is_valid(s: &str) -> bool {
    let mut count = 0;

    for ch in s.chars() {
        if ch == '(' {
            count += 1;  // Tăng đếm khi gặp dấu ngoặc mở
        } else if ch == ')' {
            count -= 1;  // Giảm đếm khi gặp dấu ngoặc đóng
            if count < 0 {
                return false;  // Nếu số dấu ngoặc đóng vượt quá số dấu ngoặc mở, biểu thức không hợp lệ
            }
        }
    }

    count == 0  // Biểu thức hợp lệ khi số dấu ngoặc mở và đóng bằng nhau
}

fn remove_invalid_parentheses(s: String) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: VecDeque<String> = VecDeque::new();

    // Khởi tạo hàng đợi với chuỗi ban đầu
    queue.push_back(s.clone());
    visited.insert(s);

    let mut found = false;

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();

        // Kiểm tra tính hợp lệ của chuỗi hiện tại
        if is_valid(&current) {
            result.push(current.clone());
            found = true;
        }

        // Nếu đã tìm được các chuỗi hợp lệ, bỏ qua các bước tiếp theo
        if found {
            continue;
        }

        // Thử loại bỏ các dấu ngoặc không hợp lệ
        for (i, ch) in current.chars().enumerate() {
            if ch != '(' && ch != ')' {
                continue;
            }

            // Tạo chuỗi mới bằng cách loại bỏ ký tự ch hiện tại
            let new_str = format!("{}{}", &current[..i], &current[i + 1..]);
            if !visited.contains(&new_str) {
                queue.push_back(new_str.clone());
                visited.insert(new_str);
            }
        }
    }

    result
}

fn main() {
    let input = "())()".to_string();
    let result = remove_invalid_parentheses(input);
    println!("Các biểu thức hợp lệ sau khi loại bỏ: {:?}", result);
}
