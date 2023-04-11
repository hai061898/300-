impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let mut merged = vec![]; // Tạo một vector để lưu trữ mảng được ghép
        let mut i = 0; // Khởi tạo chỉ số i cho nums1
        let mut j = 0; // Khởi tạo chỉ số j cho nums2
        
        while i < nums1.len() && j < nums2.len() { // Lặp qua cả hai mảng
            if nums1[i] < nums2[j] { // Nếu phần tử hiện tại của nums1 nhỏ hơn phần tử hiện tại của nums2
                merged.push(nums1[i]); // Thêm phần tử của nums1 vào vector merged
                i += 1; // Tăng chỉ số của nums1
            } else { // Ngược lại
                merged.push(nums2[j]); // Thêm phần tử của nums2 vào vector merged
                j += 1; // Tăng chỉ số của nums2
            }
        }
        
        // Sau khi lặp xong, nếu còn phần tử chưa được thêm vào vector merged, ta sẽ thêm chúng vào
        while i < nums1.len() { 
            merged.push(nums1[i]);
            i += 1;
        }
        
        while j < nums2.len() {
            merged.push(nums2[j]);
            j += 1;
        }
        
        // Tính median của vector merged
        let n = merged.len();
        if n % 2 == 0 { // Nếu n là số chẵn
            (merged[n/2 - 1] as f64 + merged[n/2] as f64) / 2.0 // Median là trung bình cộng của hai giá trị ở giữa
        } else { // Ngược lại
            merged[n/2] as f64 // Median là giá trị ở giữa
        }
    }
}

fn main() {
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let result = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(result, 2.0);

    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let result = find_median_sorted_arrays(nums1, nums2);
    assert_eq!(result, 2.5);
}