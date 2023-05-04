fn min_start_value(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut min_sum = 0;
    for num in nums {
        sum += num;
        if sum < min_sum {
            min_sum = sum;
        }
    }
    1 - min_sum
}
fn main() {
    let nums = vec![-3, 2, -3, 4, 2];
    let result = min_start_value(nums);
    println!("Minimum start value: {}", result);
}

// Với nums = [-3, 2, -3, 4, 2], kết quả minimum_value(nums) sẽ là 5.

// Ta có thể tạo ra một dãy số với các bước nhảy lần lượt là: -3, -1, -4, 0, 2. Tổng bước nhảy là 4, không dương.
// Ta có thể tạo ra một dãy số với các bước nhảy lần lượt là: -3, -1, -4, 0, 2, 5. Tổng bước nhảy là 5, dương.
// Vậy minimum_value(nums) = 5.