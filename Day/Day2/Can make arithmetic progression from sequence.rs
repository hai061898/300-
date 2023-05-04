// Input: arr = [3,5,1]
// Output: true
// Explanation: You can reorder the elements as [1,3,5] or [5,3,1] to form a valid arithmetic progression.

fn can_make_arithmetic_progression(mut arr: Vec<i32>) -> bool {
    arr.sort();
    let diff = arr[1] - arr[0];
    for i in 2..arr.len() {
        if arr[i] - arr[i - 1] != diff {
            return false;
        }
    }
    true
}

fn main() {
    let nums = vec![1, 2, 4];
    let result = can_make_arithmetic_progression(nums);
    println!("Result: {}", result);

}