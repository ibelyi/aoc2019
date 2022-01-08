use super::common::Step;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![("test_input.txt", 30.to_string())],
        Step::Second => vec![],
    }
}

fn count(data: &[i32]) -> i32 {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(data, noun, verb) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn run(data: &[i32], noun: i32, verb: i32) -> i32 {
    let mut opcode: Vec<i32> = data.to_owned();
    opcode[1] = noun;
    opcode[2] = verb;
    let mut i = 0;
    while i < opcode.len() {
        if opcode[i] == 99 {
            break;
        }
        let idx1 = opcode[i + 1] as usize;
        let idx2 = opcode[i + 2] as usize;
        let idx3 = opcode[i + 3] as usize;
        if idx1 >= opcode.len() || idx2 >= opcode.len() || idx3 >= opcode.len() {
            return 0;
        }
        opcode[idx3] = if opcode[i] == 1 {
            opcode[idx1] + opcode[idx2]
        } else if opcode[i] == 2 {
            opcode[idx1] * opcode[idx2]
        } else {
            return 0;
        };
        i += 4;
    }
    opcode[0]
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<i32> = input[0]
        .split(',')
        .map(|v| v.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => run(&data, 12, 2).to_string(),
        Step::Second => count(&data).to_string(),
    }
}
