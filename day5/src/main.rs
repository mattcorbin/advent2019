use std::fs::File;
use std::io::{BufRead, BufReader, stdin};



struct IntcodeCPU {
    pc: usize,
    opcodes: Vec<i32>
}

impl IntcodeCPU {
    pub fn new() -> Self {
        let mut cpu = IntcodeCPU {
            pc: 0,
            opcodes: vec![],
        };
        cpu
    }

    fn parse_input(&mut self, input: String) -> Vec<i32> {
        let mut retval = Vec::new();
        for opcode in input.split(",") {
            retval.push(opcode.parse::<i32>().unwrap());
        }
        retval
    }

    fn parse_command(&mut self, command: i32) -> Vec<String> {
        return format!("{:05}", command).chars().map(|x| x.to_string()).collect();
    }

    fn parse_mode(&mut self, value: i32, mode: &str) -> i32 {
        return if mode == "0" {
            self.opcodes[value as usize]
        } else {
            value
        }
    }

    pub fn execute(&mut self, input: String) -> Vec<i32> {
        let opcodes = self.parse_input(input);
        self.opcodes = opcodes.clone();
        while self.opcodes[self.pc] != 99 {
            let command = self.parse_command(self.opcodes[self.pc]);
            //let mode3 = command[0].as_str();
            let mode2 = command[1].as_str();
            let mode1 = command[2].as_str();
            let opcode = format!("{}{}", command[3], command[4]);
            match opcode.as_str() {
                "01" => self.add(
                    self.opcodes[self.pc+1],
                    mode1,
                    self.opcodes[self.pc+2],
                    mode2,
                    self.opcodes[self.pc+3]
                ),
                "02" => self.mult(
                    self.opcodes[self.pc+1],
                    mode1,
                    self.opcodes[self.pc+2],
                    mode2,
                    self.opcodes[self.pc+3]
                ),
                "03" => self.input(self.opcodes[self.pc+1]),
                "04" => self.output(self.opcodes[self.pc+1]),
                "05" => self.jump_true(
                    self.opcodes[self.pc+1],
                    mode1,
                    self.opcodes[self.pc+2],
                    mode2
                ),
                "06" => self.jump_false(
                    self.opcodes[self.pc+1],
                    mode1,
                    self.opcodes[self.pc+2],
                    mode2
                ),
                "07" => self.less_than(
                    self.opcodes[self.pc+1],
                    mode1,
                    self.opcodes[self.pc+2],
                    mode2,
                    self.opcodes[self.pc+3]
                ),
                "08" => self.equals(
                    self.opcodes[self.pc+1],
                    mode1,
                    self.opcodes[self.pc+2],
                    mode2,
                    self.opcodes[self.pc+3]
                ),
                _ => ()
            }
        }
        self.opcodes.clone()
    }

    fn add(&mut self, src1: i32, mode1: &str, src2: i32, mode2: &str, dst: i32) {
        let a = self.parse_mode(src1, mode1);
        let b = self.parse_mode(src2, mode2);
        self.opcodes[dst as usize] = a + b;
        self.pc += 4;
    }

    fn mult(&mut self, src1: i32, mode1: &str, src2: i32, mode2: &str, dst: i32) {
        let a = self.parse_mode(src1, mode1);
        let b = self.parse_mode(src2, mode2);
        self.opcodes[dst as usize] = a * b;
        self.pc += 4;
    }

    fn input(&mut self, dst: i32) {
        println!("input required");
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        self.opcodes[dst as usize] = s.trim().parse::<i32>().unwrap();
        self.pc += 2;
    }

    fn output(&mut self, src: i32) {
        println!("{}", self.opcodes[src as usize]);
        self.pc += 2;
    }

    fn jump_true(&mut self, src1: i32, mode1: &str, src2: i32, mode2: &str) {
        let test = self.parse_mode(src1, mode1);
        let jmp = self.parse_mode(src2, mode2);
        if test != 0 {
            self.pc = jmp as usize;
        } else {
            self.pc += 3;
        }
    }

    fn jump_false(&mut self, src1: i32, mode1: &str, src2: i32, mode2: &str) {
        let test = self.parse_mode(src1, mode1);
        let jmp = self.parse_mode(src2, mode2);
        if test == 0 {
            self.pc = jmp as usize;
        } else {
            self.pc += 3;
        }
    }

    fn less_than(&mut self, src1: i32, mode1: &str, src2: i32, mode2: &str, dst: i32) {
        let a = self.parse_mode(src1, mode1);
        let b = self.parse_mode(src2, mode2);
        if a < b {
            self.opcodes[dst as usize] = 1;
        } else {
            self.opcodes[dst as usize] = 0;
        }
        self.pc += 4;
    }

    fn equals(&mut self, src1: i32, mode1: &str, src2: i32, mode2: &str, dst: i32) {
        let a = self.parse_mode(src1, mode1);
        let b = self.parse_mode(src2, mode2);
        if a == b {
            self.opcodes[dst as usize] = 1;
        } else {
            self.opcodes[dst as usize] = 0;
        }
        self.pc += 4;
    }
}


fn main() {
    let mut cpu = IntcodeCPU::new();
    let file = File::open("./input.txt").expect("file not found");
    cpu.execute(
        BufReader::new(file)
            .lines()
            .next()
            .expect("should a line")
            .expect("should not error")
    );
}
