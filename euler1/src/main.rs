fn main() {
    println!("{}", sum_divisible_by(3) + sum_divisible_by(5) - sum_divisible_by(15))
}

fn sum_divisible_by(n: i32) -> i32 {
    let p: i32 = 999 / n;
    n * (p * (p + 1)) / 2
}

