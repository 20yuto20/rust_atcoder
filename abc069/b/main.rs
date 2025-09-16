use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let str_len = s.len();
    let first_letter = s.chars().next().unwrap();
    let last_letter = s.chars().last().unwrap();
    let mid_len = str_len - 2;

    println!("{}{}{}", first_letter, mid_len, last_letter);
}