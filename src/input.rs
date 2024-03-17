use core::f64;
use std::io::stdin;

pub fn read_number() -> f64 {
    loop {
        let input = read_input();
        let input = input.trim();
        let input:f64 = match input.parse() {
            Ok(value) => value,
            Err(_) => {
                println!("The given input is not a valid number");
                continue;
            }
        };

        return input;
    }
}

pub fn read_number_from_range(range: &[f64]) -> f64 {
    loop {
        let input = read_number();

        for number in range {
            if input == *number {
                return input;
            }
        }

        println!("The given option is not in the range, please try again.");
        continue;
    }
}

pub fn read_input() -> String {
    loop {
        let mut input: String = "".to_string();
        stdin()
            .read_line(&mut input)
            .expect("There was an error reading the line");
        return input;
    }
}
