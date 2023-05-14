// Xây dựng cây nhị phân từ chuỗi xem trước (preorder) và chuỗi xem giữa (inorder)

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

fn build_tree<T: PartialEq + Copy>(preorder: &[T], inorder: &[T]) -> Option<Box<TreeNode<T>>> {
    if preorder.is_empty() || inorder.is_empty() {
        return None;
    }

    // Lấy giá trị của nút gốc từ chuỗi xem trước
    let root_val = preorder[0];

    // Tìm vị trí của nút gốc trong chuỗi xem giữa
    let mut root_idx = 0;
    while inorder[root_idx] != root_val {
        root_idx += 1;
    }

    // Xây dựng nút gốc
    let mut root = TreeNode::new(root_val);

    // Xây dựng cây con bên trái từ các phần tử trong chuỗi xem trước và chuỗi xem giữa trước nút gốc
    root.left = build_tree(&preorder[1..=root_idx], &inorder[..root_idx]);

    // Xây dựng cây con bên phải từ các phần tử trong chuỗi xem trước và chuỗi xem giữa sau nút gốc
    root.right = build_tree(&preorder[root_idx + 1..], &inorder[root_idx + 1..]);

    Some(Box::new(root))
}

fn main() {
    // Ví dụ chuỗi xem trước và chuỗi xem giữa
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];

    // Xây dựng cây nhị phân từ chuỗi xem trước và chuỗi xem giữa
    let tree = build_tree(&preorder, &inorder);

    // In ra cây nhị phân đã xây dựng
    println!("{:?}", tree);
}


// [test]
// fn test() {
//     let preorder = vec![3, 9, 20, 15, 7];
//     let inorder = vec![9, 3, 15, 20, 7];
//     let res = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
//     assert_eq!(Solution::build_tree(preorder, inorder), res);
// }