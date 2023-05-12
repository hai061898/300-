// Viết một chương trình Rust để thực hiện duyệt cây nhị phân theo thứ tự theo cấp (level order traversal) và trả về kết quả theo thứ tự ngược lại.
// Định nghĩa cấu trúc của một nút trong cây nhị phân
#[derive(Debug)]
struct TreeNode<T> {
    val: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// Thực hiện duyệt cây theo thứ tự level order và trả về kết quả theo thứ tự ngược lại
fn level_order_bottom<T>(root: Option<Box<TreeNode<T>>>) -> Vec<Vec<T>> {
    let mut result = Vec::new();

    // Kiểm tra nút gốc
    if root.is_none() {
        return result;
    }

    let mut queue = Vec::new();
    queue.push(root);

    while !queue.is_empty() {
        let mut level = Vec::new();
        let mut next_level = Vec::new();

        // Duyệt qua các nút trong cùng một cấp
        for node in queue {
            if let Some(inner_node) = node {
                level.push(inner_node.val);

                // Thêm nút con vào hàng đợi của cấp tiếp theo
                if inner_node.left.is_some() {
                    next_level.push(inner_node.left);
                }

                if inner_node.right.is_some() {
                    next_level.push(inner_node.right);
                }
            }
        }

        // Kiểm tra xem cấp hiện tại có nút nào hay không
        if !level.is_empty() {
            result.push(level);
        }

        // Di chuyển đến cấp tiếp theo
        queue = next_level;
    }

    // Đảo ngược kết quả để trả về theo thứ tự ngược lại
    result.reverse();
    result
}

fn main() {
    // Ví dụ cây nhị phân
    let tree = Some(Box::new(TreeNode {
        val: 3,
        left: Some(Box::new(TreeNode::new(9))),
        right: Some(Box::new(TreeNode {
            val: 20,
            left: Some(Box::new(TreeNode::new(15))),
            right: Some(Box::new(TreeNode::new(7))),
        })),
    }));

    // Thực hiện duyệt cây theo thứ tự level order và trả về kết quả theo thứ tự ngược lại
    let result = level_order_bottom(tree);

    // In ra kết quả
    for level in result {
        println!("{:?}", level);
    }
}
