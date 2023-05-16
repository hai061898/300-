// Input:

// Hai số nguyên a và b (0 ≤ a ≤ b ≤ 2^31 - 1).
// Output:

// Giá trị bitwise AND của tất cả các số nguyên trong dãy từ a đến b.

// Input: a = 5, b = 7
// Output: 4

// Input: a = 0, b = 1
// Output: 0


fn range_bitwise_and(a: i32, b: i32) -> i32 {
    let mut shift = 0;
    let mut a = a;
    let mut b = b;
    
    // Tiến hành dịch bit sang phải cho 'a' và 'b' cho đến khi 'a' và 'b' có giá trị bằng nhau hoặc 'a' trước 'b'
    while a < b {
        a >>= 1;
        b >>= 1;
        shift += 1;
    }
    
    // Dịch bit sang trái cho 'a' 'shift' lần và trả về giá trị 'a' đó là kết quả của phép AND bitwise của dãy số từ 'a' đến 'b'
    a << shift
}
fn main() {
    let a = 5;
    let b = 7;
    let result = range_bitwise_and(a, b);
    println!("Result: {}", result);
}
