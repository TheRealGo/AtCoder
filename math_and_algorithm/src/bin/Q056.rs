use proconio::input;
const MOD: u64 = 1000000007;

fn main() {
    input! { n: u64 }

    println!("{}", recurrence_formula(n));
}

fn matrix_multiply(a: [[u64; 3]; 3], b: [[u64; 3]; 3]) -> [[u64; 3]; 3] {
    [
        [
            (a[0][0] * b[0][0] + a[0][1] * b[1][0] + a[0][2] * b[2][0]) % MOD,
            (a[0][0] * b[0][1] + a[0][1] * b[1][1] + a[0][2] * b[2][1]) % MOD,
            (a[0][0] * b[0][2] + a[0][1] * b[1][2] + a[0][2] * b[2][2]) % MOD,
        ],
        [
            (a[1][0] * b[0][0] + a[1][1] * b[1][0] + a[1][2] * b[2][0]) % MOD,
            (a[1][0] * b[0][1] + a[1][1] * b[1][1] + a[1][2] * b[2][1]) % MOD,
            (a[1][0] * b[0][2] + a[1][1] * b[1][2] + a[1][2] * b[2][2]) % MOD,
        ],
        [
            (a[2][0] * b[0][0] + a[2][1] * b[1][0] + a[2][2] * b[2][0]) % MOD,
            (a[2][0] * b[0][1] + a[2][1] * b[1][1] + a[2][2] * b[2][1]) % MOD,
            (a[2][0] * b[0][2] + a[2][1] * b[1][2] + a[2][2] * b[2][2]) % MOD,
        ],
    ]
}

fn matrix_power(mut matrix: [[u64; 3]; 3], mut n: u64) -> [[u64; 3]; 3] {
    let mut result = [[1, 0, 0], [0, 1, 0], [0, 0, 1]];
    while n > 0 {
        if n & 1 == 1 {
            result = matrix_multiply(result, matrix);
        }
        matrix = matrix_multiply(matrix, matrix);
        n >>= 1;
    }
    result
}

fn recurrence_formula(n: u64) -> u64 {
    let result = matrix_power([[1, 1, 1], [1, 0, 0], [0, 1, 0]], n - 1);
    result[0][0]
}