// Cho hai chuỗi đầu vào s và p, hãy hiện thực hóa việc kiểm tra sự khớp với biểu thức chính quy (regular expression) với hỗ trợ các ký tự '.' và '*' trong đó:

// '.' khớp với bất kỳ một ký tự nào.
// '*' khớp với số lượng ký tự trước nó bằng 0 hoặc nhiều.

struct Solution;

impl Solution {
    pub fn is_match(s: String, p: String) -> bool {
        // Chuyển các chuỗi vào trong các Vector của Rust
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        // Độ dài của s và p
        let n = s.len();
        let m = p.len();

        // Khởi tạo ma trận dp kích thước (n+1) x (m+1) với giá trị mặc định là false
        let mut dp = vec![vec![false; m + 1]; n + 1];

        // Khởi tạo giá trị đầu tiên của dp là true, tức là s rỗng và p rỗng.
        dp[0][0] = true;

        // Duyệt qua từng phần tử của s và p để tính toán giá trị của ma trận dp
        for i in 0..=n {
            for j in 1..=m {
                if p[j - 1] == '*' {
                    // Nếu p[j-1] là ký tự "*", có 2 trường hợp:
                    // - Bỏ qua ký tự "*" và ký tự trước đó của nó bằng cách sử dụng dp[i][j-2].
                    // - Sử dụng ký tự "*" để so khớp với ký tự trước đó của s nếu điều kiện thỏa mãn
                    dp[i][j] = dp[i][j - 2] || (i > 0 && (s[i - 1] == p[j - 2] || p[j - 2] == '.') && dp[i - 1][j]);
                } else {
                    // Nếu p[j-1] không phải là ký tự "*", so khớp các ký tự của s và p
                    dp[i][j] = i > 0 && (s[i - 1] == p[j - 1] || p[j - 1] == '.') && dp[i - 1][j - 1];
                }
            }
        }

        // Trả về giá trị của dp[n][m], tức là kết quả của việc so khớp s và p
        dp[n][m]
    }
}

// Hàm main để test giải thuật
fn main() {
    let s = String::from("aa");
    let p = String::from("a*");
    assert_eq!(Solution::is_match(s, p), true);

    let s = String::from("mississippi");
    let p = String::from("mis*is*p*.");
    assert_eq!(Solution::is_match(s, p), false);
}
