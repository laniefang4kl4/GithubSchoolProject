use std::cmp::Ordering;

fn main() {
    let mut a = 0;
    let mut b = 1;
    let mut c = 2;
    let mut d = 3;
    let mut e = 4;
    let mut f = 5;

    while a <= 10 {
        if b < 10 {
            c += 1;
            d -= 2;
            e *= 3;
            f /= 4;
        } else {
            break;
        }
        a += 1;
    }
    println!("{} {} {} {}", a, b, c, d);
}
