// Đề bài: Tìm độ dài của dãy con tăng dài nhất trong một mảng số nguyên.

// Input:
// Một mảng số nguyên, ví dụ: [10, 9, 2, 5, 3, 7, 101, 18]

// Output:
// Một số nguyên biểu thị độ dài của dãy con tăng dài nhất, ví dụ: 4 (với dãy con tăng dài nhất là [2, 3, 7, 101])

fn longest_increasing_subsequence(nums: &[i32]) -> i32 {
    let n = nums.len();
    if n == 0 {
        return 0;
    }

    // Khởi tạo mảng dp với giá trị mặc định là 1
    let mut dp = vec![1; n];

    // Tính toán dãy con tăng dài nhất
    for i in 1..n {
        for j in 0..i {
            if nums[i] > nums[j] && dp[i] < dp[j] + 1 {
                dp[i] = dp[j] + 1;
            }
        }
    }

    // Tìm giá trị lớn nhất trong mảng dp
    let mut max_length = 0;
    for i in 0..n {
        if dp[i] > max_length {
            max_length = dp[i];
        }
    }

    max_length
}

fn main() {
    let nums = [10, 9, 2, 5, 3, 7, 101, 18];
    let result = longest_increasing_subsequence(&nums);
    println!("Độ dài của dãy con tăng dài nhất là: {}", result);
}
