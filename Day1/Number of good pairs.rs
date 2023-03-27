// Given an array of integers nums.

// A pair (i,j) is called good if nums[i] == nums[j] and i < j.

// Return the number of good pairs.

use std::collections::HashMap;

fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::new();
    let mut count = 0;

    for num in nums {
        let entry = map.entry(num).or_insert(0);
        count += *entry;
        *entry += 1;
    }

    count
}

fn main() {
    let nums1 = vec![1, 2, 3, 1, 1, 3];

    println!("num_identical_pairs(nums1) = {}", num_identical_pairs(nums1));

}

