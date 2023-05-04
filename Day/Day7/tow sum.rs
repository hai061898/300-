fn two_sum(nums: &[i32], target: i32) -> Vec<usize> {
    let mut map = std::collections::HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = map.get(&(target - num)) {
            return vec![j, i];
        } else {
            map.insert(num, i);
        }
    }
    vec![]
}

fn main() {
    let nums = [2, 7, 11, 15];
    let target = 9;

    let indices = two_sum(&nums, target);

    if indices.is_empty() {
        println!("No two elements add up to the target.");
    } else {
        println!("Indices of elements that add up to the target: {:?}", indices);
    }
}