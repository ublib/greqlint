use self::{
    ast::{Literal, Root, RuleNode, Span, Variable},
    reader::Reader,
};

pub mod ast;
mod reader;

pub fn parse(input: &str) -> Result<Root, String> {
    Parser::new(input).parse()
}

pub struct Parser<'a> {
    source: &'a str,
    reader: Reader<'a>,
}

impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        Parser {
            source: input,
            reader: Reader::new(input),
        }
    }

    pub fn parse(&mut self) -> Result<Root<'a>, String> {
        let mut body = Vec::new();
        let start = self.reader.pos;
        while self.reader.peek().is_some() {
            body.push(self.parse_rule()?);
        }
        let end = self.reader.pos;
        let span = Span { start, end };
        Ok(Root { span, body })
    }

    fn parse_rule(&mut self) -> Result<RuleNode<'a>, String> {
        match (self.reader.peek_n(1), self.reader.peek_n(2)) {
            (Some('$'), Some('{')) => self.parse_variable(),
            (Some(_), _) => self.parse_literal(),
            (None, _) => Err("Unexpected EOF".to_string()),
        }
    }

    fn parse_literal(&mut self) -> Result<RuleNode<'a>, String> {
        let start = self.reader.pos;
        // while '${'
        loop {
            match (self.reader.peek_n(1), self.reader.peek_n(2)) {
                (Some('$'), Some('{')) => break,
                (Some(_), _) => self.reader.next(),
                (None, _) => break,
            };
        }
        let end = self.reader.pos;
        let span = Span { start, end };
        let value = &self.source[start..end];
        Ok(RuleNode::Literal(Literal { span, value }))
    }

    fn parse_variable(&mut self) -> Result<RuleNode<'a>, String> {
        let start = self.reader.pos;
        self.reader.next();
        self.reader.next();
        let name_span = self.parse_variable_name()?;
        let name = &self.source[name_span.start..name_span.end];
        if self.reader.peek() != Some('}') {
            return Err("Expected }".to_string());
        }
        self.reader.next(); // skip }
        let end = self.reader.pos;
        let span = Span { start, end };
        Ok(RuleNode::Variable(Variable { span, name }))
    }

    fn parse_variable_name(&mut self) -> Result<Span, String> {
        let start = self.reader.pos;
        while let Some(c) = self.reader.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.reader.next();
            }

            if c == '}' {
                break;
            }
        }
        let end = self.reader.pos;
        let span = Span { start, end };
        Ok(span)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_only_literal() {
        let mut parser = Parser::new("abc");
        assert_eq!(
            parser.parse(),
            Ok(Root {
                span: Span { start: 0, end: 3 },
                body: vec![RuleNode::Literal(Literal {
                    span: Span { start: 0, end: 3 },
                    value: "abc",
                })]
            })
        );
    }

    #[test]
    fn test_parse_only_variable() {
        let mut parser = Parser::new("${abc}");
        assert_eq!(
            parser.parse(),
            Ok(Root {
                span: Span { start: 0, end: 6 },
                body: vec![RuleNode::Variable(Variable {
                    span: Span { start: 0, end: 6 },
                    name: "abc",
                })]
            })
        );
    }

    #[test]
    fn test_parse_only_variables() {
        let mut parser = Parser::new("${abc}${def}${ghi}");
        assert_eq!(
            parser.parse(),
            Ok(Root {
                span: Span { start: 0, end: 18 },
                body: vec![
                    RuleNode::Variable(Variable {
                        span: Span { start: 0, end: 6 },
                        name: "abc",
                    }),
                    RuleNode::Variable(Variable {
                        span: Span { start: 6, end: 12 },
                        name: "def",
                    }),
                    RuleNode::Variable(Variable {
                        span: Span { start: 12, end: 18 },
                        name: "ghi",
                    })
                ]
            })
        );
    }

    #[test]
    fn test_parse_literal_and_variable() {
        let mut parser = Parser::new("abc${def}ghi");
        assert_eq!(
            parser.parse(),
            Ok(Root {
                span: Span { start: 0, end: 12 },
                body: vec![
                    RuleNode::Literal(Literal {
                        span: Span { start: 0, end: 3 },
                        value: "abc",
                    }),
                    RuleNode::Variable(Variable {
                        span: Span { start: 3, end: 9 },
                        name: "def",
                    }),
                    RuleNode::Literal(Literal {
                        span: Span { start: 9, end: 12 },
                        value: "ghi",
                    })
                ]
            })
        );
    }
}
