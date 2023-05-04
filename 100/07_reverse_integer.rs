// Đề bài: Cho một số nguyên x, hãy đảo ngược các chữ số của nó và trả về số mới sau khi đảo ngược. Nếu số mới vượt quá phạm vi của số nguyên 32-bit có dấu, hãy trả về 0.

struct Solution;

impl Solution {
    fn reverse(x: i32) -> i32 {
        // Chuyển số thành chuỗi, lật ngược và ghép lại thành chuỗi mới
        let x_str = x.abs().to_string().chars().rev().collect::<String>();
        // Parse chuỗi mới thành số nguyên
        if let Ok(y) = x_str.parse::<i32>() {
            // Trả về số đảo ngược kèm theo dấu của x
            x.signum() * y //signum trả về dấu của 1 số 
        } else {
            // Nếu không parse được, trả về 0
            0
        }
    }
}

#[test]
fn test() {
    let x = 2_147_483_647;
    let res = 0;
    assert_eq!(Solution::reverse(x), res);
    let x = 123_456_789;
    let res = 987_654_321;
    assert_eq!(Solution::reverse(x), res);
    let x = -123;
    let res = -321;
    assert_eq!(Solution::reverse(x), res);
}
