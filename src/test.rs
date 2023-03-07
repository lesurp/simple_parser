use crate::*;

fn math_expression_parsing() -> Parser<'static> {
    let rule_map = grammar! {
        expr => {  [term, add_op, expr] | term },
        term => { [fact, mul_op, term] | fact },
        fact => { number | ["(", expr, ")"] },
        add_op => { "+" | "-" },
        mul_op => { "*" | "/" },
        number => {[signed!()]},
    };

    Parser::new(rule_map)
}

#[test]
fn basic_expression() {
    let parser = math_expression_parsing();
    let expr = parser.parse("expr", "3+2").unwrap();
    assert_eq!(
        expr,
        Node::Expr(vec![
            Node::LiteralSigned(3),
            Node::Token("+"),
            Node::LiteralSigned(2)
        ])
    );
}

#[test]
fn precedence_1() {
    let parser = math_expression_parsing();
    let expr = parser.parse("expr", "3+2*4").unwrap();
    assert_eq!(
        expr,
        Node::Expr(vec![
            Node::LiteralSigned(3),
            Node::Token("+"),
            Node::Expr(vec![
                Node::LiteralSigned(2),
                Node::Token("*"),
                Node::LiteralSigned(4)
            ])
        ])
    );
}

#[test]
fn precedence_2() {
    let parser = math_expression_parsing();
    let expr = parser.parse("expr", "3*2+4").unwrap();
    assert_eq!(
        expr,
        Node::Expr(vec![
            Node::Expr(vec![
                Node::LiteralSigned(3),
                Node::Token("*"),
                Node::LiteralSigned(2)
            ]),
            Node::Token("+"),
            Node::LiteralSigned(4),
        ])
    );
}

#[test]
fn parentheses_1() {
    let parser = math_expression_parsing();
    let expr = parser.parse("expr", "3*(2+4)").unwrap();
    assert_eq!(
        expr,
        Node::Expr(vec![
            Node::LiteralSigned(3),
            Node::Token("*"),
            Node::Expr(vec![
                Node::Token("("),
                Node::Expr(vec![
                    Node::LiteralSigned(2),
                    Node::Token("+"),
                    Node::LiteralSigned(4),
                ]),
                Node::Token(")"),
            ]),
        ])
    );
}

#[test]
fn parentheses_2() {
    let parser = math_expression_parsing();
    let expr = parser.parse("expr", "(3*2)+4").unwrap();
    assert_eq!(
        expr,
        Node::Expr(vec![
            Node::Expr(vec![
                Node::Token("("),
                Node::Expr(vec![
                    Node::LiteralSigned(3),
                    Node::Token("*"),
                    Node::LiteralSigned(2),
                ]),
                Node::Token(")"),
            ]),
            Node::Token("+"),
            Node::LiteralSigned(4),
        ])
    );
}
