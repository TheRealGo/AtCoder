use proconio::input;
const MOD: i64 = 1000000007;

fn main() {
    input! { k: u64, n: u64 }

    if k == 3 && n % 2 == 1 {
        println!("0");
        return;
    }

    let matrix_pow = match k {
        // a_n = a_(n - 1) + a_(n - 2)
        2 => matrix_power(
            vec![vec![1, 1], vec![1, 0]],
            n - k + 1),
        // a_n = 4a_(n - 1) - a_(n - 2)
        3 => matrix_power(
            vec![vec![4, -1], vec![1, 0]],
            (n - 4) / 2),
        // a_n = a_(n - 1) + 5 * a_(n - 2) + a_(n - 3) - a_(n - 4)
        4 => matrix_power(
            vec![vec![1, 5, 1, -1], vec![1, 0, 0, 0], vec![0, 1, 0, 0], vec![0, 0, 1, 0]],
            n - k),
        _ => unreachable!(),
    };

    let result = match k {
        // a1 = 1, a2 = 1
        2 => matrix_multiply(
            &matrix_pow,
            &vec![vec![1, 0], vec![1, 0]],
        ),
        // a1 = 3, a2 = 11
        3 => matrix_multiply(
            &matrix_pow,
            &vec![vec![11, 0], vec![3, 0]],
        ),
        // a1 = 1, a2 = 5, a3 = 11, a4 = 36
        4 => matrix_multiply(
            &matrix_pow,
            &vec![vec![36, 0, 0, 0], vec![11, 0, 0, 0], vec![5, 0, 0, 0], vec![1, 0, 0, 0]],
        ),
        _ => unreachable!(),
    };
    println!("{}", (MOD + result[0][0]) % MOD);
}

fn matrix_multiply(matrix1: &Vec<Vec<i64>>, matrix2: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let n = matrix1.len();
    let mut result = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] = (result[i][j] + matrix1[i][k] * matrix2[k][j]) % MOD;
            }
        }
    }
    result
}

fn matrix_power(mut matrix: Vec<Vec<i64>>, mut n: u64) -> Vec<Vec<i64>> {
    let d = matrix.len();
    let mut result: Vec<Vec<i64>> = vec![vec![0; d]; d];
    for i in 0..d {
        result[i][i] = 1;
    }

    while n > 0 {
        if n & 1 == 1 {
            result = matrix_multiply(&result, &matrix);
        }
        matrix = matrix_multiply(&matrix, &matrix);
        n >>= 1;
    }
    result
}