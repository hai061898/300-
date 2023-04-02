// Input: nums = [2,5,1,3,4,7], n = 3
// Output: [2,3,5,4,1,7]

fn main() {
    let nums = vec![2,5,1,3,4,7];
    let n = 3;
    let shuffled_nums = shuffle(nums, n);
    println!("{:?}", shuffled_nums);
}

fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let mut result = Vec::with_capacity(nums.len());
    for i in 0..n {
        result.push(nums[i as usize]);
        result.push(nums[(i + n) as usize]);
    }
    result
}