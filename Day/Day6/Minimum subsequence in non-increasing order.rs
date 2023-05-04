fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_by(|a, b| b.cmp(a));
    
    let total_sum: i32 = nums.iter().sum();
    let mut sub_sum = 0;
    let mut subsequence = Vec::new();
    
    for num in &nums {
        sub_sum += num;
        subsequence.push(*num);
        if sub_sum > total_sum - sub_sum {
            break;
        }
    }
    
    subsequence
}

fn main() {
    let nums = vec![4, 3, 10, 9, 8];
    let result = min_subsequence(nums);
    println!("{:?}", result);
}