struct Solution;
// Tìm chuỗi palindrome dài nhất trong một chuỗi đầu vào.
use std::iter::FromIterator;

impl Solution {
    fn longest_palindrome(s: String) -> String {
        // Khởi tạo độ dài của chuỗi đầu vào
        let n = s.len();
        // Nếu độ dài của chuỗi đầu vào bằng 0, trả về chuỗi rỗng
        if n == 0 {
            return "".to_string();
        }
        // Chuyển đổi chuỗi đầu vào thành vector các ký tự
        let s: Vec<char> = s.chars().collect();
        // Khởi tạo biến start và end ban đầu bằng 0
        let mut start = 0;
        let mut end = 0;
        // Duyệt qua từng ký tự trong chuỗi đầu vào
        for i in 0..n {
            // Khởi tạo biến left và right bằng giá trị của biến i
            let mut left = i;
            let mut right = i;
            // Tìm i vị trí đối xứng
            while right + 1 < n && s[right + 1] == s[left] {
                right += 1;
            }
            // Tìm ký tự giống nhau bên trái và phải để tạo chuỗi palindrome
            while left > 0 && right < n - 1 && s[left - 1] == s[right + 1] {
                left -= 1;
                right += 1;
            }
            // Nếu độ dài của chuỗi palindrome mới tìm được lớn hơn độ dài của chuỗi palindrome hiện tại, cập nhật giá trị của start và end
            if right - left > end - start {
                start = left;
                end = right;
            }
        }
        // Trả về chuỗi palindrome dài nhất bằng phương thức from_iter() và lấy một mảng con từ s từ vị trí start đến end.
        String::from_iter(&s[start..=end])
    }
}

// Đoạn code kiểm tra hàm longest_palindrome
#[test]
fn test() {
    let s = "babad".to_string();
    let res = "bab".to_string();
    assert_eq!(Solution::longest_palindrome(s), res);
}