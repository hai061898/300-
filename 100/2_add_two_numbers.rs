// Bài toán yêu cầu chúng ta tính tổng của hai số được biểu diễn dưới dạng danh sách liên kết. Mỗi nút trong danh sách liên kết đại diện cho một chữ số của số lớn hơn hoặc bằng 0.

// Ví dụ, nếu số thứ nhất là 2 -> 4 -> 3 và số thứ hai là 5 -> 6 -> 4, thì kết quả của phép cộng hai số này sẽ là 7 -> 0 -> 8 (tức là 342 + 465 = 807).
use rustgym_util::*;

impl Solution {
    fn add_two_numbers(l1: ListLink, l2: ListLink) -> ListLink {
        // Khởi tạo giá trị ban đầu cho biến sum, p1, p2 và p3
        let mut sum: ListLink = None;
        let mut p1: &ListLink = &l1;
        let mut p2: &ListLink = &l2;
        let mut p3: &mut ListLink = &mut sum;
        let mut carry = 0;

        // Vòng lặp này lặp cho đến khi p1, p2 và carry đều bằng 0.
        while p1.is_some() || p2.is_some() || carry != 0 {
            // Tính toán tổng của các node hiện tại trong p1, p2 và carry.
            let mut val = carry;
            if let Some(n1) = p1.as_ref() {
                val += n1.val;
                p1 = &n1.next;
            }
            if let Some(n2) = p2.as_ref() {
                val += n2.val;
                p2 = &n2.next;
            }
            // Cập nhật giá trị của carry và giá trị của node mới trong sum.
            carry = val / 10;
            *p3 = ListLink::link(val % 10, None);
            p3 = &mut p3.as_mut().unwrap().next;
        }
        // Trả về danh sách liên kết sum, đó là tổng của l1 và l2.
        sum
    }
}

#[test]
fn test() {
    // Tạo danh sách liên kết l1, l2 và l3 (kết quả mong đợi).
    let l1 = list!(2, 4, 3);
    let l2 = list!(5, 6, 4);
    let l3 = list!(7, 0, 8);
    // Kiểm tra kết quả của phương thức add_two_numbers và l3 có giống nhau hay không.
    assert_eq!(Solution::add_two_numbers(l1, l2), l3);
}
