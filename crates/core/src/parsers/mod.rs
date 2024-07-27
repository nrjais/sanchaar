use pest::{iterators::Pairs, Parser};
use pest_derive::Parser;

// Token types for the parser
#[derive(Debug, PartialEq)]
pub enum Token {
    Text(String),
    Variable(String),
    // Use ! to escape the variable, only first '!' will be stripped
    Escaped(String),
}

#[derive(Debug, PartialEq)]
pub struct Span {
    token: Token,
    start: usize,
    end: usize,
}

#[derive(Parser)]
#[grammar = "parsers/template.pest"]
struct TemplateParser;

pub fn parse_template(text: &str) -> Vec<Span> {
    let pairs = TemplateParser::parse(Rule::template, text).unwrap();

    let mut tokens = Vec::new();

    add_template_tokens(pairs, &mut tokens);

    tokens
}

fn add_template_tokens(pairs: Pairs<Rule>, tokens: &mut Vec<Span>) {
    for pair in pairs {
        match pair.as_rule() {
            Rule::template => add_template_tokens(pair.into_inner(), tokens),
            Rule::text => {
                tokens.push(Span {
                    token: Token::Text(pair.as_str().to_string()),
                    start: pair.as_span().start(),
                    end: pair.as_span().end(),
                });
            }
            Rule::variable => {
                let span = pair.as_span();
                let inner = pair.into_inner();
                for inner_pair in inner {
                    match inner_pair.as_rule() {
                        Rule::ident => {
                            let var = inner_pair.as_str().to_string();
                            if var.starts_with('!') {
                                tokens.push(Span {
                                    token: Token::Escaped(var[1..].to_string()),
                                    start: span.start(),
                                    end: span.end(),
                                });
                            } else {
                                tokens.push(Span {
                                    token: Token::Variable(var),
                                    start: span.start(),
                                    end: span.end(),
                                });
                            }
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_template() {
        let text = "Hello, {{name}}!";
        let tokens = parse_template(text);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::Text("Hello, ".to_string()));
        assert_eq!(tokens[1].token, Token::Variable("name".to_string()));
        assert_eq!(tokens[2].token, Token::Text("!".to_string()));

        let text = "Hello, {{name}}! text";
        let tokens = parse_template(text);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::Text("Hello, ".to_string()));
        assert_eq!(tokens[1].token, Token::Variable("name".to_string()));
        assert_eq!(tokens[2].token, Token::Text("! text".to_string()));

        let text = "{{name}}! {{age}}";
        let tokens = parse_template(text);
        assert_eq!(tokens.len(), 3);
        assert_eq!(tokens[0].token, Token::Variable("name".to_string()));
        assert_eq!(tokens[1].token, Token::Text("! ".to_string()));
        assert_eq!(tokens[2].token, Token::Variable("age".to_string()));

        let text = "Hello, {{!name}}! {{age}}";
        let tokens = parse_template(text);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Text("Hello, ".to_string()));
        assert_eq!(tokens[1].token, Token::Escaped("name".to_string()));
        assert_eq!(tokens[2].token, Token::Text("! ".to_string()));
        assert_eq!(tokens[3].token, Token::Variable("age".to_string()));

        let text = "Hello, {{!!name}}! {{age}}";
        let tokens = parse_template(text);
        assert_eq!(tokens.len(), 4);
        assert_eq!(tokens[0].token, Token::Text("Hello, ".to_string()));
        assert_eq!(tokens[1].token, Token::Escaped("!name".to_string()));
        assert_eq!(tokens[2].token, Token::Text("! ".to_string()));
        assert_eq!(tokens[3].token, Token::Variable("age".to_string()));
    }
}
