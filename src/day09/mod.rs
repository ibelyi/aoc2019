use super::common::{intcode_run, Step};

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![],
        Step::Second => vec![],
    }
}

fn count(data: &[i64], input: i64) -> i64 {
    intcode_run(&data, &[input])
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<i64> = input[0]
        .split(',')
        .map(|v| v.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => count(&data, 1).to_string(),
        Step::Second => count(&data, 2).to_string(),
    }
}
