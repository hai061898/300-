// Định nghĩa struct cho một node trong linked list
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Định nghĩa alias ListLink cho Option<Box<ListNode>>
pub type ListLink = Option<Box<ListNode>>;

impl Solution {
    pub fn add_two_numbers(l1: ListLink, l2: ListLink) -> ListLink {
        // Khởi tạo một linked list rỗng để lưu tổng
        let mut sum = None;
        // Khởi tạo các con trỏ p1 và p2 trỏ đến head của l1 và l2
        let mut p1 = &l1;
        let mut p2 = &l2;
        // Khởi tạo con trỏ p3 trỏ đến head của linked list sum
        let mut p3 = &mut sum;
        // Khởi tạo biến carry để lưu giá trị nhớ
        let mut carry = 0;
        // Thực hiện vòng lặp cho đến khi cả p1 và p2 đều trỏ đến None và carry bằng 0
        while p1.is_some() || p2.is_some() || carry != 0 {
            // Lấy giá trị của node hiện tại của l1 và cộng với giá trị node hiện tại của l2 và giá trị carry
            let mut val = carry;
            if let Some(node) = p1 {
                val += node.val;
                p1 = &node.next;
            }
            if let Some(node) = p2 {
                val += node.val;
                p2 = &node.next;
            }
            // Tính giá trị của carry và giá trị cần thêm vào linked list sum
            carry = val / 10;
            *p3 = Some(Box::new(ListNode {
                val: val % 10,
                next: None,
            }));
            // Di chuyển con trỏ p3 đến node tiếp theo của linked list sum
            p3 = &mut p3.as_mut().unwrap().next;
        }
        // Trả về linked list sum
        sum
    }
}

fn main() {
    let l1 = list![2, 4, 3];
    let l2 = list![5, 6, 4];
    let l3 = list![7, 0, 8];
    let result = Solution::add_two_numbers(l1, l2);
    println!("Result: {:?}", result);
    println!("Expected: {:?}", l3);
    if result != l3 {
        println!("Test failed");
    } else {
        println!("Test passed");
    }
}