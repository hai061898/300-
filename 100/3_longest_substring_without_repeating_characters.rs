// Đề bài: Cho một chuỗi s bao gồm các ký tự, tìm chiều dài của chuỗi con không có ký tự lặp lại dài nhất trong chuỗi s.
use std::collections::HashMap;

impl Solution {
    fn length_of_longest_substring(s: String) -> i32 {
        let mut hm: HashMap<char, usize> = HashMap::new(); // Khởi tạo một hashmap để lưu trữ thông tin ký tự và vị trí xuất hiện
        let mut max: usize = 0; // Biến max để lưu giá trị độ dài của chuỗi con dài nhất
        let mut l: usize = 0; // Biến l để lưu vị trí bắt đầu của chuỗi con hiện tại
        for (r, c) in s.char_indices() { // Duyệt qua từng ký tự của chuỗi s, r là vị trí của ký tự và c là ký tự đó
            if let Some(end) = hm.insert(c, r) { // Nếu ký tự đã có trong hashmap, lấy ra vị trí cuối cùng của ký tự đó, cập nhật lại vị trí mới nhất vào hashmap
                l = usize::max(l, end + 1); // Cập nhật lại vị trí bắt đầu của chuỗi con hiện tại, sao cho chuỗi không chứa ký tự trùng lặp
            }
            max = usize::max(r - l + 1, max); // Cập nhật độ dài chuỗi con dài nhất tìm được
        }
        max as i32 // Trả về kết quả với kiểu dữ liệu yêu cầu là i32
    }
}

#[test]
fn test() {
    let s = " ".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "abba".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 2);
    let s = "abcabcbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
    let s = "bbbb".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 1);
    let s = "pwwkew".to_string();
    assert_eq!(Solution::length_of_longest_substring(s), 3);
}
