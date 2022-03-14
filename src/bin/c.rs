use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        zs: [(i64, i64); n],
        s: Chars,
    };

    let perms = zs.into_iter()
                  .zip(s.into_iter())
                  .map(|((x, y), c)| (y, (x, c)))
                  .collect::<Vec<(i64, (i64, char))>>();

    let mut collision = false;
    let mut map = HashMap::<i64, Vec<(i64, char)>>::new();

    perms.into_iter()
         .for_each(|(y, tuple)| map.entry(y).or_default().push(tuple));

    for (_, mut xs) in map {
        if xs.len() <= 1 {
            continue;
        }

        while let Some((x, c)) = xs.pop() {
            match c {
                'L' => {
                    collision = collision || xs.iter()
                                               .any(|&(xs_x, xs_c)| xs_x < x && xs_c == 'R');
                },
                
                'R' => {
                    collision = collision || xs.iter()
                                               .any(|&(xs_x, xs_c)| x < xs_x && xs_c == 'L');
                },

                _ => (),
            }
        }
    }

    println!("{}", if collision {"Yes"} else {"No"});
}
