use std::io::Read;

use itertools::Itertools;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("Couldn't read from stdin");

    let mut parts = input.split('-')
        .flat_map(|s| s.trim().parse::<i64>());

    let (low, high) = (parts.next().unwrap(), parts.next().unwrap());

    let num = (low..high)
        .filter(|i| {
            let s = i.to_string();
            let bytes = s.as_bytes();

            let mut last = 0;
            let increasing = !bytes
                .iter()
                .any(|&c| {
                    let res = c < last;
                    last = c;
                    res
                });
                
            let groups = s.bytes()
                .group_by(|&b| b)
                .into_iter()
                .any(|(_, group)| group.count() == 2);

            increasing && groups
        })
        .count();

    println!("{}", num);
}
