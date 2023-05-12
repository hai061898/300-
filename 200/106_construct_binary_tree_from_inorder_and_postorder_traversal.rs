// Xây dựng cây nhị phân từ chuỗi xem giữa (inorder) và chuỗi xem sau (postorder)

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

fn build_tree<T: PartialEq + Copy>(inorder: &[T], postorder: &[T]) -> Option<Box<TreeNode<T>>> {
    if inorder.is_empty() || postorder.is_empty() {
        return None;
    }

    // Lấy giá trị của nút gốc từ chuỗi xem sau
    let root_val = postorder[postorder.len() - 1];

    // Tìm vị trí của nút gốc trong chuỗi xem giữa
    let mut root_idx = 0;
    while inorder[root_idx] != root_val {
        root_idx += 1;
    }

    // Xây dựng nút gốc
    let mut root = TreeNode::new(root_val);

    // Xây dựng cây con bên trái từ các phần tử trong chuỗi xem giữa và chuỗi xem sau trước nút gốc
    root.left = build_tree(&inorder[..root_idx], &postorder[..root_idx]);

    // Xây dựng cây con bên phải từ các phần tử trong chuỗi xem giữa và chuỗi xem sau sau nút gốc
    root.right = build_tree(&inorder[root_idx + 1..], &postorder[root_idx..postorder.len() - 1]);

    Some(Box::new(root))
}

fn main() {
    // Ví dụ chuỗi xem giữa và chuỗi xem sau
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];

    // Xây dựng cây nhị phân từ chuỗi xem giữa và chuỗi xem sau
    let tree = build_tree(&inorder, &postorder);

    // In ra cây nhị phân đã xây dựng
    println!("{:?}", tree);
}


// #[test]
// fn test() {
//     let inorder = vec![9, 3, 15, 20, 7];
//     let postorder = vec![9, 15, 7, 20, 3];
//     let res = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
//     assert_eq!(Solution::build_tree(inorder, postorder), res);
// }