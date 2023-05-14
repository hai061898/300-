// Yêu cầu:

// Cho một danh sách liên kết đã được sắp xếp theo thứ tự tăng dần.
// Hãy xây dựng một cây tìm kiếm nhị phân (Binary Search Tree) từ danh sách liên kết đã cho.
// Input: Danh sách liên kết đã được sắp xếp theo thứ tự tăng dần.
// Output: Cây tìm kiếm nhị phân được xây dựng từ danh sách liên kết đã cho.


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

// Định nghĩa cấu trúc của một nút trong danh sách liên kết
#[derive(Debug)]
struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn new(val: T) -> Self {
        ListNode { val, next: None }
    }
}

fn sorted_list_to_bst(head: Option<Box<ListNode<i32>>>) -> Option<Box<TreeNode<i32>>> {
    // Chuyển đổi danh sách liên kết thành mảng
    let mut nums = vec![];
    let mut current = head;

    while let Some(node) = current {
        nums.push(node.val);
        current = node.next;
    }

    // Gọi hàm xây dựng cây tìm kiếm nhị phân từ mảng đã chuyển đổi
    sorted_array_to_bst(&nums)
}

fn sorted_array_to_bst(nums: &[i32]) -> Option<Box<TreeNode<i32>>> {
    // Nếu mảng rỗng, trả về giá trị None
    if nums.is_empty() {
        return None;
    }

    // Tìm phần tử ở giữa của mảng, đó là nút gốc của cây
    let mid = nums.len() / 2;

    let mut root = TreeNode::new(nums[mid]);

    // Xây dựng cây con bên trái của nút gốc từ mảng con bên trái của mảng đã cho
    root.left = sorted_array_to_bst(&nums[..mid]);

    // Xây dựng cây con bên phải của nút gốc từ mảng con bên phải của mảng đã cho
    root.right = sorted_array_to_bst(&nums[mid + 1..]);

    // Trả về cây nhị phân đã xây dựng từ mảng đã cho
    Some(Box::new(root))
}

fn main() {
    // Ví dụ danh sách liên kết đã sắp xếp
    let list = Some(Box::new(ListNode {
        val: -10,
        next: Some(Box::new(ListNode {
            val: -3,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode::new(9))),
                })),
            })),
        })),
    }));

    // Xây dựng cây tìm kiếm nhị phân từ danh sách liên kết
    let tree = sorted_list_to_bst(list);

    // In cây tìm kiếm nhị phân đã xây dựng
    println!("{:#?}", tree);
}
