use super::common::Step;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![],
        Step::Second => vec![],
    }
}

fn count(data: &[i32], id: i32) -> i32 {
    run(&data, &[id])
}

fn run(data: &[i32], input: &[i32]) -> i32 {
    let mut opcode: Vec<i32> = data.to_owned();
    let mut output = Vec::new();
    let mut ii = 0;
    let mut i = 0;
    while i < opcode.len() {
        if opcode[i] == 99 {
            break;
        }
        let opc = opcode[i] % 100;
        let mut m = opcode[i] / 10;
        let modes: Vec<i32> = (0..3)
            .map(move |_| {
                m /= 10;
                m % 10
            })
            .collect();
        if opc == 5 || opc == 6 {
            let idx1 = if modes[0] == 0 {
                opcode[i + 1] as usize
            } else {
                i + 1
            };
            let idx2 = if modes[1] == 0 {
                opcode[i + 2] as usize
            } else {
                i + 2
            };
            if (opcode[idx1] == 0) == (opc == 6) {
                i = opcode[idx2] as usize;
            } else {
                i += 3;
            }
        } else if opc == 3 {
            assert_eq!(modes[0], 0);
            let idx = opcode[i + 1] as usize;
            opcode[idx] = input[ii];
            ii += 1;
            i += 2;
        } else if opc == 4 {
            let idx = if modes[0] == 0 {
                opcode[i + 1] as usize
            } else {
                i + 1
            };
            output.push(opcode[idx]);
            i += 2;
        } else {
            assert_eq!(modes[2], 0);
            let idx1 = if modes[0] == 0 {
                opcode[i + 1] as usize
            } else {
                i + 1
            };
            let idx2 = if modes[1] == 0 {
                opcode[i + 2] as usize
            } else {
                i + 2
            };
            let idx3 = opcode[i + 3] as usize;
            opcode[idx3] = if opc == 1 {
                opcode[idx1] + opcode[idx2]
            } else if opc == 2 {
                opcode[idx1] * opcode[idx2]
            } else if opc == 7 {
                if opcode[idx1] < opcode[idx2] {
                    1
                } else {
                    0
                }
            } else if opc == 8 {
                if opcode[idx1] == opcode[idx2] {
                    1
                } else {
                    0
                }
            } else {
                panic!("Unknow opcode!")
            };
            i += 4;
        }
    }
    output[output.len() - 1]
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<i32> = input[0]
        .split(',')
        .map(|v| v.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => count(&data, 1).to_string(),
        Step::Second => count(&data, 5).to_string(),
    }
}
