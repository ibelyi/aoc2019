use super::common::Step;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![
            ("test_input.txt", 135.to_string()),
            ("test_input2.txt", 159.to_string()),
        ],
        Step::Second => vec![
            ("test_input.txt", 410.to_string()),
            ("test_input2.txt", 610.to_string()),
        ],
    }
}

fn next(curr: &(i32, i32), line: &(usize, i32)) -> (i32, i32) {
    match line.0 {
        0 => (curr.0 + line.1, curr.1),
        1 => (curr.0, curr.1 + line.1),
        2 => (curr.0 - line.1, curr.1),
        3 => (curr.0, curr.1 - line.1),
        _ => panic!("Unknown direction!"),
    }
}

fn triple(curr: &(i32, i32), line: &(usize, i32)) -> (i32, i32, i32) {
    match line.0 {
        0 => (curr.0, curr.0 + line.1, curr.1),
        1 => (curr.1, curr.1 + line.1, curr.0),
        2 => (curr.0 - line.1, curr.0, curr.1),
        3 => (curr.1 - line.1, curr.1, curr.0),
        _ => panic!("Unknown direction!"),
    }
}

fn count(wires: &[Vec<(usize, i32)>], steps: bool) -> i32 {
    let mut inter: Vec<i32> = Vec::new();
    let mut curr1 = (0, 0);
    let mut step1 = 0;
    for line1 in &wires[0] {
        let t1 = triple(&curr1, line1);
        let mut curr2 = (0, 0);
        let mut step2 = 0;
        for line2 in &wires[1] {
            let t2 = triple(&curr2, line2);
            if line1.0 % 2 != line2.0 % 2
                && t1.0 <= t2.2
                && t1.1 >= t2.2
                && t2.0 <= t1.2
                && t2.1 >= t1.2
                && t1.2 != 0
                && t2.2 != 0
            {
                if steps {
                    inter.push(
                        step1
                            + step2
                            + if line1.0 < 2 {
                                t2.2 - t1.0
                            } else {
                                t1.1 - t2.2
                            }
                            + if line2.0 < 2 {
                                t1.2 - t2.0
                            } else {
                                t2.1 - t1.2
                            },
                    );
                } else {
                    inter.push(t1.2.abs() + t2.2.abs());
                }
            }
            step2 += line2.1;
            curr2 = next(&curr2, line2);
        }
        step1 += line1.1;
        curr1 = next(&curr1, line1);
    }
    inter.sort_unstable();
    inter[0]
}

const DIR: [char; 4] = ['U', 'R', 'D', 'L'];

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<(usize, i32)>> = input
        .iter()
        .map(|line| {
            line.split(',')
                .map(|v| {
                    (
                        DIR.iter()
                            .position(|&d| d == v.chars().next().expect("Too short!"))
                            .expect("Unknown direction!"),
                        v.chars()
                            .skip(1)
                            .collect::<String>()
                            .parse()
                            .expect("Not a number!"),
                    )
                })
                .collect()
        })
        .collect();
    match step {
        Step::First => count(&data, false).to_string(),
        Step::Second => count(&data, true).to_string(),
    }
}
