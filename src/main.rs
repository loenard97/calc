use std::env;
use std::process;
use std::io::BufRead;
use std::f32::consts;

use colorama::Colored;

/// Math Constant
/// 
/// Is always evaluated to f32 without any input
#[derive(Debug, PartialEq)]
enum Constant {
    Pi,
    E,
}

/// Operators that consume one Stack value
#[derive(Debug, PartialEq)]
enum SingleOperator {
    Ln,
    Log2,
    Log10,
    Sin,
    Cos,
    Tan,
}

/// Operators that consume two Stack values
#[derive(Debug, PartialEq)]
enum DoubleOperator {
    Plus,
    Minus,
    Multiply,
    Divide,
    Log,
}

/// Calculator Token
#[derive(Debug, PartialEq)]
enum Token {
    Number(f32),
    Constant(Constant),
    SingleOperator(SingleOperator),
    DoubleOperator(DoubleOperator),
    Invalid,
}

impl Token {
    /// Create new Token from str
    fn from_str(token: &str) -> Self {
        match token {
            "pi" => Token::Constant(Constant::Pi),
            "e" => Token::Constant(Constant::E),

            "ln" => Token::SingleOperator(SingleOperator::Ln),
            "log2" => Token::SingleOperator(SingleOperator::Log2),
            "log10" => Token::SingleOperator(SingleOperator::Log10),
            "sin" => Token::SingleOperator(SingleOperator::Sin),
            "cos" => Token::SingleOperator(SingleOperator::Cos),
            "tan" => Token::SingleOperator(SingleOperator::Tan),

            "+" => Token::DoubleOperator(DoubleOperator::Plus),
            "-" => Token::DoubleOperator(DoubleOperator::Minus),
            "." => Token::DoubleOperator(DoubleOperator::Multiply),
            "/" => Token::DoubleOperator(DoubleOperator::Divide),
            "log" => Token::DoubleOperator(DoubleOperator::Log),

            _ => {
                match token.parse::<f32>() {
                    Ok(val) => Token::Number(val),
                    Err(_) => Token::Invalid,
                }
            }
        }
    }
}

/// Stack of f32 values
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
    /// 
    /// Token::Invalid's are silently ignored.
    ///
    /// # Errors:
    /// Errors when Stack does not contain enough values for the given Operator.
    fn push(&mut self, token: &Token) -> Result<(), Box<dyn std::error::Error>> {
        match token {
            Token::Number(number) => self.stack.push(*number),
            Token::Constant(constant) => self._apply_constant(constant),
            Token::SingleOperator(operator) => self._apply_single_operator(operator)?,
            Token::DoubleOperator(operator) => self._apply_double_operator(operator)?,
            Token::Invalid => { },
        };

        Ok(())
    }

    /// Apply Constant to current stack
    fn _apply_constant(&mut self, constant: &Constant) {
        self.stack.push(
            match constant {
                Constant::Pi => consts::PI,
                Constant::E => consts::E,
            }
        )
    }

    /// Apply SingleOperator to current stack
    ///
    /// # Errors:
    /// Errors when Stack does not contain enough values for the given Operator.
    fn _apply_single_operator(&mut self, operator: &SingleOperator) -> Result<(), Box<dyn std::error::Error>> {
        let val_1 = self.stack.pop().ok_or("empty stack")?;

        self.stack.push(
            match operator {
                SingleOperator::Ln => val_1.ln(),
                SingleOperator::Log2 => val_1.log2(),
                SingleOperator::Log10 => val_1.log10(),
                SingleOperator::Sin => val_1.sin(),
                SingleOperator::Cos => val_1.cos(),
                SingleOperator::Tan => val_1.tan(),
            }
        );

        Ok(())
    }

    /// Apply DoubleOperator to current stack
    ///
    /// # Errors:
    /// Errors when Stack does not contain enough values for the given Operator.
    fn _apply_double_operator(&mut self, operator: &DoubleOperator) -> Result<(), Box<dyn std::error::Error>> {
        let val_2 = self.stack.pop().ok_or("empty stack")?;
        let val_1 = self.stack.pop().ok_or("empty stack")?;

        self.stack.push(
            match operator {
                DoubleOperator::Plus => val_1 + val_2,
                DoubleOperator::Minus => val_1 - val_2,
                DoubleOperator::Multiply => val_1 * val_2,
                DoubleOperator::Divide => val_1 / val_2,
                DoubleOperator::Log => val_1.log(val_2),
            }
        );

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

fn main() {
    /* get input either from stdin or env args */
    let args: Vec<String> = env::args().collect();
    let input: Vec<String> = if atty::is(atty::Stream::Stdin) {
        let input = args.clone();
        if input.len() == 1 {
            eprintln!(
                "{}: no expression given\n\
                Usage: give a math expression in reverse Polish notation to calculate\n\
                $ calc 3 5 +\n8", 
                String::from("Error").color("red").style("bold")
            );
            process::exit(1);
        };
        input
    } else {
        let mut buffer = String::from(
            args.iter().next().unwrap_or(&"calc".to_string())
        );
        buffer.push(' ');
        _ = std::io::stdin()
            .lock()
            .read_line(&mut buffer);
        buffer.split(' ')
            .map(|s| { s.trim_end().to_string() })
            .collect()
    };

    /* parse input into Tokens */
    let mut invalid = vec![];
    let mut args_clone = input.clone();
    args_clone.remove(0);
    let tokens: Vec<_> = args_clone.iter()
        .map(|op| { Token::from_str(&op.to_lowercase() ) })
        .collect();

    for (i, tok) in tokens.iter().enumerate() {
        if *tok == Token::Invalid {
            invalid.push(i);
        }
    }

    /* print warning on invalid tokens */
    if invalid.len() > 0 {
        eprintln!("{}: expression contains {} invalid token{}", 
            String::from("Warning").color("yellow").style("bold"), 
            invalid.len(), 
            if invalid.len() == 1 { '\0' } else { 's' }
        );
        
        let mut sargs = String::new();
        let mut sline = String::new();
        for (i, arg) in input.iter().enumerate() {
            if i > 0 && invalid.contains(&(i-1)) {
                let c = "^".repeat(arg.len()).color("yellow");
                let arg_colored = String::from(arg).color("yellow");
                sline.push_str(&c);
                sline.push('─');
                sargs.push_str(&format!("{} ", arg_colored));
            } else {
                let c = "─".repeat(arg.len() + 1);
                sline.push_str(&c);
                sargs.push_str(&format!("{} ", arg));
            }
        }
        eprintln!(" │ {}\n └─{}", sargs, sline);
    }

    /* push tokens onto stack */
    let mut stack = Stack::new();
    for (i, tok) in tokens.iter().enumerate() {
        if stack.push(tok).is_err() {
            let mut sargs = String::new();
            let mut sline = String::new();

            for (j, arg) in input.iter().enumerate() {
                if i + 1 == j {
                    let c = "^".repeat(arg.len()).color("red");
                    let arg_colored = String::from(arg).color("red");
                    sline.push_str(&c);
                    sargs.push_str(&format!("{} ", arg_colored));
                } else {
                    let c = "─".repeat(arg.len() + 1);
                    sline.push_str(&c);
                    sargs.push_str(&format!("{} ", arg));
                }
            }

            eprintln!(
                "{}: not enough values on stack to apply operator\n │ {}\n └─{}", 
                String::from("Error").color("red").style("bold"),
                sargs, sline
            );
            process::exit(1);
        };
    }

    /* print final value */
    match stack.value() {
        Some(val) => println!("{}", val),
        None => {
            eprintln!(
                "{}: stack contains more than one value after applying all operators", 
                String::from("Error").color("red").style("bold")
            );
            process::exit(1);
        },
    };
}
