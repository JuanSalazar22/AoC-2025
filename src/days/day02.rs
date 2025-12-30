use std::char;

pub fn solve(input: &str) {
    println!("Day 02");
    println!("  Part 1: {}", part1(input));
    println!("  Part 2: {}", part2(input));
}

fn is_invalid_number(number: i64) -> bool {
    let s: String = number.to_string();
    let chars_list = s.chars().collect::<Vec<char>>();
    let len: usize = chars_list.len() + 1;

    let firts_half_string = chars_list[0..len / 2].iter().collect::<String>();
    let second_half_string = chars_list[len / 2..].iter().collect::<String>();

    if second_half_string.contains(firts_half_string.as_str()) {
        return true;
    }

    false
}

fn is_invalid_number_part_2(number: i64) -> bool {
    let s: String = number.to_string();
    let chars_list: Vec<char> = s.chars().collect::<Vec<char>>();
    let len: usize = chars_list.len();

    if len == 1 { 
        return false;
    }

    // println!("number: {}", number);

    for i in 1..=len / 2 {
        let chunked_array: Vec<&[char]> = chars_list.chunks(i).collect::<Vec<&[char]>>();

        // println!("chunk {i}: chunked_array: {:?}", chunked_array);

        if chunked_array.iter().all(|&item| {
            item.iter().collect::<String>() == chunked_array[0].iter().collect::<String>()
        }) {
            // println!("number {number} is invalid");
            return true;
        }
    }

    false
}

fn part1(_input: &str) -> i64 {
    let ranges = _input
        .split(",")
        .map(|range| {
            range
                .split("-")
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut total_invalid_numbers: Vec<i64> = Vec::new();
    for range in ranges {
        let mut list_of_invalid_numbers: Vec<i64> = Vec::new();

        for i in range[0]..=range[1] {
            if is_invalid_number(i) {
                list_of_invalid_numbers.push(i);
                total_invalid_numbers.push(i);
            }
        }
    }
    // println!("total invalid numbers: {:?}", total_invalid_numbers);
    return total_invalid_numbers.iter().sum::<i64>();
}

fn part2(_input: &str) -> i64 {
    let ranges = _input
        .split(",")
        .map(|range| {
            range
                .split("-")
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();

    let mut total_invalid_numbers: Vec<i64> = Vec::new();
    for range in ranges {
        let mut list_of_invalid_numbers: Vec<i64> = Vec::new();

        for i in range[0]..=range[1] {
            if is_invalid_number_part_2(i) {
                list_of_invalid_numbers.push(i);
                total_invalid_numbers.push(i);
            }
        }
    }
    println!("total invalid numbers: {:?}", total_invalid_numbers);
    return total_invalid_numbers.iter().sum::<i64>();
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 1227775554);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 4174379265);
    }
}
