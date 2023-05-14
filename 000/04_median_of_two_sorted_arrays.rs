impl Solution {
    fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let n1 = nums1.len();
        let n2 = nums2.len();
        if n1 < n2 {
            return Self::find_median_sorted_arrays(nums2, nums1);
        }
        let mut lo = 0;
        let mut hi = n2 * 2;
        while lo <= hi {
            let mid2 = (lo + hi) / 2;
            let mid1 = n1 + n2 - mid2;
            let l1 = if mid1 == 0 {
                std::i32::MIN
            } else {
                nums1[(mid1 - 1) / 2]
            };
            let l2 = if mid2 == 0 {
                std::i32::MIN
            } else {
                nums2[(mid2 - 1) / 2]
            };
            let r1 = if mid1 == n1 * 2 {
                std::i32::MAX
            } else {
                nums1[mid1 / 2]
            };
            let r2 = if mid2 == n2 * 2 {
                std::i32::MAX
            } else {
                nums2[mid2 / 2]
            };

            if l1 > r2 {
                lo = mid2 + 1;
            } else if l2 > r1 {
                hi = mid2 - 1;
            } else {
                return (l1.max(l2) + r1.min(r2)) as f64 / 2.0;
            }
        }
        panic!()
    }
}

#[test]
fn test() {
    use assert_approx_eq::assert_approx_eq;
    let nums1 = vec![1, 3];
    let nums2 = vec![2];
    let res = 2.0;
    assert_approx_eq!(Solution::find_median_sorted_arrays(nums1, nums2), res);
    let nums1 = vec![1, 2];
    let nums2 = vec![3, 4];
    let res = 2.5;
    assert_approx_eq!(Solution::find_median_sorted_arrays(nums1, nums2), res);
    let nums1 = vec![1];
    let nums2 = vec![2, 3];
    let res = 2.0;
    assert_approx_eq!(Solution::find_median_sorted_arrays(nums1, nums2), res);
}

Đề bài: Cho hai mảng số nguyên đã sắp xếp theo thứ tự tăng dần nums1 và nums2 có độ dài lần lượt là n1 và n2. Tìm trung vị của hai mảng số này được hợp nhất. Trung vị là một số trong số các số được sắp xếp của mảng thỏa mãn điều kiện 50% các số trong mảng nhỏ hơn nó và 50% các số lớn hơn nó.

Giải thích trong đoạn code:

Hàm find_median_sorted_arrays nhận vào hai vector số nguyên nums1 và nums2.
Đầu tiên, chúng ta lấy độ dài của hai vector này và so sánh, nếu độ dài của nums1 nhỏ hơn độ dài của nums2, ta hoán đổi chúng để đảm bảo độ dài của nums1 lớn hơn độ dài của nums2.
Sau đó, ta thiết lập hai biến lo và hi, lần lượt là 0 và n2*2.
Tiếp theo, chúng ta bắt đầu một vòng lặp while, trong đó chúng ta tính toán chỉ số mid2 của phần tử ở giữa của nums2 và tính toán chỉ số mid1 của phần tử ở giữa của nums1.
Sau đó, ta tính toán các giá trị l1, r1, l2 và r2, lần lượt là giá trị trái và phải của hai phần tử ở giữa của nums1 và nums2.
Tiếp theo, ta sử dụng các giá trị này để so sánh và điều chỉnh biến lo hoặc hi.
Nếu l1 lớn hơn r2, ta tăng biến lo để tìm giá trị trung vị.
Nếu l2 lớn hơn r1, ta giảm biến hi.
Nếu hai điều kiện trên không được thỏa mãn, ta tìm được giá trị trung vị và trả về nó.
Nếu không tìm thấy giá trị trung vị, ta sẽ panic!().
Trong hàm kiểm tra, chúng ta sử dụng macro assert_approx_eq để kiểm tra kết quả trả về từ hàm find_median_sorted_arrays có gần đúng với kết quả mong đợi hay không.