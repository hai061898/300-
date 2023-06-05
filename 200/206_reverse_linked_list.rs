// Đề bài: Reverse Linked List

// Cho một danh sách liên kết đơn, hãy đảo ngược danh sách đó.

// Ví dụ:

// Input: 1 -> 2 -> 3 -> 4 -> 5 -> NULL

// Output: 5 -> 4 -> 3 -> 2 -> 1 -> NULL


// Định nghĩa cấu trúc Node cho danh sách liên kết đơn
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    // Phương thức tạo nút mới với giá trị đã cho và next là None
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Hàm đảo ngược danh sách liên kết
pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        let next = node.next;
        node.next = prev;
        prev = Some(node);
        current = next;
    }

    prev
}

fn main() {
    // Tạo danh sách liên kết đơn: 1 -> 2 -> 3 -> 4 -> 5
    let mut head = Some(Box::new(ListNode::new(1)));
    let mut current = head.as_mut();

    if let Some(node) = current {
        node.next = Some(Box::new(ListNode::new(2)));
        current = node.next.as_mut();
    }

    if let Some(node) = current {
        node.next = Some(Box::new(ListNode::new(3)));
        current = node.next.as_mut();
    }

    if let Some(node) = current {
        node.next = Some(Box::new(ListNode::new(4)));
        current = node.next.as_mut();
    }

    if let Some(node) = current {
        node.next = Some(Box::new(ListNode::new(5)));
    }

    // Đảo ngược danh sách liên kết
    let reversed_list = reverse_list(head);

    // In danh sách liên kết đã đảo ngược
    let mut current = reversed_list.as_ref();
    while let Some(node) = current {
        print!("{} ", node.val);
        current = node.next.as_ref();
    }
}
