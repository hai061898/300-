// Cho một cây nhị phân, hãy kiểm tra xem nó có phải là một cây đối xứng hay không (tức là nó có thể gập đôi theo trục trung tâm của nó).

// Input: root = [1,2,2,3,4,4,3]
// Output: true

// 1
// / \
// 2   2
// / \ / \
// 3  4 4  3


// Input: root = [1,2,2,null,3,null,3]
// Output: false

// 1
// / \
// 2   2
// \   \
// 3    3

struct Solution;


// Định nghĩa trait Symmetric để kiểm tra tính đối xứng của cây nhị phân
trait Symmetric {
    fn is_symmetric(&self) -> bool;          // Hàm kiểm tra tính đối xứng của cây
    fn is_mirror(&self, right: &TreeLink) -> bool;   // Hàm kiểm tra tính đối xứng của hai cây con trái và phải
}

// Implement trait Symmetric cho TreeLink (được định nghĩa trong thư viện rustgym_util)
impl Symmetric for TreeLink {
    // Hàm kiểm tra tính đối xứng của cây nhị phân
    fn is_symmetric(&self) -> bool {
        // Kiểm tra trường hợp đặc biệt cây trống
        if let Some(node) = self {
            let node = node.borrow();
            node.left.is_mirror(&node.right)   // Kiểm tra tính đối xứng của hai cây con trái và phải
        } else {
            true
        }
    }

    // Hàm kiểm tra đối xứng của hai cây con trái và phải
    fn is_mirror(&self, right: &TreeLink) -> bool {
        // Kiểm tra các trường hợp đặc biệt
        match (self, right) {
            // Cả hai cây con đều trống
            (None, None) => true,
            // Một trong hai cây con trống
            (None, _) | (_, None) => false,
            (Some(p), Some(q)) => {
                let p = p.borrow();
                let q = q.borrow();
                p.val == q.val &&            // Kiểm tra giá trị của nút hiện tại
                p.left.is_mirror(&q.right) &&   // Kiểm tra tính đối xứng của cây con trái của cây con trái và cây con phải của cây con phải
                p.right.is_mirror(&q.left)      // Kiểm tra tính đối xứng của cây con phải của cây con trái và cây con trái của cây con phải
            }
            _ => false,
        }
    }
}

// Implement struct Solution
impl Solution {
    // Hàm kiểm tra tính đối xứng của cây nhị phân
    fn is_symmetric(root: TreeLink) -> bool {
        root.is_symmetric()   // Gọi hàm kiểm tra tính đối xứng của cây
    }
}

// Hàm kiểm tra bằng unit test
#[test]
fn test() {
    let q = tree!(
        1,
        tree!(2, tree!(1), tree!(1)),
        tree!(2, tree!(1), tree!(1))
    );
    let res = true;
    assert_eq!(Solution::is_symmetric(q), res);
}




