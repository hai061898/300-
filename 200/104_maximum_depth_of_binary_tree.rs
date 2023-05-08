// Đề bài:

// Cho một cây nhị phân, hãy tìm chiều sâu lớn nhất của nó.

// 3             1
// / \
// 9  20         2  
//  /  \
// 15   7        3
// Chiều sâu lớn nhất là 3.

// Định nghĩa cây nhị phân
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    // Hàm khởi tạo nút
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Hàm tính toán chiều sâu lớn nhất
pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    max_depth_helper(root.as_ref())
}

// Hàm đệ quy để tính toán chiều sâu của từng nút
fn max_depth_helper(root: Option<&Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        None => 0, // Nút rỗng có chiều sâu bằng 0
        Some(node) => {
            let left_depth = max_depth_helper(node.borrow().left.as_ref());
            let right_depth = max_depth_helper(node.borrow().right.as_ref());
            1 + left_depth.max(right_depth) // Chiều sâu của nút là 1 cộng với chiều sâu lớn nhất của các nút con
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_depth() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 9,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
            }))),
        })));

        assert_eq!(max_depth(tree), 3);
    }
}
