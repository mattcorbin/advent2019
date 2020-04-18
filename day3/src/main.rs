use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_input(reader: BufReader<File>) -> Vec<Vec<String>> {
    let mut retval = Vec::new();
    for line in reader.lines() {
        let mut things = Vec::new();
        for item in line.unwrap().split(",") {
            things.push(item.to_string());
        }
        retval.push(things);
    }
    retval
}

fn remove_first(s: String) -> String {
    let mut chars = s.chars();
    chars.next();
    chars.as_str().to_string()
}

fn find_min_intersection_distance(wires: &Vec<Vec<String>>) -> i32 {
    let mut coords:Vec<Vec<(i32, i32)>> = Vec::new();
    for wire in &*wires {
        let mut wire_points = Vec::new();
        let mut location = (0, 0);
        for action in wire {
            let direction = action.chars().next().unwrap();
            let distance = remove_first(action.clone()).parse::<i32>().unwrap();
            match direction {
                'U' => {
                    for _ in 0..distance {
                        location.1 += 1;
                        wire_points.push(location.clone());
                    }
                },
                'D' => {
                    for _ in 0..distance {
                        location.1 -= 1;
                        wire_points.push(location.clone());
                    }
                },
                'L' => {
                    for _ in 0..distance {
                        location.0 -= 1;
                        wire_points.push(location.clone());
                    }
                },
                'R' =>  {
                    for _ in 0..distance {
                        location.0 += 1;
                        wire_points.push(location.clone());
                    }
                },
                _ => (),
            }
            wire_points.push(location.clone());
        }
        coords.push(wire_points);
    }

    let mut closest_distance = 2147483647;

    for i in 0..coords[0].len() {
        for j in 0..coords[1].len() {
            if coords[0][i].0 ==  coords[1][j].0 && coords[0][i].1 ==  coords[1][j].1 {
                let distance = coords[0][i].0.abs() + coords[0][i].1.abs();
                if distance < closest_distance {
                    closest_distance = distance;
                }
            }
        }
    }

    closest_distance
}

fn find_fewest_steps(wires: &Vec<Vec<String>>) -> i32 {
    let mut coords:Vec<Vec<(i32, i32)>> = Vec::new();
    for wire in &*wires {
        let mut wire_points = Vec::new();
        let mut location = (0, 0);
        for action in wire {
            let direction = action.chars().next().unwrap();
            let distance = remove_first(action.clone()).parse::<i32>().unwrap();
            match direction {
                'U' => {
                    for _ in 0..distance {
                        location.1 += 1;
                        wire_points.push(location.clone());
                    }
                },
                'D' => {
                    for _ in 0..distance {
                        location.1 -= 1;
                        wire_points.push(location.clone());
                    }
                },
                'L' => {
                    for _ in 0..distance {
                        location.0 -= 1;
                        wire_points.push(location.clone());
                    }
                },
                'R' =>  {
                    for _ in 0..distance {
                        location.0 += 1;
                        wire_points.push(location.clone());
                    }
                },
                _ => (),
            }
        }
        coords.push(wire_points);
    }

    let mut least_steps = 2147483647;

    for i in 0..coords[0].len() {
        for j in 0..coords[1].len() {
            if coords[0][i].0 ==  coords[1][j].0 && coords[0][i].1 ==  coords[1][j].1 {
                let steps = (i+1 + j+1) as i32;
                if steps < least_steps {
                    least_steps = steps;
                }
            }
        }
    }

    least_steps
}

fn main() {
    let file = File::open("./input.txt").expect("file not found");
    let directions = read_input(BufReader::new(file));
    println!("part 1: {}", find_min_intersection_distance(&directions));
    println!("part 2: {}", find_fewest_steps(&directions));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_intersection_distance() {
        assert_eq!(
            find_min_intersection_distance(
                &vec![
                    vec!["R75", "D30","R83","U83","L12","D49","R71","U7","L72"]
                        .into_iter().map(|x| x.to_string()).collect(),
                    vec!["U62","R66","U55","R34","D71","R55","D58","R83"]
                        .into_iter().map(|x| x.to_string()).collect()
                ]
            ),
            159
        );
        assert_eq!(
            find_min_intersection_distance(
                &vec![
                    vec!["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"]
                        .into_iter().map(|x| x.to_string()).collect(),
                    vec!["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"]
                        .into_iter().map(|x| x.to_string()).collect()
                ]
            ),
            135
        );
    }
    #[test]
    fn test_find_fewest_steps() {
        assert_eq!(
            find_fewest_steps(
                &vec![
                    vec!["R75", "D30","R83","U83","L12","D49","R71","U7","L72"]
                        .into_iter().map(|x| x.to_string()).collect(),
                    vec!["U62","R66","U55","R34","D71","R55","D58","R83"]
                        .into_iter().map(|x| x.to_string()).collect()
                ]
            ),
            610
        );
        assert_eq!(
            find_fewest_steps(
                &vec![
                    vec!["R98","U47","R26","D63","R33","U87","L62","D20","R33","U53","R51"]
                        .into_iter().map(|x| x.to_string()).collect(),
                    vec!["U98","R91","D20","R16","D67","R40","U7","R15","U6","R7"]
                        .into_iter().map(|x| x.to_string()).collect()
                ]
            ),
            410
        );
    }
}