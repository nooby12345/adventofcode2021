

#[cfg(test)]
mod problems {
    use crate::common::get_input;

    #[test]
    fn part1() {
        let numbers: Vec<i32> = get_input(include_str!("input/day1.txt"));
        let increased_count: i32 = numbers
            .windows(2)
            .filter(|w| w[1] > w[0])
            .count()
            .try_into()
            .unwrap();
        println!("Increased Count: {}", increased_count);
    }

    #[test]
    fn part2() {
        let numbers: Vec<i32> = get_input(include_str!("input/day1.txt"));
        let increased_count: i32 = numbers
            .windows(4)
            .filter(|w| w[3] > w[0])
            .count()
            .try_into()
            .unwrap();
        println!("Increased Count: {}", increased_count);
    }
}