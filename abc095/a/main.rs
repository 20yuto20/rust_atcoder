use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut topping_count = 0;
    for c in s.chars() {
        if c == 'o' {
            topping_count += 1;
        }
    }

    let total_price = 700 + 100 * topping_count;

    println!("{}", total_price);
}