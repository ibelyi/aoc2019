use super::common::Step;
use std::cmp::Ordering;

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![
            ("test1.txt", 33.to_string()),
            ("test2.txt", 35.to_string()),
            ("test3.txt", 41.to_string()),
            ("test4.txt", 210.to_string()),
        ],
        Step::Second => vec![("test4.txt", 802.to_string())],
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if a == 0 {
        b
    } else if b == 0 || a == b {
        a
    } else if a < b {
        gcd(a, b % a)
    } else {
        gcd(b, a % b)
    }
}

fn vision(view: &mut [Vec<usize>]) -> (usize, usize) {
    let mut max = 0;
    let mut coord = (0, 0);
    for y1 in 0..view.len() {
        for x1 in 0..view[0].len() {
            if view[y1][x1] == 0 {
                continue;
            }
            let mut count = 0;
            for y2 in 0..view.len() {
                for x2 in 0..view[0].len() {
                    if y1 == y2 && x1 == x2 || view[y2][x2] == 0 {
                        continue;
                    }
                    let (xs, xf) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                    let (ys, yf) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
                    let d = gcd(xf - xs, yf - ys) as i32;
                    let (dx, dy) = ((x2 as i32 - x1 as i32) / d, (y2 as i32 - y1 as i32) / d);
                    if (1..d).all(|i| {
                        view[(y1 as i32 + i * dy) as usize][(x1 as i32 + i * dx) as usize] == 0
                    }) {
                        count += 1;
                    }
                }
            }
            view[y1][x1] = count;
            if count > max {
                max = count;
                coord = (x1, y1);
            }
        }
    }
    coord
}

fn count(data: &[Vec<usize>]) -> usize {
    let mut view = data.to_owned();
    let (x, y) = vision(&mut view);
    view[y][x]
}

fn evaporate(data: &[Vec<usize>]) -> usize {
    let mut view = data.to_owned();
    let (x, y) = vision(&mut view);
    let mut asteroids = Vec::new();
    for y2 in 0..view.len() {
        for x2 in 0..view[0].len() {
            if y2 == y && x2 == x || view[y2][x2] == 0 {
                continue;
            }
            let (xs, xf) = if x < x2 { (x, x2) } else { (x2, x) };
            let (ys, yf) = if y < y2 { (y, y2) } else { (y2, y) };
            let d = gcd(xf - xs, yf - ys) as i32;
            let (dx, dy) = ((x2 as i32 - x as i32) / d, (y2 as i32 - y as i32) / d);
            if (1..d).all(|i| view[(y as i32 + i * dy) as usize][(x as i32 + i * dx) as usize] == 0)
            {
                let degree = if x <= x2 {
                    if y2 < y {
                        (0, ((x2 - x) as f64 / (y - y2) as f64))
                    } else if x != x2 {
                        (1, ((y2 - y) as f64 / (x2 - x) as f64))
                    } else {
                        (2, 0f64)
                    }
                } else if y < y2 {
                    (2, ((x - x2) as f64 / (y2 - y) as f64))
                } else {
                    (3, ((y - y2) as f64 / (x - x2) as f64))
                };
                asteroids.push((degree, x2 * 100 + y2));
            }
        }
    }
    asteroids.sort_by(|a, b| match a.0 .0.cmp(&b.0 .0) {
        Ordering::Equal => a.0 .1.partial_cmp(&b.0 .1).unwrap(),
        other => other,
    });
    assert_eq!(asteroids.len(), view[y][x]);
    if asteroids.len() < 200 {
        println!("{}", asteroids.len());
        panic!("Too few asteroids!")
    } else {
        asteroids[199].1
    }
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<usize>> = input
        .iter()
        .map(|l| l.chars().map(|c| if c == '#' { 1 } else { 0 }).collect())
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => evaporate(&data).to_string(),
    }
}
