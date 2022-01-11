use std::collections::HashMap;

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
    opcode: Vec<i64>,
    index: usize,
    base: i64,
    writes: HashMap<usize, i64>,
}

impl Intcode {
    pub fn parse(input: &str) -> Intcode {
        let data: Vec<i64> = input
            .split(',')
            .map(|v| v.parse().expect("Not a number!"))
            .collect();
        Intcode::new(&data)
    }

    pub fn new(data: &[i64]) -> Intcode {
        Intcode {
            opcode: data.to_owned(),
            index: 0,
            base: 0,
            writes: HashMap::new(),
        }
    }

    fn address(&self, offset: usize, modes: &[i64]) -> usize {
        assert_ne!(modes[offset], 1);
        (if modes[offset] == 0 {
            self.opcode[self.index + offset + 1]
        } else {
            assert_eq!(modes[offset], 2);
            self.opcode[self.index + offset + 1] + self.base
        }) as usize
    }

    fn write(&mut self, offset: usize, modes: &[i64], data: i64) {
        let idx = self.address(offset, modes);
        if idx < self.opcode.len() {
            self.opcode[idx] = data;
        } else {
            self.writes.insert(idx, data);
        };
    }

    fn read(&self, offset: usize, modes: &[i64]) -> i64 {
        let idx = if modes[offset] == 1 {
            self.index + offset + 1
        } else {
            self.address(offset, modes)
        };
        if idx < self.opcode.len() {
            self.opcode[idx]
        } else if let Some(v) = self.writes.get(&idx) {
            *v
        } else {
            0
        }
    }

    pub fn run(&mut self, input: &mut Option<i64>) -> Option<i64> {
        while self.index < self.opcode.len() {
            if self.opcode[self.index] == 99 {
                self.index = self.opcode.len();
                return None;
            }
            let opc = self.opcode[self.index] % 100;
            let mut m = self.opcode[self.index] / 10;
            let modes: Vec<i64> = (0..3)
                .map(move |_| {
                    m /= 10;
                    m % 10
                })
                .collect();
            #[cfg(feature = "trace")]
            {
                let fields = if opc == 3 || opc == 4 || opc == 9 {
                    2
                } else if opc == 5 || opc == 6 {
                    3
                } else {
                    4
                };
                println!(
                    "[{}]> {}",
                    self.index,
                    self.opcode
                        .iter()
                        .skip(self.index)
                        .take(fields)
                        .map(|v| v.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                );
            }
            // Jumps
            if opc == 5 || opc == 6 {
                if (self.read(0, &modes) == 0) == (opc == 6) {
                    self.index = self.read(1, &modes) as usize;
                } else {
                    self.index += 3;
                }
            // Base
            } else if opc == 9 {
                self.base += self.read(0, &modes);
                self.index += 2;
            // Input
            } else if opc == 3 {
                if let Some(inp) = input {
                    self.write(0, &modes, *inp);
                    self.index += 2;
                    *input = None;
                } else {
                    return None;
                }
            // Output
            } else if opc == 4 {
                let output = Some(self.read(0, &modes));
                self.index += 2;
                return output;
            } else {
                let val1 = self.read(0, &modes);
                let val2 = self.read(1, &modes);
                self.write(
                    2,
                    &modes,
                    if opc == 1 {
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
                    },
                );
                self.index += 4;
            }
        }
        panic!("Ran out of instructions!")
    }

    pub fn halted(&self) -> bool {
        self.index >= self.opcode.len()
    }
}

/// Simple use of Intcode
///
/// # Example
/// ```
/// use aoc2019::common::intcode_run;
/// let data = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
/// let result = intcode_run(&data, &[]);
/// assert_eq!(result, 99);
/// let data = vec![1102,34915192,34915192,7,4,7,99,0];
/// let result = intcode_run(&data, &[]);
/// assert_eq!(result, 1219070632396864);
/// let data = vec![104,1125899906842624,99];
/// let result = intcode_run(&data, &[]);
/// assert_eq!(result, 1125899906842624);
/// let data = vec![109,2,209,-1,204,0,99];
/// let result = intcode_run(&data, &[]);
/// assert_eq!(result, 204);
/// ```
pub fn intcode_run(data: &[i64], input: &[i64]) -> i64 {
    let mut output = Vec::new();
    let mut intcode = Intcode::new(data);
    let mut i = 0;
    let mut inp = None;
    loop {
        if let Some(val) = intcode.run(&mut inp) {
            output.push(val);
        } else if intcode.halted() {
            break;
        } else if i < input.len() {
            inp = Some(input[i]);
            i += 1;
        } else {
            panic!("No input when one is expected!");
        }
    }
    if i < input.len() {
        panic!("Not all input is used!");
    }
    #[cfg(feature = "trace")]
    println!(
        "Output: {}",
        output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(", ")
    );
    output[output.len() - 1]
}

pub fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 || a == b {
        a
    } else if a < b {
        gcd(a, b % a)
    } else {
        gcd(b, a % b)
    }
}
