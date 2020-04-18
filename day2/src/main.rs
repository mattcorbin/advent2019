use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_input(input: String) -> Vec<usize> {
    let mut retval = Vec::new();
    for opcode in input.split(",") {
        retval.push(opcode.parse::<usize>().unwrap());
    }
    retval
}

fn execute(opcodes: &Vec<usize>) -> Vec<usize> {
    let mut retval = opcodes.clone();
    let mut pc = 0;
    while retval[pc] != 99 {
        let action = retval[pc];
        let src1 = retval[pc+1];
        let src2 = retval[pc+2];
        let dst = retval[pc+3];
        match action {
            1 => retval[dst] = retval[src1] + retval[src2],
            2 => retval[dst] = retval[src1] * retval[src2],
            _ => ()
        }
        pc += 4
    }
    retval
}

fn main() {
    let file = File::open("./input.txt").expect("file not found");
    let opcodes = parse_input(
        BufReader::new(file)
            .lines()
            .next()
            .expect("should a line")
            .expect("should not error")
    );
    let mut opcodes_p1 = opcodes.clone();
    opcodes_p1[1] = 12;
    opcodes_p1[2] = 2;
    println!("part 1: {}", execute(&opcodes_p1)[0]);

    let mut opcodes_p2 = opcodes.clone();
    let desired_result = 19690720;
    let mut done = false;

    for i in 0..100 {
        for j in 0..100 {
            opcodes_p2[1] = i;
            opcodes_p2[2] = j;
            if execute(&opcodes_p2)[0] == desired_result {
                done = true;
                break;
            }
        }
        if done {
            break;
        }
    }
    println!("part 2: {:02},{:02}", opcodes_p2[1], opcodes_p2[2]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(execute(&vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(execute(&vec![2,3,0,3,99]), vec![2,3,0,6,99]);
        assert_eq!(execute(&vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(execute(&vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}