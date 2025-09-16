use proconio::input;

fn main()  {
    input! {
        n: usize,
        a: usize,
    }

    let mut ans = "No";
    for i in 0..=a {
        if n >=i && (n - i) % 500 == 0 {
            ans = "Yes";
            break;            
        }
    }
    println!("{}", ans)

}