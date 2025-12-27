pub fn solve(input: &str) {
    println!("Day 01");
    println!("  Part 1: {}", part1(input));
    println!("  Part 2: {}", part2(input));
}

fn part1(_input: &str) -> i64 {
    let min = 0;
    let max = 99;

    let mut total_zeroes = 0;
    let mut dial_position = 50;

    // println!("the dial starts by pointing at {initial_dial_position}");

    let lines = _input.lines();
    for line in lines {
        let mut direction = 0;
        let mut new_line = line.to_string();
        if new_line.contains("R"){
            direction = 1;
            new_line = new_line.replace("R", "");
        } else if new_line.contains("L"){
            direction = -1;
            new_line = new_line.replace("L", "");
        }

        let steps = new_line.parse::<i64>().unwrap() % 100;

        let mut new_dial_position = dial_position + (direction * steps);

        if new_dial_position < min {
            new_dial_position = new_dial_position + 100;
        } else if new_dial_position > max {
            new_dial_position = new_dial_position - 100;
        }

        // println!("Started at: {dial_position} The dial is rotated {line} to point at {steps} steps to the {direction} direction, now pointing at {new_dial_position}");

        dial_position = new_dial_position;

        if new_dial_position == min {
            total_zeroes = total_zeroes + 1;
        }

    }

    return total_zeroes;
}

fn part2(_input: &str) -> i64 {
    let min = 0;
    let max = 99;

    let mut total_zeroes = 0;
    let mut dial_position = 50;

    // println!("the dial starts by pointing at {initial_dial_position}");

    let lines = _input.lines();
    for line in lines {
        let mut direction = 0;
        let mut new_line = line.to_string();
        if new_line.contains("R"){
            direction = 1;
            new_line = new_line.replace("R", "");
        } else if new_line.contains("L"){
            direction = -1;
            new_line = new_line.replace("L", "");
        }

        let laps: i64 = new_line.parse::<i64>().unwrap() / 100;
        let steps = new_line.parse::<i64>().unwrap() % 100;

        let mut new_dial_position = dial_position + (direction * steps);
        let mut times_it_points_to_zero = laps;

        if new_dial_position < min {
            new_dial_position = new_dial_position + 100;
            if dial_position != 0 {
                times_it_points_to_zero = times_it_points_to_zero + 1;
            }
        } else if new_dial_position > max {
            new_dial_position = new_dial_position - 100;
            if dial_position != 100 {
                times_it_points_to_zero = times_it_points_to_zero + 1;
            }
        } else if new_dial_position == min {
            if dial_position != 0 {
                total_zeroes = total_zeroes + 1;
            }
        }

        total_zeroes = total_zeroes + times_it_points_to_zero;

        // println!("The dial is rotated {line} to point at {new_dial_position}; during this rotation, it points to zero {times_it_points_to_zero} times, for a total of {total_zeroes} times");

        //println!("Started at: {dial_position} The dial is rotated {line}, {laps} times, to point at {steps} steps to the {direction} direction, now pointing at {new_dial_position}");

        dial_position = new_dial_position;

    }

    return total_zeroes;
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}

