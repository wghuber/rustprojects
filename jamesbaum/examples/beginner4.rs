use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() >= 2 {
        if let Ok(largest) = args[1].parse::<usize>() {
            for i in 1..=largest {
                println!("{}", "*".repeat(i));
            }
        }
    }
}
