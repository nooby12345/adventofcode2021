fn main() {
    let problem_number = 2;
    let result = match problem_number {
        1 => problem1::problem_main(),
        2 => problem2::problem_main(),
        _ => unimplemented!("Problem {} not implemented", problem_number)
    };
    match result {
        Ok(()) => {
            println!("Problem {} ran successfully", problem_number)
        },
        Err(error) => {
            eprintln!("Problem {} failed with error: {}", problem_number, error)
        }
    }
}
