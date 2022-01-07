use super::common::Step;

pub fn test_result(step: &Step) -> String {
    match step {
        Step::First => (2 + 2 + 654 + 33583).to_string(),
        Step::Second => (2 + 2 + 966 + 50346).to_string(),
    }
}

fn count_plain(data: &[u32]) -> u32 {
    data.iter().fold(0, |t, m| {
        let f = m / 3;
        if f > 2 {
            t + f - 2
        } else {
            t
        }
    })
}

fn count(data: &[u32]) -> u32 {
    data.iter().fold(0, |t, m| {
        let mut f = m / 3;
        let mut curr = 0;
        while f > 2 {
            curr += f - 2;
            f = (f - 2) / 3;
        }
        t + curr
    })
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<u32> = input
        .iter()
        .map(|line| line.parse().expect("Not a number!"))
        .collect();
    match step {
        Step::First => count_plain(&data).to_string(),
        Step::Second => count(&data).to_string(),
    }
}
