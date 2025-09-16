use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
    }

    let num = 100 * r + 10 * g + b;

    let ans = if num % 4 == 0 { "YES" } else { "NO" };

    println!("{}", ans)
}