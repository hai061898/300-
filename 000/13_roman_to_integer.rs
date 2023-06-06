use std::collections::HashMap;

fn roman_to_int(s: String) -> i32 {
    // Khởi tạo một hashmap để ánh xạ các ký tự La Mã với giá trị tương ứng
    let mut roman_values = std::collections::HashMap::new();
    roman_values.insert('I', 1);
    roman_values.insert('V', 5);
    roman_values.insert('X', 10);
    roman_values.insert('L', 50);
    roman_values.insert('C', 100);
    roman_values.insert('D', 500);
    roman_values.insert('M', 1000);

    let chars: Vec<char> = s.chars().collect();
    let mut result = 0;

    // Duyệt qua từng ký tự trong chuỗi đầu vào
    for i in 0..chars.len() {
        let current_value = roman_values[&chars[i]];

        // Nếu ký tự hiện tại là ký tự cuối cùng hoặc có giá trị lớn hơn ký tự tiếp theo
        if i == chars.len() - 1 || roman_values[&chars[i + 1]] <= current_value {
            result += current_value;
        } else {
            // Nếu ký tự hiện tại có giá trị nhỏ hơn ký tự tiếp theo, ta trừ giá trị của ký tự hiện tại
            result -= current_value;
        }
    }

    result
}

fn main() {
    let s = String::from("MCMXCIV");
    let num = roman_to_int(s);
    println!("The integer representation of the Roman numeral {} is: {}", s, num);
}
