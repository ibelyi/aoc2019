use super::common::{intcode_run, Intcode, Step};

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![],
        Step::Second => vec![
            ("test_input1.txt", "139629729".to_string()),
            ("test_input2.txt", "18216".to_string()),
        ],
    }
}

fn next(curr: Vec<i32>) -> Option<Vec<i32>> {
    for i in (1..curr.len()).rev() {
        if curr[i] > curr[i - 1] {
            let p = curr.split_at(i - 1);
            let mut result = p.0.to_vec();
            let mut rest = p.1.to_vec();
            rest.sort_unstable();
            if let Some(i) = rest.iter().position(|&v| v > curr[i - 1]) {
                result.push(rest.remove(i));
            } else {
                panic!("Didn't find bigger value");
            }
            result.append(&mut rest);
            return Some(result);
        }
    }
    None
}

fn count_loopback(data: &[i32]) -> i32 {
    let mut max = 0;
    let mut choice = Some(vec![5, 6, 7, 8, 9]);
    while let Some(curr) = choice {
        let mut amps = vec![Intcode::new(data); 5];
        for i in 0..amps.len() {
            let mut val = Some(curr[i]);
            if amps[i].run(&mut val).is_some() {
                panic!("Didn't expect output on phase input!");
            }
            if amps[i].halted() {
                panic!("Should not be halted after initialization!");
            }
        }
        let mut val = 0;
        let mut i = 0;
        loop {
            if amps[i].halted() {
                panic!("Halted amplifier!");
            }
            let mut input = Some(val);
            if let Some(new) = amps[i].run(&mut input) {
                val = new;
            } else if !amps[i].halted() {
                panic!("Unexpectedly waiting for an input!");
            }
            if i == amps.len() - 1 && amps[i].halted() {
                break;
            }
            i = (i + 1) % amps.len();
        }
        if val > max {
            max = val;
        }
        choice = next(curr);
    }
    max
}

fn count(data: &[i32]) -> i32 {
    let mut max = 0;
    let mut choice = Some(vec![0, 1, 2, 3, 4]);
    while let Some(curr) = choice {
        let mut val = 0;
        for p in &curr {
            val = intcode_run(data, &[*p, val]);
        }
        if val > max {
            max = val;
        }
        choice = next(curr);
    }
    max
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<i32> = input[0]
        .split(',')
        .map(|v| v.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => count_loopback(&data).to_string(),
    }
}
