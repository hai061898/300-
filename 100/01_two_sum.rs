// Đề bài:

// Cho một mảng nums chứa các số nguyên.
// Cho một số nguyên target.
// Tìm các chỉ số của hai phần tử trong mảng nums có tổng bằng target.

use std::collections::HashMap;

impl Solution {
    // Hàm giải bài toán "Two Sum"
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Khởi tạo hashmap để lưu trữ giá trị của các phần tử trong mảng
        let mut hm: HashMap<i32, i32> = HashMap::new();
        // Duyệt từng phần tử trong mảng nums
        for (i, &num) in nums.iter().enumerate() {
            // Tìm giá trị j tương ứng với phần tử cần tìm để tổng bằng target
            if let Some(&j) = hm.get(&(target - num)) {
                // Nếu tìm thấy, trả về vector chứa chỉ số của hai phần tử đó
                return vec![j, i as i32];
            } else {
                // Nếu không tìm thấy, thêm giá trị hiện tại vào hashmap
                hm.insert(num, i as i32);
            }
        }
        // Nếu không có cặp nào được tìm thấy, trả về vector rỗng
        vec![]
    }
}

// Đoạn code kiểm tra hàm giải bài toán "Two Sum"
#[test]
fn test() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    assert_eq!(Solution::two_sum(nums, target), vec![0, 1]);
}
