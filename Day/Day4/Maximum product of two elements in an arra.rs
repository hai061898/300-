// Input: nums = [3,4,5,2]
// Output: 12
// Note :(5-1) * (4-1) = 12.

fn max_product(nums: Vec<i32>) -> i32 {
    let mut max1 = std::i32::MIN;
    let mut max2 = std::i32::MIN;

    for num in nums {
        if num >= max1 {
            max2 = max1;
            max1 = num;
        } else if num > max2 {
            max2 = num;
        }
    }

    (max1 - 1) * (max2 - 1)
}

fn main() {
    let nums = vec![3,4,5,2];
    let result = max_product(nums);
    println!("Maximum product of two elements in the array is: {}", result);
}