#[derive(Debug, PartialEq, Clone)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Root<'a> {
    pub span: Span,
    pub body: Vec<RuleNode<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RuleNode<'a> {
    Variable(Variable<'a>),
    Literal(Literal<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Variable<'a> {
    pub span: Span,
    pub name: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Literal<'a> {
    pub span: Span,
    pub value: &'a str,
}
