// Cho một cây nhị phân, hãy trả về các giá trị của các nút theo thứ tự từ trên xuống dưới và từ trái sang phải.

// Input:


// 3
// / \
// 9  20
//  /  \
// 15   7

//Output

// [
//   [3],
//   [9,20],
//   [15,7]
// ]

use std::collections::VecDeque;

// Định nghĩa struct cho một nút trong cây nhị phân
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Box<TreeNode>>,
    right: Option<Box<TreeNode>>,
}

// Hàm trả về danh sách giá trị của các nút theo thứ tự từ trên xuống dưới và từ trái sang phải
fn level_order(root: Option<Box<TreeNode>>) -> Vec<Vec<i32>> {
    let mut result = vec![]; // Danh sách kết quả
    let mut queue = VecDeque::new(); // Hàng đợi lưu trữ các nút
    if let Some(node) = root {
        queue.push_back(node); // Thêm nút gốc vào hàng đợi
    }
    while !queue.is_empty() {
        let mut level = vec![]; // Danh sách giá trị các nút cùng cấp
        for _ in 0..queue.len() {
            let node = queue.pop_front().unwrap(); // Lấy nút ở đầu hàng đợi
            level.push(node.val); // Thêm giá trị của nút vào danh sách kết quả
            if let Some(left) = node.left { // Thêm nút con trái vào hàng đợi nếu tồn tại
                queue.push_back(left);
            }
            if let Some(right) = node.right { // Thêm nút con phải vào hàng đợi nếu tồn tại
                queue.push_back(right);
            }
        }
        result.push(level); // Thêm danh sách giá trị các nút cùng cấp vào danh sách kết quả
    }
    result // Trả về danh sách kết quả
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_order() {
        // Tạo cây nhị phân đơn giản
        let root = Some(Box::new(TreeNode {
            val: 3,
            left: Some(Box::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            })),
            right: Some(Box::new(TreeNode {
                val: 20,
                left: Some(Box::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                })),
                right: Some(Box::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                })),
            })),
        }));

        // Tạo kết quả dự kiến
        let expected = vec![vec![3], vec![9, 20], vec![15, 7]];

        // Kiểm tra kết quả của hàm `level_order`
        assert_eq!(level_order(root), expected);
    }
}
