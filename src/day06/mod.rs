use super::common::Step;
use std::collections::HashMap;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![("test_input.txt", 42.to_string())],
        Step::Second => vec![("test_input2.txt", 4.to_string())],
    }
}

fn get_count(
    key: &str,
    orbits: &HashMap<String, String>,
    counts: &mut HashMap<String, u32>,
) -> u32 {
    if let Some(val) = counts.get(key) {
        return *val;
    }
    let val = if let Some(key2) = orbits.get(key) {
        get_count(key2, orbits, counts) + 1
    } else {
        0
    };
    counts.insert(key.to_string(), val);
    val
}

fn transfers(orbits: &HashMap<String, String>) -> usize {
    let mut y_path = vec![];
    let mut curr = "YOU";
    while let Some(planet) = orbits.get(curr) {
        y_path.push(planet.to_string());
        curr = planet;
    }
    y_path.reverse();
    let mut s_path = vec![];
    curr = "SAN";
    while let Some(planet) = orbits.get(curr) {
        s_path.push(planet.to_string());
        curr = planet;
    }
    s_path.reverse();
    if let Some(s) = (0..y_path.len()).position(|i| y_path[i] != s_path[i]) {
        y_path.len() + s_path.len() - 2 * s
    } else {
        0
    }
}

fn count(orbits: &HashMap<String, String>) -> u32 {
    let mut counts = HashMap::new();
    let mut result = 0;
    for planet in orbits.keys() {
        result += get_count(&planet, &orbits, &mut counts);
    }
    result
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: HashMap<String, String> = input
        .iter()
        .map(|l| {
            let x = l.split(')').collect::<Vec<&str>>();
            (x[1].to_string(), x[0].to_string())
        })
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => transfers(&data).to_string(),
    }
}
