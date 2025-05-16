///! builtin functions

use crate::value::{LoxFunction, LoxValue};
use crate::env::Environment;
use crate::error::RloxError;

/// Macros to initialize built-in functions
macro_rules! init_builtin {
    ($env:expr, $name:expr, $arity:expr, $impl:expr) => {
        $env.define_globally($name, LoxValue::Callable(LoxFunction::BuiltInFunction($arity, $impl)));
    };
}


pub fn register_builtins(env: &mut Environment) {
    init_builtin!(env, "clock", 0, clock_impl);
    init_builtin!(env, "input", 0, input_impl);
    init_builtin!(env, "parseNumber", 1, parse_number_impl);
}

fn clock_impl(_args: Vec<LoxValue>) -> Result<LoxValue, RloxError> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let seconds = duration.as_secs() as f64;
            Ok(LoxValue::Number(seconds))
        }
        Err(_) => Err(RloxError::RuntimeError("Failed to get clock".to_string(), "clock".to_string())),
    }
}

fn input_impl(_args: Vec<LoxValue>) -> Result<LoxValue, RloxError> {
    use std::io;
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    Ok(LoxValue::String(input.trim().to_string()))
}

fn parse_number_impl(args: Vec<LoxValue>) -> Result<LoxValue, RloxError> {
    if args.len() != 1 {
        return Err(RloxError::RuntimeError("parseNumber takes exactly 1 argument".to_string(), "parseNumber".to_string()));
    }
    match &args[0] {
        LoxValue::String(s) => {
            match s.parse::<f64>() {
                Ok(n) => Ok(LoxValue::Number(n)),
                Err(_) => Err(RloxError::RuntimeError("Invalid number format".to_string(), "parseNumber".to_string())),
            }
        }
        _ => Err(RloxError::RuntimeError("parseNumber takes a string argument".to_string(), "parseNumber".to_string())),
    }
}