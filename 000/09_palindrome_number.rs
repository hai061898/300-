// kiểm tra xem một số nguyên có phải là số đối xứng hay không.

struct Solution;

impl Solution {
    fn is_palindrome(x: i32) -> bool {
        // Trả về false nếu x là số âm
        if x < 0 {
            return false;
        }
        // Khởi tạo reversed là 0, number là x
        let mut reversed = 0;
        let mut number = x;
        // Trong khi number > 0, lấy số đảo của number và gán vào reversed
        while number > 0 {
            reversed = reversed * 10 + number % 10;
            number /= 10;
        }
        // So sánh x với số đảo của nó để kiểm tra xem x có phải số đối xứng hay không
        x == reversed
        // Cách khác để kiểm tra: so sánh số ban đầu với số đảo của nó dưới dạng chuỗi
        // let x_str = x.abs().to_string();
        // x >= 0 && x_str == x_str.chars().rev().collect::<String>()
    }
}

#[test]
fn test() {
    let x = -123;
    let res = false;
    assert_eq!(Solution::is_palindrome(x), res);
    let x = 12321;
    let res = true;
    assert_eq!(Solution::is_palindrome(x), res);
}
