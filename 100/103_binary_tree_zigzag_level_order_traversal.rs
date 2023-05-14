// Cho một cây nhị phân, hãy trả về các node theo thứ tự "zigzag level order", tức là trả về các node trên mỗi level từ trái sang phải, rồi từ phải sang trái trên level tiếp theo, và tiếp tục xen kẽ như vậy cho đến khi hết các level của cây.

// 3
// / \
// 9  20
//  /  \
// 15   7


// [
//   [3],
//   [20,9],
//   [15,7]
// ]


use std::cell::RefCell;
use std::rc::Rc;
use std::collections::VecDeque;

// Định nghĩa cấu trúc TreeNode biểu diễn một node trong cây
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Hàm thực hiện việc duyệt cây và trả về các node theo thứ tự "zigzag level order"
fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = vec![]; // Mảng chứa kết quả trả về
    if root.is_none() {
        return res; // Nếu cây rỗng, trả về mảng rỗng
    }
    let mut queue = VecDeque::new(); // Queue chứa các node trong cây
    queue.push_back(root.unwrap()); // Thêm node gốc vào queue
    let mut is_reverse = false; // Biến đổi hướng của duyệt từ trái sang phải hoặc từ phải sang trái

    while !queue.is_empty() { // Duyệt cho đến khi queue rỗng
        let mut level = vec![]; // Mảng chứa các node trên một level
        let size = queue.len(); // Lưu lại số lượng node hiện có trong queue để duyệt từng node
        for _ in 0..size {
            let node = queue.pop_front().unwrap(); // Lấy node đầu tiên ra khỏi queue
            level.push(node.borrow().val); // Thêm giá trị của node vào mảng level
            if node.borrow().left.is_some() { // Nếu node có con trái, thêm node con trái vào queue
                queue.push_back(node.borrow().left.as_ref().unwrap().clone());
            }
            if node.borrow().right.is_some() { // Nếu node có con phải, thêm node con phải vào queue
                queue.push_back(node.borrow().right.as_ref().unwrap().clone());
            }
        }
        if is_reverse { // Nếu hướng duyệt là từ phải sang trái, đảo ngược mảng level
            level.reverse();
        }
        is_reverse = !is_reverse; // Đổi hướng duyệt
        res.push(level); // Thêm mảng level vào kết quả trả về
    }
    res // Trả về kết quả
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zigzag_level_order() {
        // Tạo cây nhị phân đơn giản
        //      3
        //     / \
        //    9  20
        //      /  \
        //     15   7
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        let expected_output = vec![vec![3], vec![20, 9], vec![15, 7]];
        assert_eq!(zigzag_level_order(root), expected_output);
    }
}