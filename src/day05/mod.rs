use super::common::{intcode_run, Step};

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![],
        Step::Second => vec![],
    }
}

fn count(data: &[i32], id: i32) -> i32 {
    intcode_run(&data, &[id])
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
