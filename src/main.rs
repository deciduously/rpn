use std::{io, num::ParseIntError};

mod test;

/// Read a line of input
fn get_input() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input)
}

/// Read input from command line until newline
fn rpn(input: String) -> Result<i32, ParseIntError> {
    let mut stack = Vec::new();

    for c in input.split_whitespace() {
        match c {
            "+" | "-" | "*" | "/" => {
                let operand2 = stack.pop().expect("Need at least one operand");
                let operand1 = stack.pop().expect("Need two operands");
                match c {
                    "+" => stack.push(operand1 + operand2),
                    "-" => stack.push(operand1 - operand2),
                    "*" => stack.push(operand1 * operand2),
                    "/" => stack.push(operand1 / operand2),
                    _ => panic!("Not an operator"),
                }
            }
            _ => stack.push(c.parse::<i32>()?),
        }
    }
    Ok(stack.pop().expect("No result found"))
}

fn main() {
    // TODO Take cmd line args as input
    println!("Enter input: ");
    let input = get_input().expect("Bad input, doofus");
    let result = rpn(input);
    println!("{:?}", result.unwrap());
}
