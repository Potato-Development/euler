fn main() {
    let mut sum = 0u32;
    let mut a = 1u32;
    let mut b = 1u32;
    let mut c = a + b;

    while c < 4000000 {
        sum += c;
        a = b + c;
        b = c + a;
        c = a + b;
    }

    println!("{}", sum);
}
