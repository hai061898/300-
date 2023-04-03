// Input: startTime = [1,2,3], endTime = [3,2,7], queryTime = 4
// Output: 1
// Giải thích:

// Bài tập 0 hoàn thành từ thời gian 1 đến thời gian 3 so với thời điểm queryTime = 4 là chưa hoàn thành.
// Bài tập 1 hoàn thành từ thời gian 2 đến thời gian 2 so với thời điểm queryTime = 4 là chưa hoàn thành.
// Bài tập 2 hoàn thành từ thời gian 3 đến thời gian 7 so với thời điểm queryTime = 4 là đã hoàn thành

fn busy_student(start_time: Vec<i32>, end_time: Vec<i32>, query_time: i32) -> i32 {
    let mut count = 0;
    for i in 0..start_time.len() {
        if query_time >= start_time[i] && query_time <= end_time[i] {
            count += 1;
        }
    }
    count
}

fn main() {
    let start_time = vec![1, 2, 3];
    let end_time = vec![3, 2, 7];
    let query_time = 4;
    let count = busy_student(start_time, end_time, query_time);
    println!("{}", count);
}
