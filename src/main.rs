use std::io::{stdin, stdout, Write};

use eval::f;

fn main() {
    loop {
        print!(">>> ");
        stdout().flush().unwrap();
        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();
        input = input.replace("\n", "");
        if input.contains("|") {
            let split: Vec<&str> = input.split("|").into_iter().collect::<Vec<&str>>();
            let vars: Vec<f64> = split[1]
                .trim()
                .split_ascii_whitespace()
                .into_iter()
                .map(|x| x.parse::<f64>().unwrap())
                .collect::<Vec<f64>>();
            let expression: &str = split[0].trim();
            let answer: f64 = eval::math::eval(&eval::math::fill(expression, &vars));
            let string_answer: String = answer.to_string();
            if input.chars().filter(|x| *x == '|').count() == 2 {
                let round: i32 = split[2].trim().parse::<i32>().unwrap();
                println!("{}", eval::f!(&string_answer; round));
            } else {
                println!("{:.width$}", answer, width = string_answer.len());
            }
        } else {
            let answer = f!(&input);
            println!(
                "answer is {:.width$}",
                answer,
                width = answer.to_string().len()
            );
        }
        input.clear();
    }
}
