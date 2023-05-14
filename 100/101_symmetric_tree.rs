// Định nghĩa một nút của cây nhị phân
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

// Sử dụng Rc và RefCell để giúp quản lý sự chia sẻ nút
use std::cell::RefCell;
use std::rc::Rc;

// Hàm chính kiểm tra tính đối xứng của cây
pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    // Gọi đệ quy kiểm tra đối xứng từ nút gốc, với nút trái và phải của nó
    is_mirror(&root, &root)
}

// Hàm kiểm tra tính đối xứng giữa hai nút
fn is_mirror(t1: &Option<Rc<RefCell<TreeNode>>>, t2: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    // Trường hợp cơ sở: cả hai nút đều rỗng, hoặc chỉ có một trong hai nút rỗng
    if t1.is_none() && t2.is_none() {
        true
    } else if t1.is_none() || t2.is_none() {
        false
    } else {
        // So sánh giá trị của hai nút
        let t1 = t1.as_ref().unwrap().borrow();
        let t2 = t2.as_ref().unwrap().borrow();
        if t1.val != t2.val {
            false
        } else {
            // Đệ quy kiểm tra tính đối xứng giữa nút trái của t1 và nút phải của t2,
            // và giữa nút phải của t1 và nút trái của t2
            is_mirror(&t1.left, &t2.right) && is_mirror(&t1.right, &t2.left)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_symmetric() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));
        assert!(is_symmetric(root));
        
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            }))),
        })));
        assert!(!is_symmetric(root));
    }
}
