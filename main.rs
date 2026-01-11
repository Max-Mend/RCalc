use std::io;

fn main() {
    println!("----------- RCalc -----------");
    println!("Welcome to the RCalc!");
    println!("By: Max-Mend");
    println!("For example: 3 + 5 or 4 // 0");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

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
