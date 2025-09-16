use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut res = s;
    for t in &["eraser", "erase", "dreamer", "dream"] {
        res = res.replace(t, "")
    }

    if res.is_empty() {
        println!("YES")
    } else {
        println!("NO")
    }
}