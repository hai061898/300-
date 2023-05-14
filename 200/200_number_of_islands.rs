// Đề bài: Đếm số hòn đảo trong một ma trận 2D. Trong ma trận, các ô có giá trị 1 đại diện cho đất liền và các ô có giá trị 0 đại diện cho nước biển. Các ô được kết nối theo chiều ngang hoặc chiều dọc được coi là thuộc cùng một hòn đảo.

// Input:
// Một ma trận 2D (grid) có kích thước m hàng và n cột, trong đó grid[i][j] là 0 hoặc 1.

// Output:
// Số lượng hòn đảo trong ma trận.


fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut count = 0;  // Biến đếm số lượng hòn đảo
    let m = grid.len();  // Số hàng của ma trận
    if m == 0 {
        return 0;
    }
    let n = grid[0].len();  // Số cột của ma trận

    // Duyệt qua từng ô trong ma trận
    for i in 0..m {
        for j in 0..n {
            if grid[i][j] == '1' {  // Nếu gặp ô đất liền
                count += 1;  // Tăng biến đếm số lượng hòn đảo
                dfs(&grid, i, j);  // Duyệt toàn bộ hòn đảo từ ô hiện tại
            }
        }
    }

    count  // Trả về số lượng hòn đảo
}

fn dfs(grid: &Vec<Vec<char>>, i: usize, j: usize) {
    let m = grid.len();  // Số hàng của ma trận
    let n = grid[0].len();  // Số cột của ma trận

    // Kiểm tra điều kiện dừng đệ quy
    if i < 0 || j < 0 || i >= m || j >= n || grid[i][j] != '1' {
        return;
    }

    // Đánh dấu ô đã được duyệt bằng '0'
    grid[i][j] = '0';

    // Duyệt qua các ô kề (trên, dưới, trái, phải)
    dfs(grid, i - 1, j);  // Ô phía trên
    dfs(grid, i + 1, j);  // Ô phía dưới
    dfs(grid, i, j - 1);  // Ô bên trái
    dfs(grid, i, j + 1);  // Ô bên phải
}

fn main() {
    let grid: Vec<Vec<char>> = vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ];

    let result = num_islands(grid);
    println!("Số lượng hòn đảo: {}", result);
}
