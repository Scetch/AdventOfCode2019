
fn compute(mut program: Vec<i32>) {
    let mut idx = 0;
    loop {
        let opcode = program[idx] % 100;

        let params = (0..3)
            .map(|i| (program[idx] / 10_i32.pow(2 + i as u32)) % 10)
            .collect::<Vec<_>>();

        let param = |n| {
            if params[n] == 0 {
                program[program[idx + n + 1] as usize]
            } else {
                program[idx + n + 1]
            }
        };

        match opcode {
            1 => {
                let (a, b, c) = (param(0), param(1), program[idx + 3] as usize);
                program[c] = a + b;
                idx += 4;
            }
            2 => {
                let (a, b, c) = (param(0), param(1), program[idx + 3] as usize);
                program[c] = a * b;
                idx += 4;
            }
            3 => {
                let a = program[idx + 1] as usize;
                
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                
                let input = input.trim().parse::<i32>()
                    .expect("Parse error");

                program[a] = input;
                idx += 2;
            }
            4 => {
                let a = program[idx + 1] as usize;
                println!("{}", program[a]);
                idx += 2;
            }
            5 | 6 => {
                let (a, b) = (param(0), param(1));

                idx = match opcode {
                    5 if a != 0 => b as usize,
                    6 if a == 0 => b as usize,
                    _ => idx + 3,
                };
            }
            7 | 8 => {
                let (a, b, c) = (param(0), param(1), program[idx + 3] as usize);

                program[c] = match opcode {
                    7 if a < b => 1,
                    7 if a >= b => 0,
                    8 if a == b => 1,
                    8 if a != b => 0,
                    _ => program[c],
                };

                idx += 4;
            }
            99 => break,
            opcode => panic!("Invalid opcode {}", opcode),
        }
    }
}

fn main() {
    let input = std::fs::read_to_string("p5.input")
        .expect("Couldn't read input");

    let program = input
        .split(',')
        .flat_map(str::parse::<i32>)
        .collect::<Vec<_>>();

    compute(program);
}
