// "Three Sum" yêu cầu tìm tất cả các bộ ba số trong một mảng, sao cho tổng của chúng bằng 0. 

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    let mut nums = nums;
    let n = nums.len();

    // Sắp xếp mảng đầu vào theo thứ tự tăng dần
    nums.sort();

    for i in 0..n - 2 {
        // Bỏ qua các giá trị trùng lặp của i
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = i + 1;
        let mut right = n - 1;

        while left < right {
            let sum = nums[i] + nums[left] + nums[right];
            if sum == 0 {
                // Tìm được một bộ ba số có tổng bằng 0
                result.push(vec![nums[i], nums[left], nums[right]]);

                // Bỏ qua các giá trị trùng lặp của left và right
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }

    result
}

fn main() {
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let result = three_sum(nums);
    println!("{:?}", result);
}
