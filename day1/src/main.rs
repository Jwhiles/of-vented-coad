use std::fs;

fn solve_first(input: &str) {
    let mut increases = -1;
    let mut prev = -1;
    for line in input.lines() {
        let parsed = line.parse::<i32>().unwrap();

        if parsed > prev {
            increases += 1;
        }
        prev = parsed
    }
    println!("It increased {} times", increases);
}

fn solve_second(input: &str) {
    let mut increases = -3;
    let mut prevsum = -1;

    let mut m_one = 0;
    let mut m_two = 0;

    for line in input.lines() {
        let parsed = line.parse::<i32>().unwrap();
        let sum = m_one + m_two + parsed;
        if sum > prevsum {
            increases += 1
        }
        prevsum = sum;
        m_one = m_two;
        m_two = parsed;
    }
    println!("It increased {} times", increases);
}

fn main() {
    let input = fs::read_to_string("./input").expect("failed to read");
    solve_first(&input);
    solve_second(&input);
}
