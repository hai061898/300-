// Longest Common Prefix

// Write a function to find the longest common prefix string amongst an array of strings.

// If there is no common prefix, return an empty string "".

// Example 1:

// Input: ["flower","flow","flight"]

// Output: "fl"

// Example 2:

// Input: ["dog","racecar","car"]

// Output: ""

// Explanation:

// There is no common prefix among the input strings.



fn longest_common_prefix(strs: Vec<String>) -> String {
    // Kiểm tra nếu danh sách rỗng, trả về chuỗi rỗng
    if strs.is_empty() {
        return String::new();
    }
    
    // Lấy từ đầu tiên trong danh sách để so sánh
    let first_word = &strs[0];
    // Khởi tạo chuỗi tiền tố chung
    let mut prefix = String::new();
    
    // Duyệt qua từng ký tự trong từ đầu tiên
    for (i, c) in first_word.chars().enumerate() {
        // Duyệt qua các từ từ thứ hai trở đi trong danh sách
        for word in strs.iter().skip(1) {
            // Kiểm tra ký tự tại vị trí i của từ hiện tại
            if let Some(ch) = word.chars().nth(i) {
                // Nếu không khớp, trả về tiền tố chung
                if ch != c {
                    return prefix;
                }
            } else {
                // Nếu từ hiện tại không có ký tự tại vị trí i, trả về tiền tố chung
                return prefix;
            }
        }
        
        // Thêm ký tự c vào tiền tố chung
        prefix.push(c);
    }
    
    prefix
}

fn main() {
    // Nhập danh sách các từ
    let input = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight")
    ];
    
    // Tìm tiền tố chung dài nhất
    let result = longest_common_prefix(input);
    
    // Hiển thị kết quả
    println!("The longest common prefix is: {}", result);
}
