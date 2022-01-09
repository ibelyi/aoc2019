#[derive(Debug)]
pub enum Step {
    First,
    Second,
}

use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

pub fn lines_from_file(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

#[derive(Clone)]
pub struct Intcode {
    opcode: Vec<i32>,
    index: usize,
}

impl Intcode {
    pub fn new(data: &[i32]) -> Intcode {
        Intcode {
            opcode: data.to_owned(),
            index: 0,
        }
    }

    fn value(&self, offset: usize, modes: &[i32]) -> i32 {
        let idx = if modes[offset] == 0 {
            self.opcode[self.index + offset + 1] as usize
        } else {
            self.index + offset + 1
        };
        self.opcode[idx]
    }

    pub fn run(&mut self, input: &mut Option<i32>) -> Option<i32> {
        while self.index < self.opcode.len() {
            if self.opcode[self.index] == 99 {
                self.index = self.opcode.len();
                return None;
            }
            let opc = self.opcode[self.index] % 100;
            let mut m = self.opcode[self.index] / 10;
            let modes: Vec<i32> = (0..3)
                .map(move |_| {
                    m /= 10;
                    m % 10
                })
                .collect();
            // Jumps
            if opc == 5 || opc == 6 {
                if (self.value(0, &modes) == 0) == (opc == 6) {
                    self.index = self.value(1, &modes) as usize;
                } else {
                    self.index += 3;
                }
            // Input
            } else if opc == 3 {
                assert_eq!(modes[0], 0);
                if let Some(inp) = input {
                    let idx = self.opcode[self.index + 1] as usize;
                    self.opcode[idx] = *inp;
                    self.index += 2;
                    *input = None;
                } else {
                    return None;
                }
            // Output
            } else if opc == 4 {
                let output = Some(self.value(0, &modes));
                self.index += 2;
                return output;
            } else {
                assert_eq!(modes[2], 0);
                let val1 = self.value(0, &modes);
                let val2 = self.value(1, &modes);
                let idx3 = self.opcode[self.index + 3] as usize;
                self.opcode[idx3] = if opc == 1 {
                    val1 + val2
                } else if opc == 2 {
                    val1 * val2
                } else if opc == 7 {
                    if val1 < val2 {
                        1
                    } else {
                        0
                    }
                } else if opc == 8 {
                    if val1 == val2 {
                        1
                    } else {
                        0
                    }
                } else {
                    panic!("Unknow opcode!")
                };
                self.index += 4;
            }
        }
        panic!("Ran out of instructions!")
    }

    pub fn halted(&self) -> bool {
        self.index >= self.opcode.len()
    }
}

pub fn intcode_run(data: &[i32], input: &[i32]) -> i32 {
    let mut output = Vec::new();
    let mut intcode = Intcode::new(data);
    let mut ii = 0;
    let mut inp = None;
    loop {
        if let Some(val) = intcode.run(&mut inp) {
            output.push(val);
        } else if intcode.halted() {
            break;
        } else if ii < input.len() {
            inp = Some(input[ii]);
            ii += 1;
        } else {
            panic!("No input when one is expected!");
        }
    }
    if ii < input.len() {
        panic!("Not all input is used!");
    }
    output[output.len() - 1]
}
