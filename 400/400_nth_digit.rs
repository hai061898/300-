// "Nth Digit" yêu cầu tìm chữ số thứ n trong dãy số tự nhiên liên tiếp.
// Input: n = 11
// Output: Nth Digit: 0

// Input: n = 3
// Output: Nth Digit: 3

// Input: n = 100
// Output: Nth Digit: 5

pub fn find_nth_digit(n: i32) -> i32 {
    let n = n as i64;  // Chuyển đổi n sang kiểu i64 để đảm bảo tính toán chính xác cho các số lớn
    let mut base: i64 = 9;  // Giá trị cơ số ban đầu là 9 (số chữ số đầu tiên là 1-9)
    let mut digits: i64 = 1;  // Số chữ số ban đầu là 1
    let mut num = 1;  // Giá trị số ban đầu là 1

    // Tìm mức số chữ số của n
    while n - base * digits > 0 {
        n - base * digits;  // Bỏ qua các giá trị không sử dụng
        digits += 1;  // Tăng số chữ số lên một mức
        num *= 10;  // Tăng giá trị số lên một mức (10, 100, 1000, ...)
        base *= 10;  // Tăng cơ số lên một mức (90, 900, 9000, ...)
    }

    // Tính toán giá trị số cần tìm
    num += (n - 1) / digits;  // Thêm (n - 1) / digits vào giá trị số
    let position = (n - 1) % digits;  // Xác định vị trí của chữ số trong số
    let digit = num.to_string().chars().nth(position as usize).unwrap();  // Lấy chữ số tại vị trí đã xác định

    digit.to_digit(10).unwrap() as i32  // Chuyển đổi chữ số thành số nguyên và trả về kết quả
}

fn main() {
    let n = 11;
    let result = find_nth_digit(n);
    println!("Nth Digit: {}", result);
}
