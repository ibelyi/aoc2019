use aoc2019::{
    common::{lines_from_file, Step},
    day09::{solution, test_results},
};
const DAY: &str = "day09";

fn main() {
    let input = "./src/".to_string() + DAY + "/input.txt";
    if let Ok(lines) = lines_from_file(&input) {
        for s in &[Step::First, Step::Second] {
            for (file, expected) in test_results(s) {
                let test_input = "./src/".to_string() + DAY + "/" + file;
                if let Ok(test_lines) = lines_from_file(&test_input) {
                    let actual = solution(s, &test_lines);
                    if actual == expected {
                        continue;
                    }
                    eprintln!(
                        "{:?}: Test from {} got {}, expected {}",
                        s, file, actual, expected
                    );
                } else {
                    eprintln!("Failed to read lines from {}", test_input);
                }
                std::process::exit(1);
            }
            println!("{:?}: {}", s, solution(s, &lines));
        }
    } else {
        eprintln!("Failed to load lines from {}", input);
    }
}
