use proconio::input;

fn main() {
    input! {
        h: usize, w: usize,
        a_matrix: [[u32; w]; h],
    }

    let mut row_sum: Vec<u32> = vec![0; h];
    let mut col_sum: Vec<u32> = vec![0; w];
    for i in 0..h {
        for j in 0..w {
            row_sum[i] += a_matrix[i][j];
            col_sum[j] += a_matrix[i][j];
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{} ", row_sum[i] + col_sum[j] - a_matrix[i][j]);
        }
        println!();
    }
}