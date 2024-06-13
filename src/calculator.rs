pub fn calculate(operator: &str, operand1: f64, operand2: f64) -> Result<f64, String> {
    match operator {
        "+" | "add" => Ok(operand1 + operand2),
        "-" | "sub" => Ok(operand1 - operand2),
        "*" | "mul" => Ok(operand1 * operand2),
        "/" | "div" => {
            if operand2 != 0.0 {
                Ok(operand1 / operand2)
            } else {
                Err(String::from("Error: Division by zero"))
            }
        }
        _ => Err(String::from("Error: Invalid operator")),
    }
}
