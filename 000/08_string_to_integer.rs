// Đề bài: Viết nhận vào một chuỗi s và trả về một số nguyên (i32). Nếu s không phải là một số nguyên hợp lệ, hàm trả về 0.

// Một số trường hợp không hợp lệ:

// Chỉ có khoảng trắng hoặc các ký tự không phải số ở đầu chuỗi.
// Ký tự "+" hoặc "-" không ở đầu chuỗi.
// Số vượt quá giới hạn của kiểu dữ liệu i32.


struct Solution;
use std::i32;

impl Solution {
    fn my_atoi(s: String) -> i32 {
        let mut start = s.trim_start(); // bỏ qua khoảng trắng ở đầu chuỗi

        let mut res: i32 = 0;
        let mut positive = true; // biến này xác định số đang xét có dấu là dương hay âm

        if start.len() > 1 {
            let c = &start[0..1];
            match c {
                "+" => { // nếu ký tự đầu tiên là dấu "+"
                    start = &start[1..]; // bỏ qua ký tự đầu tiên
                }
                "-" => { // nếu ký tự đầu tiên là dấu "-"
                    start = &start[1..]; // bỏ qua ký tự đầu tiên
                    positive = false; // đánh dấu số đang xét là số âm
                }
                _ => { // nếu ký tự đầu tiên không phải dấu "+/-"
                    if let Some(c) = c.chars().next() {
                        if !('0'..='9').contains(&c) { // nếu ký tự đầu tiên không phải là số
                            return 0; // trả về 0
                        }
                    }
                }
            }
        }

        for c in start.chars() { // duyệt các ký tự tiếp theo trong chuỗi
            if ('0'..='9').contains(&c) { // nếu ký tự đó là số
                res = match res.checked_mul(10) { // nhân res với 10, tránh tràn số
                    None => {
                        return Self::overflow(positive); // nếu tràn số, trả về giá trị tương ứng với dấu
                    }
                    Some(val) => val,
                };
                res = match res.checked_add((c as u8 - b'0') as i32) { // cộng số vào res, tránh tràn số
                    None => {
                        return Self::overflow(positive); // nếu tràn số, trả về giá trị tương ứng với dấu
                    }
                    Some(val) => val,
                };
            } else { // nếu ký tự đó không phải là số
                break; // thoát vòng lặp
            }
        }

        if !positive { // nếu số đang xét là số âm
            res = match res.checked_mul(-1) { // đổi dấu của res
                None => {
                    return Self::overflow(positive); // nếu tràn số, trả về giá trị tương ứng với dấu
                }
                Some(val) => val,
            };
        }

        res // trả về kết quả cuối cùng
    }

    fn overflow(positive: bool) -> i32 {
        if positive {
            i32::MAX 
        }
        else {
            i32::MIN
        }

        //  hàm sẽ trả về giá trị tối đa cho kiểu i32, được định nghĩa trong thư viện chuẩn i32::MAX. Nếu biến positive có giá trị false, nghĩa là số đang xử lý là số âm, hàm sẽ trả về giá trị tối thiểu cho kiểu i32, được định nghĩa trong thư viện chuẩn i32::MIN.
    }
}

#[test]
fn test() {
    let s = "42".to_string();
    let res = 42;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "   -42".to_string();
    let res = -42;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "4193 with words".to_string();
    let res = 4193;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "words and 987".to_string();
    let res = 0;
    assert_eq!(Solution::my_atoi(s), res);
    let s = "-91283472332".to_string();
    let res = -2_147_483_648;
    assert_eq!(Solution::my_atoi(s), res);
}
