//  helper functions

pub fn format_math_result(result: f64) -> String {
    if result.fract() == 0.0 {
        format!("{}", result as i64)
    } else {
        format!("{:.6}", result).trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

pub fn is_valid_math_char(c: char) -> bool {
    c.is_ascii_digit() 
        || c.is_ascii_alphabetic() 
        || "+-*/()=^.√∫∑∏πеx ".contains(c)
}

pub fn clean_math_input(input: &str) -> String {
    input
        .chars()
        .filter(|&c| is_valid_math_char(c))
        .collect::<String>()
        .trim()
        .to_string()
}