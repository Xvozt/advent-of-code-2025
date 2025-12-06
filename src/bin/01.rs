advent_of_code::solution!(1);

fn parse_rotations(input: &str) -> Vec<i16> {
    input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| {
            let direction = s.chars().next().unwrap();
            let number = s[1..].parse::<i16>().unwrap();
            match direction {
                'L' => -number,
                'R' => number,
                _ => unreachable!(),
            }
        })
        .collect::<Vec<i16>>()
}

pub fn part_one(input: &str) -> Option<u64> {
    let numbers = parse_rotations(input);

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

pub fn part_two(input: &str) -> Option<u64> {
    let numbers = parse_rotations(input);

    let mut position = 50i16;
    let mut count = 0u64;

    for &rotation in &numbers {
        let distance = rotation.abs();

        let full_circles = distance / 100;
        count += full_circles as u64;

        let remainder = distance % 100;

        if rotation > 0 {
            if position + remainder > 100 {
                count += 1;
            }
        } else if rotation < 0 {
            if position != 0 && position - remainder < 0 {
                count += 1;
            }
        }

        position = (position + rotation).rem_euclid(100);
        if position == 0 {
            count += 1;
        }
    }

    return Some(count);
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
        assert_eq!(result, Some(6));
    }
}
