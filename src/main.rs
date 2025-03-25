use rlox::runner;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.len() {
        1 => {
            runner::run_prompt().unwrap();
        }
        2 => {
            runner::run_file(&args[1]).unwrap();
        }
        _ => {
            eprintln!("Usage: rlox [script]");
            std::process::exit(64);
        }
    }
}
