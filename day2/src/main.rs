use std::fs;

fn solve_first(input: &str) {
    let mut depth = 0;
    let mut horizontal = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        match split[0] {
            "forward" => horizontal += split[1].parse::<i32>().unwrap(),
            "down" => depth += split[1].parse::<i32>().unwrap(),
            "up" => depth -= split[1].parse::<i32>().unwrap(),
            _ => panic!("found a bad instruction"),
        }
    }

    println!("depth was :{}, horizontal was: {}", depth, horizontal);
    println!("multiplied that is {}", depth * horizontal);
}

fn solve_second(input: &str) {
    let mut aim = 0;
    let mut depth = 0;
    let mut horizontal = 0;

    for line in input.lines() {
        let split: Vec<&str> = line.split(' ').collect();

        match split[0] {
            "forward" => {
                let x = split[1].parse::<i32>().unwrap();
                horizontal += x;
                depth += x * aim;
            }

            "down" => aim += split[1].parse::<i32>().unwrap(),
            "up" => aim -= split[1].parse::<i32>().unwrap(),

            _ => panic!("found a bad instruction"),
        }
    }

    println!("depth was :{}, horizontal was: {}", depth, horizontal);
    println!("multiplied that is {}", depth * horizontal);
}

// down X increases your aim by X units.
// up X decreases your aim by X units.
// forward X does two things:
// It increases your horizontal position by X units.
// It increases your depth by your aim multiplied by X.

fn main() {
    let test_input = fs::read_to_string("./test-input").expect("failed to read input");
    solve_first(&test_input);
    solve_second(&test_input);

    let input = fs::read_to_string("./input").expect("failed to read input");
    solve_first(&input);
    solve_second(&input);
}
