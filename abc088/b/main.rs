use proconio::input;

fn main() {
    input! {
        n: usize, // カードの枚数
        mut a: [i32; n]
    }
    a.sort();
    a.reverse();

    let mut sum = 0;

    for (i, val) in a.iter().enumerate() {
        sum += if i % 2 == 0 { val * 1 } else { val * -1 };
    }

    println!("{}", sum)
}