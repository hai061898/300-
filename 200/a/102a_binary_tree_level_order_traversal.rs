struct Solution;

// Định nghĩa trait Preorder để duyệt cây theo thứ tự preorder và lưu giữ giá trị của các nút vào `levels`
trait Preorder {
    fn preorder(&self, levels: &mut Vec<Vec<i32>>, level: usize);
}

// Triển khai trait Preorder cho kiểu dữ liệu `TreeLink`
impl Preorder for TreeLink {
    fn preorder(&self, levels: &mut Vec<Vec<i32>>, level: usize) {
        // Nếu cây không rỗng, thực hiện duyệt preorder để lưu giữ giá trị của các nút
        if let Some(node) = self {
            // Lấy giá trị của nút hiện tại
            let val = node.borrow().val;
            // Nếu levels chưa có vector tương ứng với cấp độ này, thêm một vector mới vào
            if levels.len() == level {
                levels.push(vec![val]);
            // Nếu levels đã có vector tương ứng với cấp độ này, thêm giá trị vào vector đó
            } else {
                levels[level].push(val);
            }
            // Thực hiện duyệt preorder cho cây con bên trái
            node.borrow().left.preorder(levels, level + 1);
            // Thực hiện duyệt preorder cho cây con bên phải
            node.borrow().right.preorder(levels, level + 1);
        }
    }
}

// Triển khai phương thức `level_order` cho cấu trúc `Solution`
impl Solution {
    fn level_order(root: TreeLink) -> Vec<Vec<i32>> {
        let mut res = vec![]; // Khởi tạo kết quả là một vector rỗng
        root.preorder(&mut res, 0); // Thực hiện duyệt cây theo thứ tự preorder để lưu giữ giá trị của các nút vào `res`
        res // Trả về kết quả
    }
}

#[test]
fn test() {
    let root = tree!(3, tree!(9), tree!(20, tree!(15), tree!(7)));
    let res: Vec<Vec<i32>> = vec_vec_i32![[3], [9, 20], [15, 7]];
    assert_eq!(Solution::level_order(root), res);
}
