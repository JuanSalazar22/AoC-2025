pub fn solve(input: &str) {
    println!("Day 03");
    println!("  Part 1: {}", part1(input));
    println!("  Part 2: {}", part2(input));
}

fn part1(_input: &str) -> i64 {
    // TODO: Implement solution
    0
}

fn part2(_input: &str) -> i64 {
    // TODO: Implement solution
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "987654321111111
        811111111111119
        234234234234278
        818181911112111";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}

