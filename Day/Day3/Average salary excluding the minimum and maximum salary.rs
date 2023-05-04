// Giả sử mảng salary ban đầu là: [4000,3000,1000,2000].

// Ta loại trừ giá trị lương nhỏ nhất (1000) và lớn nhất (4000) và tính trung bình của các giá trị còn lại.

// Trung bình lương sẽ là: (3000+2000) / 2 = 2500.

fn average(salary: Vec<i32>) -> f64 {
    let mut sum = 0;
    let mut max = std::i32::MIN;
    let mut min = std::i32::MAX;

    for s in salary {
        sum += s;
        max = max.max(s);
        min = min.min(s);
    }

    ((sum - max - min) as f64) / ((salary.len() - 2) as f64)
}

fn main() {
    let nums = vec![4000,3000,1000,2000];
    let result = average(&nums);
    println!("{}", result);
}