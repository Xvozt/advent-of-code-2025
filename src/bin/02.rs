advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        input
            .split(',')
            .map(|range_string| {
                let (start, end) = range_string.split_once('-').unwrap();
                (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
            })
            .flat_map(|(start, end)| start..=end)
            .filter(|&num| {
                let s = num.to_string();
                let len = s.len();
                if len % 2 != 0 {
                    return false;
                }
                &s[0..len / 2] == &s[len / 2..]
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(
        input
            .split(',')
            .map(|range_string| {
                let (start, end) = range_string.split_once('-').unwrap();
                (start.parse::<u64>().unwrap(), end.parse::<u64>().unwrap())
            })
            .flat_map(|(start, end)| start..=end)
            .filter(|&num| {
                let s = num.to_string();
                let len = s.len();

                for pattern_len in 1..=len / 2 {
                    if len % pattern_len != 0 {
                        continue;
                    }
                    let pattern = &s[0..pattern_len];
                    if pattern.repeat(len / pattern_len) == s {
                        return true;
                    }
                }
                false
            })
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
