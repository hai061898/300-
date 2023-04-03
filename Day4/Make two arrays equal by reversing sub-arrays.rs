// Example 1:
// Input: target = [1,2,3,4], arr = [2,4,1,3]
// Output: true
// Explanation: You can obtain arr by reversing subarrays [1,2] and [3,4].

// Example 2:
// Input: target = [7], arr = [7]
// Output: true
// Explanation: arr is equal to target without any reverses.

// Example 3:
// Input: target = [1,12], arr = [12,1]
// Output: true


fn main() {
    let target = vec![1, 2, 3, 4];
    let arr = vec![2, 4, 1, 3];

    let mut target_counts = vec![0; 1001];
    let mut arr_counts = vec![0; 1001];

    for i in 0..target.len() {
        target_counts[target[i]] += 1;
        arr_counts[arr[i]] += 1;
    } // đếm số lần phần tử xuất hiện 

    for i in 0..target_counts.len() {
        if target_counts[i] != arr_counts[i] {
            println!("false");
            return;
        }
    } // so sánh 

    println!("true");
}
