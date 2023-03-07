#![allow(incomplete_features)]
#![feature(adt_const_params)]

use log::debug;
use std::{collections::HashMap, fmt::Debug};

#[macro_use]
pub mod primitives;
#[macro_use]
pub mod grammar;

#[derive(Debug, Clone)]
pub enum Node<'a> {
    LiteralStr(&'a str),
    LiteralUnsigned(usize),
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

    pub fn parse(&'a self, key: &'a str, expr: &'a str) -> ParseResult<'a> {
        debug!("Parsing expression: '{}'", expr);
        if expr.is_empty() {
            debug!("Is empty, return EOF");
            return ParseResult::EoF;
        }

        let offset = Parser::trim(expr, 0);

        if expr[offset..].is_empty() {
            debug!("Is empty after trimming, return EmptyLine");
            return ParseResult::EmptyLine;
        }

        ParseKey(key)
            .unify_key_move(self, expr, offset)
            .map(|(node, _)| ParseResult::OK(node))
            .unwrap_or(ParseResult::Err)
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

#[derive(Clone, Debug)]
pub enum ParseResult<'a> {
    EmptyLine,
    EoF,
    OK(Node<'a>),
    Err,
}

pub fn parse_token<const TOKEN: &'static str>(
    expr: &str,
    offset: usize,
) -> Option<(Node<'_>, usize)> {
    debug!("Parsing token '{}' from input '{}'", TOKEN, &expr[offset..]);
    if expr[offset..].starts_with(TOKEN) {
        debug!("\tOK");
        Some((Node::Token(TOKEN), offset + TOKEN.len()))
    } else {
        debug!("\tErr");
        None
    }
}
