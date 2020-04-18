use std::fs::File;
use std::io::{BufReader, BufRead};

fn read_input(reader: BufReader<File>) -> Vec<i32> {
    let mut retval = Vec::new();
    for line in reader.lines() {
        retval.push(line.unwrap().parse::<i32>().unwrap());
    }
    retval
}

fn part1(masses: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for mass in masses {
        let fuel_required = (mass / 3) - 2;
        sum += fuel_required;
    }
    return sum
}

fn part2(masses: &Vec<i32>) -> i32 {
    let mut sum = 0;

    for mass in masses {
        let mut fuel_required = (mass / 3) - 2;
        while fuel_required > 0 {
            sum += fuel_required;
            fuel_required = (fuel_required / 3) - 2;
        }
    }
    return sum
}

fn main() {
    let file = File::open("./input.txt").expect("file not found");
    let masses = read_input(BufReader::new(file));
    println!("part 1: {}", part1(&masses));
    println!("part 2: {}", part2(&masses));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&vec![12]), 2);
        assert_eq!(part1(&vec![14]), 2);
        assert_eq!(part1(&vec![1969]), 654);
        assert_eq!(part1(&vec![100756]), 33583);
        assert_eq!(part1(&vec![12, 14, 1969, 100756]), 34241);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![12]), 2);
        assert_eq!(part2(&vec![14]), 2);
        assert_eq!(part2(&vec![1969]), 966);
        assert_eq!(part2(&vec![100756]), 50346);
        assert_eq!(part2(&vec![12, 14, 1969, 100756]), 51316);
    }
}