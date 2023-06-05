// Đề bài: Count Primes

// Một số nguyên dương n, hãy viết một chương trình để trả về số lượng số nguyên tố nhỏ hơn n.

// Ví dụ:

// Input: 10

// Output: 4

// Giải thích: Các số nguyên tố nhỏ hơn 10 là 2, 3, 5, 7.

fn count_primes(n: i32) -> i32 {
    // Tạo một mảng bool để theo dõi các số nguyên tố
    let mut is_prime = vec![true; n as usize];

    // Khởi tạo số lượng số nguyên tố là 0
    let mut count = 0;

    // Duyệt qua từng số từ 2 đến n - 1
    for num in 2..n {
        // Nếu num là số nguyên tố
        if is_prime[num as usize] {
            // Tăng đếm số lượng số nguyên tố
            count += 1;

            // Đánh dấu tất cả bội của num là false, vì chúng không phải là số nguyên tố
            let mut multiple = 2 * num;
            while multiple < n {
                is_prime[multiple as usize] = false;
                multiple += num;
            }
        }
    }

    count
}

fn main() {
    let n = 20;
    let result = count_primes(n);
    println!("The number of primes less than {} is {}", n, result);
}
