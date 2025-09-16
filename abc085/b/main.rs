use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize, // 長さ
        a: [u32; n]
    }

    let ans: BTreeSet<u32> = a.into_iter().collect();

    println!("{}", ans.len())
}