pub fn parse_and_calculate(input: &str) -> Result<f64, String> {
    let input = input.trim().replace(" ", "");

    if let Some(pos) = input.find("**") {
        let num1: f64 = input[..pos].parse()
            .map_err(|_| "Error: first number is invalid".to_string())?;
        let num2: f64 = input[pos+2..].parse()
            .map_err(|_| "Error: second number is invalid".to_string())?;
        return Ok(num1.powf(num2));
    }

    if let Some(pos) = input.find("//") {
        let num1: f64 = input[..pos].parse()
            .map_err(|_| "Error: number is invalid".to_string())?;
        return Ok(num1.sqrt());
    }

    let operators = ['+', '-', '*', '/'];
    for (i, ch) in input.chars().enumerate() {
        if i > 0 && operators.contains(&ch) {
            let num1: f64 = input[..i].parse()
                .map_err(|_| "Error: first number is invalid".to_string())?;
            let num2: f64 = input[i+1..].parse()
                .map_err(|_| "Error: second number is invalid".to_string())?;

            return match ch {
                '+' => Ok(num1 + num2),
                '-' => Ok(num1 - num2),
                '*' => Ok(num1 * num2),
                '/' => {
                    if num2 == 0.0 {
                        Err("Error: division by zero!".to_string())
                    } else {
                        Ok(num1 / num2)
                    }
                },
                _ => Err("Error: unknown operator".to_string())
            };
        }
    }

    Err("Error: invalid format".to_string())
}