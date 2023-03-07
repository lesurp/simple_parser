#![allow(incomplete_features)]
#![feature(adt_const_params)]

use log::debug;
use std::{collections::HashMap, fmt::Debug};
use thiserror::Error;

#[macro_use]
pub mod primitives;
#[macro_use]
pub mod grammar;

#[cfg(test)]
mod test;

#[derive(Debug, Clone, PartialEq)]
pub enum Node<'a> {
    LiteralStr(&'a str),
    LiteralUnsigned(u64),
    LiteralSigned(i64),
    LiteralFloat(f64),
    Token(&'a str),
    Expr(Vec<Node<'a>>),
}

pub trait PrintableCallback: Fn(&str, usize) -> Option<(Node, usize)> {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
impl<F> PrintableCallback for F where F: Fn(&str, usize) -> Option<(Node, usize)> {}

impl Debug for dyn PrintableCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.type_name())
    }
}

pub type TerminalRule = Box<dyn PrintableCallback>;

pub type RuleSet<'a> = Vec<Rule<'a>>;
pub type AlternativeRules<'a> = Vec<RuleSet<'a>>;

#[derive(Debug)]
pub enum Rule<'a> {
    NonTerminal(ParseKey<'a>),
    Terminal(TerminalRule),
}

impl<'a> From<Rule<'a>> for AlternativeRules<'a> {
    fn from(value: Rule<'a>) -> Self {
        vec![vec![value]]
    }
}

pub struct Parser<'a>(HashMap<ParseKey<'a>, AlternativeRules<'a>>);

#[derive(Debug, Clone, Eq, Hash, PartialEq, PartialOrd)]
pub struct ParseKey<'a>(pub &'a str);

impl<'a> Parser<'a> {
    pub fn new(rule_map: HashMap<ParseKey<'a>, AlternativeRules<'a>>) -> Parser {
        Parser(rule_map)
    }

    fn trim(expr: &str, offset: usize) -> usize {
        offset
            + expr[offset..]
                .chars()
                .take_while(|c| c.is_whitespace())
                .count()
    }

    pub fn parse(&'a self, key: &'a str, expr: &'a str) -> Result<Node<'a>, ParseError> {
        debug!("Parsing expression: '{}'", expr);
        let offset = Parser::trim(expr, 0);

        if expr[offset..].is_empty() {
            debug!("Is empty after trimming, return EmptyLine");
            return Err(ParseError::EmptyInput);
        }

        ParseKey(key)
            .unify_key_move(self, expr, offset)
            .map(|(node, offset)| {
                if offset != expr.len() {
                    Err(ParseError::Trailingcharacters(offset))
                } else {
                    Ok(node)
                }
            })
            .unwrap_or(Err(ParseError::NoRuleMatched))
    }
}

impl<'a> ParseKey<'a> {
    fn unify_key_move(
        self,
        parser: &'a Parser,
        expr: &'a str,
        offset: usize,
    ) -> Option<(Node<'a>, usize)> {
        debug!(
            "Unifying key '{:#?}', for input '{}'",
            self,
            &expr[offset..]
        );
        match parser.0.get(&self) {
            Some(alternative_rules) => alternative_rules
                .iter()
                .find_map(|rule_set| ParseKey::unify_rule_set(parser, rule_set, expr, offset)),
            None => panic!("The parse key map was not defined for key {:#?}", self),
        }
    }

    fn unify_key(
        &'a self,
        parser: &'a Parser,
        expr: &'a str,
        offset: usize,
    ) -> Option<(Node<'a>, usize)> {
        debug!(
            "Unifying key '{:#?}', for input '{}'",
            self,
            &expr[offset..]
        );
        match parser.0.get(self) {
            Some(alternative_rules) => alternative_rules
                .iter()
                .find_map(|rule_set| ParseKey::unify_rule_set(parser, rule_set, expr, offset)),
            None => panic!("The parse key map was not defined for key {:#?}", self),
        }
    }

    fn unify_rule_set(
        parser: &'a Parser,
        rule_set: &'a RuleSet,
        expr: &'a str,
        mut offset: usize,
    ) -> Option<(Node<'a>, usize)> {
        debug!(
            "Unifying ruleset '{:#?}', for input '{}'",
            rule_set,
            &expr[offset..]
        );
        let mut out = Vec::new();
        for rule in rule_set.iter() {
            debug!("Before left trimming: '{}'", &expr[offset..]);
            offset = Parser::trim(expr, offset);
            debug!("After left trimming: '{}'", &expr[offset..]);

            let (node, new_offset) = match rule {
                Rule::NonTerminal(parse_key) => parse_key.unify_key(parser, expr, offset)?,
                Rule::Terminal(parser_fn) => parser_fn(expr, offset)?,
            };
            out.push(node);
            offset = new_offset;
        }

        match out.len() {
            0 => None,
            1 => Some((out.pop().unwrap(), offset)),
            _ => Some((Node::Expr(out), offset)),
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum ParseError {
    #[error("Input is empty (or only contains whitespaces")]
    EmptyInput,
    #[error("Parsing stopped at character {0}, but trailing characters still exist")]
    Trailingcharacters(usize),
    #[error("No rule matched the given input")]
    NoRuleMatched,
}
