use std::str::FromStr;

pub fn get_input(input_data: &str) -> Vec<i32> {
    input_data.split('\n')
        .into_iter()
        .map(|s| { i32::from_str(s).unwrap()})
        .collect()
}
