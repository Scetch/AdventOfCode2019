use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)
        .expect("Couldn't load");

    println!("{}", input.lines()
        .flat_map(str::parse::<i32>)
        .map(|mut i| {
            let mut sum = 0;
            while i > 0 {
                i = ((i / 3) - 2).max(0);
                sum += i;
            }
            sum
        })
        .sum::<i32>());
}
