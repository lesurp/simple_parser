use crate::Node;
use log::debug;
use regex::Regex;

pub fn parse_word(expr: &str, offset: usize) -> Option<(Node<'_>, usize)> {
    debug!("Parsing word from input '{}'", &expr[offset..]);
    let new_offset = offset
        + expr[offset..]
            .chars()
            .take_while(|c| !c.is_whitespace())
            .count();
    if new_offset != offset {
        debug!("\tOK: extracted '{}'", &expr[offset..new_offset]);
        Some((Node::LiteralStr(&expr[offset..new_offset]), new_offset))
    } else {
        debug!("\tErr: input is empty or starts with whitespaces");
        None
    }
}

pub fn parse_unsigned(expr: &str, offset: usize) -> Option<(Node<'_>, usize)> {
    debug!("Parsing unsigned integer from input '{}'", &expr[offset..]);
    let new_offset = offset
        + expr[offset..]
            .chars()
            .take_while(|c| c.is_ascii_digit())
            .count();
    if new_offset != offset {
        let number = expr[offset..new_offset].parse::<usize>().ok()?;
        debug!("\tOK: extracted number '{}'", number);
        Some((Node::LiteralUnsigned(number), new_offset))
    } else {
        debug!("\tErr: input is empty or starts with whitespaces");
        None
    }
}

pub fn parse_float(expr: &str, offset: usize) -> Option<(Node<'_>, usize)> {
    debug!("Parsing float from input '{}'", &expr[offset..]);
    let re = Regex::new(r"^[+-]?\d+\.\d+").unwrap();
    let m = re.find(&expr[offset..])?;
    debug!("Found group: {}", &expr[offset..offset + m.end()]);
    let new_offset = m.end() + offset;
    let number = expr[offset..new_offset].parse::<f64>().ok()?;
    debug!("\tOK: extracted float '{}'", number);
    Some((Node::LiteralFloat(number), new_offset))
}

#[macro_export]
macro_rules! word {
    () => {
        parser::Rule::Terminal(Box::new(crate::parser::parse_word))
    };
}

#[macro_export]
macro_rules! unsigned {
    () => {
        parser::Rule::Terminal(Box::new(crate::parser::parse_unsigned))
    };
}

#[macro_export]
macro_rules! float {
    () => {
        parser::Rule::Terminal(Box::new(crate::parser::parse_float))
    };
}
