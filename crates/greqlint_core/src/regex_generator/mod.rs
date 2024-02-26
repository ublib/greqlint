#![allow(dead_code)]
#![allow(unused_imports)]

use crate::parser::ast::{Root, RuleNode};
use regex::Regex;

const ESCAPE_KEYWORDS: [char; 2] = ['(', ')'];

pub fn gen_regex(ast: &Root) -> Regex {
    let result = gen_regex_str(ast);
    Regex::new(&result).unwrap()
}

fn gen_regex_str(ast: &Root) -> String {
    let mut result = String::new();
    for node in ast.body.clone() {
        match node {
            RuleNode::Variable(var) => {
                result.push_str(&format!("(?<{}>.+)", var.name));
            }
            RuleNode::Literal(lit) => {
                result.push_str(escape_keyword(lit.value).as_str());
            }
        }
    }
    result
}

fn escape_keyword(keyword: &str) -> String {
    let mut result = String::new();
    for c in keyword.chars() {
        if ESCAPE_KEYWORDS.contains(&c) {
            result.push('\\');
        }
        result.push(c);
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use regex::Regex;

    use crate::parser::ast::{Literal, Root, RuleNode, Span, Variable};

    /// ${type}\(${scope}\): .+
    /// ↓
    /// ast: [Variable("type"), Literal("\("), Variable("scope"), Literal("\):"), Literal(".+")]
    /// ↓
    /// r#"(?<type>.+)\((?<scope>.+)\): .+"#
    #[test]
    fn test_gen_regex() {
        let ast = Root {
            span: Span { start: 0, end: 16 },
            body: vec![
                RuleNode::Variable(Variable {
                    span: Span { start: 0, end: 5 },
                    name: "type",
                }),
                RuleNode::Literal(Literal {
                    span: Span { start: 5, end: 6 },
                    value: "(",
                }),
                RuleNode::Variable(Variable {
                    span: Span { start: 6, end: 11 },
                    name: "scope",
                }),
                RuleNode::Literal(Literal {
                    span: Span { start: 11, end: 16 },
                    value: "): .+",
                }),
            ],
        };

        let re = gen_regex_str(&ast);
        assert_eq!(re, r#"(?<type>.+)\((?<scope>.+)\): .+"#);

        let re = gen_regex(&ast);
        let caps = re.captures("feat(scope): message").unwrap();
        assert_eq!(caps.name("type").unwrap().as_str(), "feat");
    }
}
