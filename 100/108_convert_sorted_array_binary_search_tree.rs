// Yêu cầu:

// Cho một mảng số nguyên đã được sắp xếp theo thứ tự tăng dần.
// Hãy xây dựng một cây tìm kiếm nhị phân (Binary Search Tree) từ mảng đã cho.
// Input: Một mảng số nguyên đã được sắp xếp theo thứ tự tăng dần.
// Output: Cây tìm kiếm nhị phân được xây dựng từ mảng đã cho.

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

// Xây dựng cây tìm kiếm nhị phân từ mảng đã cho
fn sorted_array_to_bst(nums: &[i32]) -> Option<Box<TreeNode<i32>>> {
    // Kiểm tra nếu mảng rỗng, trả về None
    if nums.is_empty() {
        return None;
    }

    // Tìm phần tử ở giữa mảng
    let mid = nums.len() / 2;

    // Tạo một nút mới với giá trị là nút gốc
    let mut root = TreeNode::new(nums[mid]);

    // Đệ quy xây dựng cây tìm kiếm nhị phân cho phần mảng bên trái của mid và gán vào nút con trái của nút gốc
    root.left = sorted_array_to_bst(&nums[..mid]);

    // Đệ quy xây dựng cây tìm kiếm nhị phân cho phần mảng bên phải của mid và gán vào nút con phải của nút gốc
    root.right = sorted_array_to_bst(&nums[mid + 1..]);

    Some(Box::new(root))
}

fn main() {
    // Ví dụ mảng đã sắp xếp
    let nums = vec![-10, -3, 0, 5, 9];

    // Xây dựng cây tìm kiếm nhị phân từ mảng đã cho
    let tree = sorted_array_to_bst(&nums);

    // In ra cây tìm kiếm nhị phân đã xây dựng
    println!("{:?}", tree);
}

// #[test]
// fn test() {
//     let nums = vec![-10, -3, 0, 5, 9];
//     let bst = tree!(0, tree!(-3, tree!(-10), None), tree!(9, tree!(5), None));
//     assert_eq!(Solution::sorted_array_to_bst(nums), bst);
// }