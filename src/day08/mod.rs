use super::common::Step;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![],
        Step::Second => vec![],
    }
}

fn image(data: &[usize]) -> i32 {
    let mut output = [[2; 25]; 6];
    for (i, v) in data.iter().enumerate() {
        let (x, y) = (i % 25, (i % 150) / 25);
        if output[y][x] == 2 {
            output[y][x] = *v;
        }
    }
    for line in output {
        println!(
            "{}",
            line.iter()
                .map(|&v| if v == 0 {
                    ' '
                } else if v == 1 {
                    '*'
                } else {
                    '_'
                })
                .collect::<String>()
        );
    }
    0
}

fn count(data: &[usize]) -> u32 {
    let mut min = u32::MAX;
    let mut result = 0;
    let mut counts = [0, 0, 0];
    for (i, v) in data.iter().enumerate() {
        if (i + 1) % 150 == 0 {
            if counts[0] < min {
                min = counts[0];
                result = counts[1] * counts[2];
            }
            counts = [0, 0, 0];
        }
        counts[*v] += 1;
    }
    result
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<usize> = input[0]
        .chars()
        .map(|c| c.to_digit(10).expect("Not a digit!") as usize)
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => image(&data).to_string(),
    }
}
