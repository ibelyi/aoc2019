use super::common::{Intcode, Step};
use std::cmp::Ordering;
use std::collections::HashMap;
#[cfg(feature = "display")]
use std::{thread, time};

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![/*("test_input.txt", 0.to_string())*/],
        Step::Second => vec![/*("test_input.txt", 0.to_string())*/],
    }
}

fn play(data: &[&String]) -> i64 {
    let mut game = Intcode::parse(data[0]);
    game.set(0, 2);
    let mut screen: HashMap<(i64, i64), i64> = HashMap::new();

    let mut score = 0;
    while !game.halted() {
        let inp = if let Some(paddle) = screen
            .iter()
            .find_map(|(&k, &v)| if v == 3 { Some(k) } else { None })
        {
            screen
                .iter()
                .find_map(|(&k, &v)| if v == 4 { Some(k) } else { None })
                .map(|ball| match ball.0.cmp(&paddle.0) {
                    Ordering::Equal => 0,
                    Ordering::Greater => 1,
                    Ordering::Less => -1,
                })
        } else {
            None
        };
        if let Some(v) = execute(inp, &mut game, &mut screen) {
            score = v;
        };
    }
    score
}

fn count(data: &[&String]) -> usize {
    let mut game = Intcode::parse(data[0]);
    let mut screen = HashMap::new();

    execute(None, &mut game, &mut screen);

    if !game.halted() {
        panic!("Expecting input!");
    }
    screen.iter().filter(|(_, v)| **v == 2).count()
}

fn execute(
    inp: Option<i64>,
    game: &mut Intcode,
    screen: &mut HashMap<(i64, i64), i64>,
) -> Option<i64> {
    let mut inp = inp;
    let mut stage = (0, 0, 0);
    let mut score = None;
    while let Some(v) = game.run(&mut inp) {
        match stage.0 {
            0 => {
                stage.1 = v;
            }
            1 => {
                stage.2 = v;
            }
            2 => {
                if stage.1 == -1 && stage.2 == 0 {
                    score = Some(v);
                } else {
                    screen.insert((stage.1, stage.2), v);
                }
            }
            _ => panic!("Incorrect stage!"),
        }
        stage.0 = (stage.0 + 1) % 3;
    }
    assert_eq!(stage.0, 0);
    #[cfg(feature = "display")]
    display(&screen, &score);
    score
}

#[cfg(feature = "display")]
const DISP: [char; 5] = [' ', '#', 'U', '_', 'o'];

#[cfg(feature = "display")]
fn display(screen: &HashMap<(i64, i64), i64>, score: &Option<i64>) {
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
    for ((x, y), _) in screen {
        if max_x < *x {
            max_x = *x;
        }
        if min_x > *x {
            min_x = *x;
        }
        if max_y < *y {
            max_y = *y;
        }
        if min_y > *y {
            min_y = *y;
        }
    }
    println!("\x1bc");
    println!("({},{})x({},{})", min_x, min_y, max_x, max_y);
    for y in min_y..=max_y {
        println!(
            "{}",
            (min_x..=max_x)
                .map(|x| DISP[if let Some(v) = screen.get(&(x, y)) {
                    *v as usize
                } else {
                    0
                }])
                .collect::<String>()
        );
    }
    if let Some(v) = score {
        println!("Score: {}", v);
    }
    thread::sleep(time::Duration::from_millis(20));
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<&String> = input.iter().collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => play(&data).to_string(),
    }
}
