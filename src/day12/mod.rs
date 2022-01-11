use super::common::{gcd, Step};

pub fn test_results(step: &Step) -> Vec<(&'static str, String)> {
    match step {
        Step::First => vec![/*("test1.txt", 179.to_string())*/],
        Step::Second => vec![
            ("test1.txt", 2772.to_string()),
            ("test2.txt", 4686774924i64.to_string()),
        ],
    }
}

fn repeat(data: &[Vec<i32>]) -> u64 {
    (0..data[0].len())
        .map(|i| {
            let mut count = 0u64;
            let mut ps: Vec<i32> = data.iter().map(|p| p[i]).collect();
            let mut vs = vec![0; ps.len()];
            loop {
                for p1 in 0..ps.len() - 1 {
                    for p2 in p1 + 1..ps.len() {
                        if ps[p1] < ps[p2] {
                            vs[p1] += 1;
                            vs[p2] -= 1;
                        } else if ps[p1] > ps[p2] {
                            vs[p1] -= 1;
                            vs[p2] += 1;
                        }
                    }
                }
                count += 1;
                if (0..ps.len()).all(|p| ps[p] == data[p][i] && vs[p] == 0) {
                    break;
                }
                ps = (0..ps.len()).map(|p| ps[p] + vs[p]).collect();
            }
            count
        })
        .fold(1, |r, v| r * v / gcd(r, v))
}

fn count(data: &[Vec<i32>]) -> i32 {
    let mut ps = data.to_owned();
    let mut vs = vec![vec![0; ps[0].len()]; ps.len()];
    for _ in 0..1000 {
        // Update velocities
        for p1 in 0..ps.len() - 1 {
            for p2 in p1..ps.len() {
                for i in 0..ps[0].len() {
                    if ps[p1][i] < ps[p2][i] {
                        vs[p1][i] += 1;
                        vs[p2][i] -= 1;
                    } else if ps[p1][i] > ps[p2][i] {
                        vs[p1][i] -= 1;
                        vs[p2][i] += 1;
                    }
                }
            }
        }
        // Update coordinates
        ps = ps
            .iter()
            .enumerate()
            .map(|(c, p)| (0..ps[0].len()).map(|i| p[i] + vs[c][i]).collect())
            .collect();
    }
    ps.iter()
        .enumerate()
        .map(|(c, p)| {
            [&p, &vs[c]]
                .iter()
                .map(|v| v.iter().map(|n| n.abs()).sum::<i32>())
                .product::<i32>()
        })
        .sum()
}

pub fn solution(step: &Step, input: &[String]) -> String {
    let data: Vec<Vec<i32>> = input
        .iter()
        .map(|l| {
            let mut s = l.chars();
            s.next();
            s.next_back();
            s.as_str()
                .split(", ")
                .map(|c| {
                    c.split("=")
                        .nth(1)
                        .expect("Missing value!")
                        .parse()
                        .expect("Not a number!")
                })
                .collect()
        })
        .collect();
    match step {
        Step::First => count(&data).to_string(),
        Step::Second => repeat(&data).to_string(),
    }
}
