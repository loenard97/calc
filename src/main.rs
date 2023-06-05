use std::env;
use std::process;

use colorama::Colored;

#[derive(Debug, PartialEq)]
enum Operator {
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f32),
    Operator(Operator),
    Invalid,
}

impl Token {
    /// Create new Token from str
    fn from_str(token: &str) -> Self {
        match token {
            "+" => Token::Operator(Operator::Plus),
            "-" => Token::Operator(Operator::Minus),
            "." => Token::Operator(Operator::Multiply),
            "/" => Token::Operator(Operator::Divide),
            _ => {
                match token.parse::<f32>() {
                    Ok(val) => Token::Number(val),
                    Err(_) => Token::Invalid,
                }
            }
        }
    }
}

#[derive(Debug)]
struct Stack {
    stack: Vec<f32>,
}

impl Stack {
    /// Create new empty Stack
    fn new() -> Self {
        Stack { stack: vec![] }
    }

    /// Push Token to stack
    /// Token::Invalid's are silently ignored.
    ///
    /// # Errors:
    /// Errors when Stack does not contain enough values for the given Operator.
    fn push(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>> {
        match token {
            Token::Number(number) => self.stack.push(*number),
            Token::Operator(operator) => self._apply_operator(operator)?,
            Token::Invalid => { },
        };

        Ok(())
    }

    /// Apply Operator to current stack
    ///
    /// Errors:
    /// Errors when Stack does not contain enough values for the given Operator.
    fn _apply_operator(&mut self, operator: &Operator) -> Result<(), Box<dyn std::error::Error>> {
        let val_2 = self.stack.pop().ok_or("empty stack")?;
        let val_1 = self.stack.pop().ok_or("empty stack")?;

        let val_ret = match operator {
            Operator::Plus => val_1 + val_2,
            Operator::Minus => val_1 - val_2,
            Operator::Multiply => val_1 * val_2,
            Operator::Divide => val_1 / val_2,
        };

        self.stack.push(val_ret);
        Ok(())
    }

    /// Returns final value if calculation is done,
    /// returns None otherwise.
    fn value(&mut self) -> Option<f32> {
        if self.stack.len() > 1 {
            return None;
        }
        self.stack.pop()
    }
}

/// Print msg to stderr, then
/// exit process with code 1.
fn process_exit(msg: &str) {
    eprintln!("{}: {}", String::from("Application error").color("red").style("bold"), msg);
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let tokens: Vec<_> = args.clone().iter()
        .map(|op| { Token::from_str(op) })
        .collect();

    let mut stack = Stack::new();
    for (i, tok) in tokens.iter().enumerate() {
        if stack.push(tok).is_err() {
            let mut sargs = String::new();
            let mut sline = String::new();

            for (j, arg) in args.iter().enumerate() {
                if i == j {
                    let c = "^".repeat(arg.len());
                    let arg_colored = String::from(arg).color("green");
                    sline.push_str(&c);
                    sargs.push_str(&format!("{} ", arg_colored));
                } else {
                    let c = "─".repeat(arg.len() + 1);
                    sline.push_str(&c);
                    sargs.push_str(&format!("{} ", arg));
                }
                
            }

            process_exit(&format!("not enough values on stack to apply operator\n{}\n{}", sargs, sline));
        };
    }

    match stack.value() {
        Some(val) => println!("{}", val),
        None => process_exit("stack contains more than one value after applying all operators\nAre you missing an operator at the end?"),
    };
}
