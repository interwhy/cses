use std::env;

mod task;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("need task number");
    }
    if args.len() > 2 {
        panic!("too many arguments");
    }

    let task_number = args[1].parse::<usize>().unwrap_or(0);
    match task_number {
        1068 => task::weird_algorithm::main(),
        1069 => task::repetitions::main(),
        1083 => task::missing_no::main(),
        1624 => task::queens::main(),
        _ => panic!("unknown task"),
    }
}
