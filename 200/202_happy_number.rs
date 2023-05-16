// Đề bài "Happy Number" yêu cầu kiểm tra xem một số nguyên dương có phải là "happy number" hay không. Một số được coi là "happy number" nếu sau một chuỗi các bước tính toán, tổng bình phương của các chữ số của số đó cuối cùng là 1.

// Ví dụ:

// Input: 19
// Output: true

// Input: 2
// Output: false

use std::collections::HashSet;

fn is_happy(n: i32) -> bool {
    let mut n = n;
    let mut seen = HashSet::new();
    
    // Tiếp tục tính toán cho đến khi n = 1 hoặc đã gặp lại một số đã xuất hiện trước đó
    while n != 1 && !seen.contains(&n) {
        seen.insert(n);
        n = get_next(n);
    }
    
    // Trả về true nếu n là happy number (n = 1), ngược lại trả về false
    n == 1
}

fn get_next(n: i32) -> i32 {
    let mut n = n;
    let mut next = 0;
    
    // Lấy từng chữ số của n, bình phương và cộng dồn vào next
    while n > 0 {
        let digit = n % 10;
        next += digit * digit;
        n /= 10;
    }
    
    next
}

fn main() {
    let n = 19;
    let result = is_happy(n);
    println!("Is {} a happy number? {}", n, result);
}
