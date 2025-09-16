use proconio::input;

fn main() {
    input! {
        n: usize,
        mut txy: [(i32, i32, i32); n]
    }

    txy.insert(0,(0,0,0));

    let mut ok = true;
    for i in 0..n {
        let (t_prev, x_prev, y_prev) = txy[i];
        let (t_curr, x_curr, y_curr) = txy[i + 1];

        let dt = t_curr - t_prev;
        let dist = (x_curr - x_prev).abs() + (y_curr - y_prev).abs();

        if dt < dist || (dt - dist) % 2 != 0 {
            ok = false;
            break;
        }
    }
    
    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}