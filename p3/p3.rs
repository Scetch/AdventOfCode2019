use std::collections::HashSet;
use std::io::Read;

fn make_wire<'a, I>(i: I) -> Vec<(i32, i32)>
where
    I: Iterator<Item = &'a str>
{
    let mut pos = (0, 0);
    let mut positions = vec![(0, 0)];

    for step in i {
        let mut chars = step.chars();
        let direction = chars.next().unwrap();
        let mut steps = chars.as_str().parse::<u32>().unwrap();

        while steps > 0 {
            match direction {
                'U' => pos.1 += 1,
                'R' => pos.0 += 1,
                'D' => pos.1 -= 1,
                'L' => pos.0 -= 1,
                _ => panic!("??"),
            }

            positions.push(pos);

            steps -= 1;
        }
    }

    positions
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("Couldn't read from stdin");

    let mut wires = input
        .lines()
        .map(|line| line.split(','))
        .map(make_wire);

    let a_positions = wires.next().expect("Couldn't get wire a");
    let b_positions = wires.next().expect("Couldn't get wire b");

    let (mut a_set, mut b_set) = (HashSet::new(), HashSet::new());
    a_set.extend(a_positions.iter().skip(1));
    b_set.extend(b_positions.iter().skip(1));

    let min = a_set.intersection(&b_set)
        .into_iter()
        .map(|intersection| {
            a_positions.iter().position(|p| p == intersection).unwrap()
                + b_positions.iter().position(|p| p == intersection).unwrap()
        })
        .min()
        .unwrap_or(0);

    println!("{}", min);
}
