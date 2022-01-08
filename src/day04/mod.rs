use super::common::Step;
use std::cmp::Ordering;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![],
        Step::Second => vec![],
    }
}

fn less_than(a: &[u32], b: &[u32]) -> bool {
    for (i,v) in a.iter().enumerate() {
        match v.cmp(&b[i]) {
            Ordering::Greater => { return false; },
            Ordering::Less => { return true; },
            Ordering::Equal => (),
        }
    }
    false
}

fn has_double (a: &[u32], strict: bool) -> bool {
    let mut repeats = vec![0];
    for i in 1..a.len() {
        if a[i-1] == a[i] {
            if !strict {
                return true;
            }
            if let Some(repeat) = repeats.last_mut() {
                *repeat += 1;
            }
        } else {
            repeats.push(0);
        }
    }
    repeats.iter().any(|&v| v == 1)
}

fn count(range: &[Vec<u32>], strict: bool) -> usize {
    let mut passwds = vec![];
    let mut curr = range[0].to_owned();
    // Find the minimal value
    for i in 1..curr.len() {
        if curr[i] < curr[i-1] {
            for i in i..curr.len() {
                curr[i] = curr[i-1];
            }
            break
        }
    }
    while less_than(&curr, &range[1]) {
        if has_double(&curr, strict) {
            passwds.push(curr.to_owned());
        }
        if let Some(i) = curr.iter().rposition(|&v| v!=9) {
            curr[i] += 1;
            for i in i+1..curr.len() {
                curr[i] = curr[i-1];
            }
        } else {
            break;
        }
    }
    passwds.len()
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<u32>> = input[0]
        .split('-')
        .map(|v| {
            v.chars()
                .map(|c| c.to_digit(10).expect("Not a digit!"))
                .collect()
        })
        .collect();
    match step {
        Step::First => count(&data, false).to_string(),
        Step::Second => count(&data, true).to_string(),
    }
}
