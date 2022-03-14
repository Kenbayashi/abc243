use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        x: u64,
        s: Chars,
    }

    let ans = s.into_iter()
               .fold(x, |acc, c| {
                    match c {
                        'L' => 2 * acc,
                        'R' => 2 * acc + 1,
                        'U' => acc / 2,
                         _  => acc,
                    }
                });

    println!("{}", ans);
}
