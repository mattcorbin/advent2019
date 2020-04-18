const LOWER_BOUND: usize = 145852;
const UPPER_BOUND: usize = 616942;

fn count_chars(chars: Vec<u8>) -> Vec<(u8, i32)> {
    let mut counts = Vec::new();
    for i in 0..chars.len() {
        if i == 0 {
            counts.push((chars[i], 1))
        } else {
            let mut curr = counts.pop().unwrap();
            if chars[i] == curr.0 {
                curr.1 += 1;
                counts.push(curr);
            } else {
                counts.push(curr);
                counts.push((chars[i], 1));

            }
        }
    }
    counts
}

fn has_dupes(item: &String) -> bool {
    let counts = count_chars(item.clone().into_bytes());
    counts.into_iter().any(|x| x.1 >= 2)
}

fn ascending(item: &String) -> bool {
    let digits: Vec<u32> = item.chars().map(|c| c.to_digit(10).unwrap()).collect();
    for i in 0..item.len() {
        if i+1 < item.len() {
            if digits[i] > digits[i+1] {
                return false
            }
        }
    }
    true
}

fn exactly_two(item: &String) -> bool {
    let counts= count_chars(item.clone().into_bytes());
    counts.into_iter().any(|x| x.1 == 2)
}

fn part1() -> usize {
    let mut count = 0;
    for item in LOWER_BOUND..UPPER_BOUND+1 {
        let str = item.to_string();
        if has_dupes(&str) && ascending(&str) {
            count += 1;
        }
    }
    count
}

fn part2() -> usize {
    let mut count = 0;
    for item in LOWER_BOUND..UPPER_BOUND+1 {
        let str = item.to_string();
        if exactly_two(&str) && ascending(&str) {
            count += 1;
        }
    }
    count
}

fn main() {
    println!("part1 : {}", part1());
    println!("part2 : {}", part2());
}
