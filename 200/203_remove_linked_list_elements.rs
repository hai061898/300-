// Đề bài "Remove Linked List Elements" yêu cầu xóa các nút có giá trị x khỏi danh sách liên kết đơn.

// Đề bài cung cấp đầu vào và đầu ra như sau:

// Input:

// head: Con trỏ đầu tiên của danh sách liên kết.
// x: Giá trị cần xóa khỏi danh sách.
// Output:

// Trả về con trỏ đầu tiên của danh sách liên kết sau khi đã xóa các nút có giá trị x.

// Định nghĩa cấu trúc cho một nút trong danh sách liên kết
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

// Hàm xóa các nút có giá trị x khỏi danh sách liên kết
pub fn remove_elements(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    // Tạo một nút giả để định nghĩa đầu danh sách liên kết
    let mut dummy = ListNode { val: 0, next: head };
    let mut curr = &mut dummy;

    // Duyệt qua danh sách liên kết
    while curr.next.is_some() {
        if curr.next.as_ref().unwrap().val == x {
            // Xóa nút có giá trị x
            let next_node = curr.next.as_mut().unwrap().next.take();
            curr.next = next_node;
        } else {
            // Di chuyển đến nút tiếp theo
            curr = curr.next.as_mut().unwrap();
        }
    }

    dummy.next
}

fn main() {
    // Tạo danh sách liên kết ban đầu: 1 -> 2 -> 6 -> 3 -> 4 -> 5 -> 6
    let mut head = Some(Box::new(ListNode { val: 1, next: None }));
    let mut curr = &mut head;
    curr.as_mut().unwrap().next = Some(Box::new(ListNode { val: 2, next: None }));
    curr = &mut curr.as_mut().unwrap().next;
    curr.as_mut().unwrap().next = Some(Box::new(ListNode { val: 6, next: None }));
    curr = &mut curr.as_mut().unwrap().next;
    curr.as_mut().unwrap().next = Some(Box::new(ListNode { val: 3, next: None }));
    curr = &mut curr.as_mut().unwrap().next;
    curr.as_mut().unwrap().next = Some(Box::new(ListNode { val: 4, next: None }));
    curr = &mut curr.as_mut().unwrap().next;
    curr.as_mut().unwrap().next = Some(Box::new(ListNode { val: 5, next: None }));
    curr = &mut curr.as_mut().unwrap().next;
    curr.as_mut().unwrap().next = Some(Box::new(ListNode { val: 6, next: None }));

    // Hiển thị danh sách liên kết ban đầu
    println!("Danh sách liên kết ban đầu: {:?}", head);

    // Gọi hàm remove_elements để xóa các nút có giá trị 6
    let result = remove_elements(head, 6);

    // Hiển thị danh sách liên kết sau khi xóa
    println!("Danh sách liên kết sau khi xóa: {:?}", result);
}
