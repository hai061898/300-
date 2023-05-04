// Đề bài:

// Cho một chuỗi s và một số nguyên dương num_rows, hãy sắp xếp lại các ký tự của s thành một bảng có num_rows hàng, sao cho đọc từ trên xuống dưới và từ trái sang phải, ta được chuỗi mới.

// Ví dụ: Cho chuỗi s = "PAYPALISHIRING" và num_rows = 3, ta cần sắp xếp các ký tự của chuỗi s thành một bảng có 3 hàng như sau:

// P   A   H   N
// A P L S I I G
// Y   I   R


struct Solution;

impl Solution {
    fn convert(s: String, num_rows: i32) -> String {
        // Khởi tạo biến kết quả là một chuỗi rỗng
        let mut res = "".to_string();
        // Chuyển đổi num_rows thành số nguyên không dấu
        let n = num_rows as usize;
        // Nếu num_rows bằng 1, trả về chuỗi đầu vào
        if n == 1 {
            return s;
        }
        // Lấy độ dài của chuỗi đầu vào
        let m = s.len();
        // Khởi tạo một vector các chuỗi rỗng với độ dài là num_rows
        let mut v: Vec<String> = vec!["".to_string(); n];
        // Khởi tạo biến vị trí hàng ban đầu
        let mut row = 0;
        // Khởi tạo biến hướng ban đầu là từ trên xuống dưới
        let mut direction = true;
        // Duyệt từng ký tự trong chuỗi đầu vào
        for j in 0..m {
            // Thêm ký tự vào chuỗi tại hàng hiện tại
            v[row] += &s[j..=j];
            // Nếu đang ở hàng đầu tiên, di chuyển xuống dưới
            if row == 0 {
                direction = true;
                row += 1;
            // Nếu đang ở hàng cuối cùng, di chuyển lên trên
            } else if row == n - 1 {
                direction = false;
                row -= 1;
            // Nếu không phải hàng đầu tiên và cuối cùng, 
            // di chuyển theo hướng hiện tại (lên hoặc xuống)
            } else {
                if direction {
                    row += 1;
                } else {
                    row -= 1;
                }
            }
        }
        // Kết hợp các chuỗi trong vector v để tạo chuỗi kết quả
        for t in v {
            res += &t;
        }
        // Trả về chuỗi kết quả
        res
    }
}

#[test]
fn test() {
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 3;
    let res = "PAHNAPLSIIGYIR".to_string();
    assert_eq!(Solution::convert(s, num_rows), res);
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 4;
    let res = "PINALSIGYAHRPI".to_string();
    assert_eq!(Solution::convert(s, num_rows), res);
    let s = "AB".to_string();
    let num_rows = 1;
    let res = "AB".to_string();
    assert_eq!(Solution::convert(s, num_rows), res);
}
