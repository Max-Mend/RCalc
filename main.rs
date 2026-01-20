use std::io;

fn main() {
    println!("----------- RCalc -----------");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "help" {
            println!("Commands:");
            println!("-- Calc operation --");
            println!("  number + number");
            println!("  number - number");
            println!("  number * number");
            println!("  number / number");
            println!("  number ** number   (power)");
            println!("  number // number(0)   (sqrt of first)");
            println!("-- Command --");
            println!("  info");
            println!("  exit");
            continue;
        }

        if input == "info" {
            println!("Name app: RCalc");
            println!("Made in Rust");
            println!("Version: {}", 1.0);
            println!("Author: Max-Mend");
            continue;
        }


        if input == "exit" {
            break;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.len() != 3 {
            println!("Input error! Use format: number operator number");
            continue;
        }

        let num1: f64 = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: first number is incorrect");
                continue;
            }
        };

        let num2: f64 = match parts[2].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error: second number is incorrect");
                continue;
            }
        };

        let operation = parts[1];

        let res = match operation {
            "+" => num1 + num2,
            "-" => num1 - num2,
            "*" => num1 * num2,
            "/" => {
                if num2 == 0.0 {
                    println!("Error: division by zero!");
                    continue;
                } else {
                    num1 / num2
                }
            }
            "**" => num1.powf(num2),
            "//" => num1.sqrt(),
            _ => {
                println!("Error: Unknown operator");
                continue;
            }
        };

        println!("Result: {}", res);
    }
}
