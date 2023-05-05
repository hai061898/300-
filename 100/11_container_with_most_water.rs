// Cho một mảng các số nguyên dương height, mỗi số biểu thị độ cao của một cột nước. Nhiệm vụ của bạn là tìm hai cột, nơi giữa chúng có thể chứa nhiều nước nhất, và trả về diện tích của nước đó.

struct Solution;

impl Solution {
    // Định nghĩa phương thức tĩnh `max_area` với đầu vào là một vector `height` chứa chiều cao của các cột
    fn max_area(height: Vec<i32>) -> i32 {
        let mut max = 0; // Khởi tạo biến `max` để lưu trữ diện tích nước lớn nhất tìm được
        let mut l = 0; // Đánh dấu vị trí cột nước bên trái
        let mut r = height.len() - 1; // Đánh dấu vị trí cột nước bên phải

        // Vòng lặp while để duyệt qua các cặp cột nước
        while l < r {
            // Tính diện tích nước giữa hai cột được đánh dấu bởi `l` và `r`, và so sánh nó với `max`
            max = i32::max(i32::min(height[l], height[r]) * (r - l) as i32, max);
            
            // Di chuyển con trỏ trái (`l`) hoặc phải (`r`) tùy thuộc vào độ cao của hai cột hiện tại
            if height[l] < height[r] {
                l += 1;
            } else {
                r -= 1;
            }
        }

        // Trả về giá trị của `max`
        max
    }
}

#[test]
fn test() {
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    assert_eq!(Solution::max_area(height), 49);
}
