use std::io::Read;

fn compute(mut program: Vec<i32>, p1: i32, p2: i32) -> i32 {
    program[1] = p1;
    program[2] = p2;

    let mut idx = 0;
    loop {
        let (opcode, a, b, c) = (
            *program.get(idx).unwrap_or(&0) as usize,
            *program.get(idx + 1).unwrap_or(&0) as usize,
            *program.get(idx + 2).unwrap_or(&0) as usize,
            *program.get(idx + 3).unwrap_or(&0) as usize,
        );

        match opcode {
            1 => program[c] = program[a] + program[b],
            2 => program[c] = program[a] * program[b],
            99 => break,
            _ => panic!("Invalid opcode"),
        }

        idx += 4;
    }

    program[0]
}

fn main() {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read input");

    let program = input
        .split(',')
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>();

    let (noun, verb) = (0..=99)
        .flat_map(|noun| std::iter::repeat(noun).zip(0..=99))
        .find(|(noun, verb)| compute(program.clone(), *noun, *verb) == 19690720)
        .expect("No solution");

    println!("{}", 100 * noun + verb);
}
