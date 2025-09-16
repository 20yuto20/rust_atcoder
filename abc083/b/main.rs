use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut ans = 0;
    for i in 1..n + 1 {
        let mut dig = i;
        let mut sum = 0;

        while dig > 0 {
            sum += dig % 10; // 123という数字だとここで3がまず取り出されるそしてそれが各位の合計のsumに足される
            dig /= 10; // ここは商を出すことで次のくらいを上のやつで割れるようにする
            if dig == 0 {
                break;
            }
        }
        if a <= sum && sum <= b {
            ans += i;
        }
    }

    println!("{}", ans)

}