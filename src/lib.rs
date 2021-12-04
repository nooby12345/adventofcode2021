mod common;
mod day01;

pub fn fake() -> Vec<i32> {
    let input_data = "1\n2\n3";

    let vec = common::get_input(input_data);
    vec
}

#[cfg(test)]
mod input_tests {
    use crate::fake;

    #[test]
    fn get_input() {
        let result = fake();
        assert_eq!(result, vec![1, 2, 3]);
    }
}