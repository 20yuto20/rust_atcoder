use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let mut s_chars: Vec<char> = s.chars().collect();
    s_chars.sort();

    let mut t_chars: Vec<char> = t.chars().collect();
    t_chars.sort();
    t_chars.reverse();

    let s_min: String = s_chars.into_iter().collect();
    let t_max: String = t_chars.into_iter().collect();

    if s_min < t_max {
        println!("Yes")
    } else {
        println!("No")
    }
}