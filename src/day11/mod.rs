use super::common::{Intcode, Step};
use std::collections::HashMap;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![("test1.txt", 6.to_string())],
        Step::Second => vec![],
    }
}

fn next(val: i64, robot: &(i32, (i32, i32))) -> (i32, (i32, i32)) {
    let (f, (x, y)) = robot;
    let sign = 1 - 2 * val as i32;
    let dx = (1 - (f & 1)) * ((f & 2) - 1);
    let dy = (f & 1) * ((f & 2) - 1);
    ((f + 2 + sign) % 4, (x + sign * dx, y + sign * dy))
}

fn count(data: &[&String], start: i64) -> usize {
    let mut intcode = Intcode::parse(&data[0]);
    let mut painted = HashMap::new();
    // (facing:0..=3, (x:i32, y:i32))
    let mut robot = (0, (0, 0));
    painted.insert(robot.1, start);
    let mut min = (0, 0);
    let mut max = (0, 0);
    loop {
        #[cfg(feature = "display")]
        println!("Robot facing {} @ ({},{})", robot.0, robot.1 .0, robot.1 .1);
        let mut inp = Some(if let Some(v) = painted.get(&robot.1) {
            *v
        } else {
            0
        });
        #[cfg(feature = "display")]
        if let Some(v) = &inp {
            println!("Sending {}", *v);
        }
        if let Some(val) = intcode.run(&mut inp) {
            #[cfg(feature = "display")]
            println!("Got {} back", val);
            painted.insert(robot.1, val);
        } else if intcode.halted() {
            break;
        } else {
            panic!("Didn't get color output!");
        }
        if let Some(val) = intcode.run(&mut inp) {
            #[cfg(feature = "display")]
            println!("Got {} for turn", val);
            robot = next(val, &robot);
        } else {
            panic!("Didn't get turn output!");
        }
        if robot.1 .0 < min.0 {
            min.0 = robot.1 .0;
        }
        if robot.1 .0 > max.0 {
            max.0 = robot.1 .0;
        }
        if robot.1 .1 < min.1 {
            min.1 = robot.1 .1;
        }
        if robot.1 .1 > max.1 {
            max.1 = robot.1 .1;
        }
    }
    if start == 1 {
        for y in min.1..=max.1 {
            println!(
                "{}",
                (min.0..=max.0)
                    .map(|x| DISP[*painted.get(&(x, y)).unwrap_or(&0) as usize])
                    .collect::<String>()
            );
        }
    }
    painted.len()
}

const DISP: [char; 2] = ['.', '*'];

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<&String> = input.iter().collect();
    match step {
        Step::First => count(&data, 0).to_string(),
        Step::Second => count(&data, 1).to_string(),
    }
}
