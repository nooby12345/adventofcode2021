use error::AOCError;
use std::fs;
use std::path::Path;
use std::str::FromStr;
use error::AOCError::GeneralError;

pub fn problem_main() -> Result<(), AOCError> {
    println!("Running problem2::problem_main()");
    let path_buf = Path::new("problem1").join("input.txt");
    match fs::read_to_string(path_buf) {
        Ok(data) => {
            let mut input: Vec<i32> = vec![];
            for line in data.split('\n') {
                if let Ok(number) = i32::from_str(line) {
                    input.push(number);
                } else {
                    return Err(GeneralError("Unable to parse problem2 input".to_string()));
                }
            }

            let window_size = 3;
            let mut increased_count = 0;
            if input.len() > window_size {
                let mut previous = input[0] + input[1] + input[2];
                let mut current: i32;
                for index in 1..(input.len() - window_size + 1) {
                    current = input[index..(index + window_size)].iter().sum();
                    if current > previous {
                        increased_count += 1;
                    }
                    previous = current;
                }
            }
            println!("Increased Count is {}", increased_count)
        }
        Err(error) => {
            eprintln!("Error reading file {:?}", error);
        }
    }
    Ok(())
}