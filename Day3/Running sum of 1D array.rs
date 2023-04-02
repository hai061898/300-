fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());
    let mut sum = 0;
    for num in nums {
        sum += num;
        result.push(sum);
    }
    result
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = running_sum(nums);
    println!("{:?}", result); // Output: [1, 3, 6, 10]
}