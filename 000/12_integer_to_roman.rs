// Integer to Roman

// Roman numerals are represented by seven different symbols: I, V, X, L, C, D, and M.

// Symbol	Value
// I	1
// V	5
// X	10
// L	50
// C	100
// D	500
// M	1000
// For example, two is written as II in Roman numeral, just two one's added together. Twelve is written as, XII, which is simply X + II. The number twenty-seven is written as XXVII, which is XX + V + II.

// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not IIII. Instead, the number four is written as IV. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as IX. There are six instances where subtraction is used:

// I can be placed before V (5) and X (10) to make 4 and 9.
// X can be placed before L (50) and C (100) to make 40 and 90.
// C can be placed before D (500) and M (1000) to make 400 and 900.
// Given an integer, convert it to a roman numeral. Input is guaranteed to be within the range from 1 to 3999.

// Example 1:

// Input: 3

// Output: "III"

// Example 2:

// Input: 4

// Output: "IV"

// Example 3:

// Input: 9

// Output: "IX"

// Example 4:

// Input: 58

// Output: "LVIII"

// Explanation: L = 50, V = 5, III = 3.

// Example 5:

// Input: 1994

// Output: "MCMXCIV"

// Explanation: M = 1000, CM = 900, XC = 90, IV = 4.


fn int_to_roman(num: i32) -> String {
    // Khởi tạo mảng chứa các giá trị tương ứng với ký tự La Mã từ lớn đến nhỏ
    let values = vec![1000, 900, 500, 400, 100, 90, 50, 40, 10, 9, 5, 4, 1];
    // Khởi tạo mảng chứa các ký tự La Mã tương ứng với các giá trị
    let symbols = vec!["M", "CM", "D", "CD", "C", "XC", "L", "XL", "X", "IX", "V", "IV", "I"];
    // Khởi tạo chuỗi kết quả rỗng
    let mut result = String::new();
    // Biến để lưu giá trị số còn lại
    let mut remaining = num;

    // Duyệt qua mảng values và symbols
    for (i, &value) in values.iter().enumerate() {
        // Kiểm tra nếu giá trị hiện tại nhỏ hơn hoặc bằng giá trị còn lại
        while remaining >= value {
            // Thêm ký tự tương ứng vào chuỗi kết quả
            result.push_str(symbols[i]);
            // Giảm giá trị còn lại đi
            remaining -= value;
        }
    }

    // Trả về chuỗi kết quả
    result
}

fn main() {
    // Nhập giá trị đầu vào
    let num = 123;
    // Chuyển đổi số nguyên thành ký tự La Mã
    let roman = int_to_roman(num);
    // Hiển thị kết quả
    println!("The Roman numeral representation of {} is: {}", num, roman);
}
