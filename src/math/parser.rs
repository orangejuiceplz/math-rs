use anyhow::Result;
use regex::Regex;

#[derive(Debug, Clone)]
pub enum MathExpression {
    Arithmetic(String),
    Algebraic(String),
    Calculus(String),
    Unknown(String),
}

pub fn parse_math_expression(input: &str) -> Result<MathExpression> {
    let input = input.trim();
    
    // simple regex patterns for different types of math
    let arithmetic_pattern = Regex::new(r"^[0-9+\-*/().\s]+$")?;
    let solve_pattern = Regex::new(r"(?i)solve|=|equation")?;
    let calculus_pattern = Regex::new(r"(?i)derivative|integral|limit|dx|dy")?;
    
    if calculus_pattern.is_match(input) {
        Ok(MathExpression::Calculus(input.to_string()))
    } else if solve_pattern.is_match(input) {
        Ok(MathExpression::Algebraic(input.to_string()))
    } else if arithmetic_pattern.is_match(input) {
        Ok(MathExpression::Arithmetic(input.to_string()))
    } else {
        Ok(MathExpression::Unknown(input.to_string()))
    }
}