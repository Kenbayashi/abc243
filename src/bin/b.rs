fn main() {
    proconio::input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
    };

    let tuples = a.into_iter()
                  .zip(b.iter())
                  .collect::<Vec<(u64, &u64)>>();

    let hit = tuples.iter()
                    .map(|&(a_num, &b_num)| if a_num == b_num {1} else {0})
                    .sum::<u64>();

    let a_blow = tuples.into_iter()
                       .filter(|&(a_num, &b_num)| a_num != b_num)
                       .map(|(a_num, _)| a_num)
                       .collect::<Vec<u64>>();

    let blow = a_blow.into_iter()
                     .map(|num| if b.contains(&num) {1} else {0})
                     .sum::<u64>();

    println!("{}", hit);
    println!("{}", blow);
}
