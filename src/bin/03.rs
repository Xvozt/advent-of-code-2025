advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for num_str in input.lines() {
        let mut max_number: u64 = 0;

        for (i, first_char) in num_str.chars().enumerate() {
            for second_char in num_str.chars().skip(i + 1) {
                if let (Some(a), Some(b)) = (first_char.to_digit(10), second_char.to_digit(10)) {
                    let number = a * 10 + b;
                    max_number = max_number.max(number as u64)
                }
            }
        }
        result += max_number;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
