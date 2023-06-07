// "3Sum Closest" yêu cầu tìm tổng của ba số trong một mảng, sao cho tổng này gần nhất với một giá trị đích (target).
pub fn three_sum_closest(nums: Vec<i32>, target: i32) -> i32 {
    let n = nums.len();
    let mut nums = nums;
    nums.sort();
    let mut closest_sum = nums[0] + nums[1] + nums[2];

    for i in 0..n - 2 {
        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == target {
                // Nếu tổng bằng target, trả về kết quả
                return sum;
            } else {
                // Nếu tổng mới gần hơn với target, cập nhật closest_sum
                if (sum - target).abs() < (closest_sum - target).abs() {
                    closest_sum = sum;
                }

                if sum < target {
                    left += 1;
                } else {
                    right -= 1;
                }
            }
        }
    }

    closest_sum
}

fn main() {
    let nums = vec![-1, 2, 1, -4];
    let target = 1;
    let result = three_sum_closest(nums, target);
    println!("Closest sum to target: {}", result);
}
