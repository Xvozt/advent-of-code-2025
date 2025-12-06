advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = input
        .split('\n')
        .map(|s| -> i16 {
            let (direction, number) = (&s[..1], &s[1..]);
            match direction {
                "L" => -number.to_string().parse::<i16>().unwrap(),
                "R" => number.to_string().parse::<i16>().unwrap(),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<i16>>();

    let mut position = 50i16;
    let mut count = 0u64;

    for &rotation in &numbers {
        position = (position + rotation).rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(_input: &str) -> Option<u64> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
