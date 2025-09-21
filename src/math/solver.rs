use crate::math::parser::{parse_math_expression, MathExpression};
use anyhow::{anyhow, Result};
use meval::eval_str;

pub async fn solve_math_problem(input: &str) -> Result<String> {
    let expression = parse_math_expression(input)?;
    
    match expression {
        MathExpression::Arithmetic(expr) => solve_arithmetic(&expr),
        MathExpression::Algebraic(expr) => solve_algebraic(&expr),
        MathExpression::Calculus(expr) => solve_calculus(&expr),
        MathExpression::Unknown(expr) => {
            Err(anyhow!(
                "I'm not sure how to solve this type of problem: '{}'\n\n\
                Try:\n\
                • Basic arithmetic: `2 + 3 * 4`\n\
                • Algebraic equations: `solve 2x + 5 = 11`\n\
                • Calculus: `derivative of x^2`",
                expr
            ))
        }
    }
}

fn solve_arithmetic(expr: &str) -> Result<String> {
    match eval_str(expr) {
        Ok(result) => Ok(format!(
            "🧮 **Arithmetic Solution**\n\n\
            **Problem:** `{}`\n\
            **Answer:** `{}`\n\n\
            ✅ **Step-by-step:**\n\
            1. Evaluate the expression: `{}`\n\
            2. Result: `{}`",
            expr, result, expr, result
        )),
        Err(e) => Err(anyhow!("Could not evaluate arithmetic expression: {}", e)),
    }
}

fn solve_algebraic(expr: &str) -> Result<String> {
    // placeholder for algebraic solving
    // what it needs to do: parse equations and solve for variables
    Ok(format!(
        "📐 **Algebraic Solution**\n\n\
        **Problem:** `{}`\n\n\
        ⚠️ **Note:** Algebraic equation solving is not yet implemented.\n\
        This will be added in a future update!\n\n\
        For now, try basic arithmetic expressions like `2 + 3 * 4`",
        expr
    ))
}

fn solve_calculus(expr: &str) -> Result<String> {
    // placeholder for calculus solving
    Ok(format!(
        "🎓 **Calculus Solution**\n\n\
        **Problem:** `{}`\n\n\
        ⚠️ **Note:** Calculus solving is not yet implemented.\n\
        This will be added in a future update!\n\n\
        For now, try basic arithmetic expressions like `2 + 3 * 4`",
        expr
    ))
}