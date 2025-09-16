use proconio::input;

fn main() {
    input! {
        a: String
    }

    println!(
        "{}",
        a.chars().filter(|&c| c == '1').count()
    );
}