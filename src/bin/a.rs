fn main() {
    proconio::input! {
        mut v: u64,
        a: u64,
        b: u64,
        c: u64,
    };

    loop {
        if a <= v {
            v -= a;
        } else {
            println!("F");
            break;
        }

        if b <= v {
            v -= b;
        } else {
            println!("M");
            break;
        }

        if c <= v {
            v -= c;
        } else {
            println!("T");
            break;
        }
    }
}
