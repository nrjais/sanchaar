pub mod runner;

pub use runner::run;

use std::collections::HashMap;

use anyhow::Context;
use hcl::{
    BlockLabel, Identifier, Value,
    structure::{BlockBuilder, BodyBuilder},
};
use serde::Deserialize;
use strum::{Display, EnumString};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, EnumString)]
pub enum MatchType {
    Null,
    Undefined,
    Bool,
    Number,
    String,
    Array,
    Object,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Display)]
pub enum Matcher {
    Eq(Value),
    Ne(Value),

    Gt(f64),
    Gte(f64),
    Lt(f64),
    Lte(f64),

    In(Vec<Value>),
    NotIn(Vec<Value>),

    Contains(String),
    NotContains(String),
    StartsWith(String),
    EndsWith(String),
    Matches(String),
    NotMatches(String),

    Is(MatchType),
    IsNot(MatchType),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Condition {
    key: String,
    matcher: Matcher,
}

fn to_string_vec(val: &[Value]) -> String {
    let vals = val
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(", ");
    format!("[{}]", vals)
}

impl Matcher {
    fn to_string(&self) -> &'static str {
        match self {
            Matcher::Eq(_) => "eq",
            Matcher::Ne(_) => "ne",
            Matcher::Gt(_) => "gt",
            Matcher::Gte(_) => "gte",
            Matcher::Lt(_) => "lt",
            Matcher::Lte(_) => "lte",
            Matcher::Contains(_) => "contains",
            Matcher::NotContains(_) => "not_contains",
            Matcher::StartsWith(_) => "starts_with",
            Matcher::EndsWith(_) => "ends_with",
            Matcher::Matches(_) => "matches",
            Matcher::NotMatches(_) => "not_matches",
            Matcher::In(_) => "in",
            Matcher::NotIn(_) => "not_in",
            Matcher::Is(_) => "is",
            Matcher::IsNot(_) => "is_not",
        }
    }

    fn describe(&self) -> String {
        match self {
            Matcher::Eq(exp) => format!("{} {}", self, exp),
            Matcher::Ne(exp) => format!("{} {}", self, exp),
            Matcher::Gt(exp) => format!("{} {}", self, exp),
            Matcher::Gte(exp) => format!("{} {}", self, exp),
            Matcher::Lt(exp) => format!("{} {}", self, exp),
            Matcher::Lte(exp) => format!("{} {}", self, exp),
            Matcher::In(exp) => format!("{} {}", self, to_string_vec(exp)),
            Matcher::NotIn(exp) => format!("{} {}", self, to_string_vec(exp)),
            Matcher::Contains(exp) => format!("{} {}", self, exp),
            Matcher::NotContains(exp) => format!("{} {}", self, exp),
            Matcher::StartsWith(exp) => format!("{} {}", self, exp),
            Matcher::EndsWith(exp) => format!("{} {}", self, exp),
            Matcher::Matches(exp) => format!("{} {}", self, exp),
            Matcher::NotMatches(exp) => format!("{} {}", self, exp),
            Matcher::Is(exp) => format!("{} {}", self, exp),
            Matcher::IsNot(exp) => format!("{} {}", self, exp),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Assertion {
    Status(Vec<Condition>),
    Duration(Vec<Condition>),
    Headers(Vec<Condition>),
    Body(Vec<Condition>),
}
impl Assertion {
    fn name(&self) -> String {
        match self {
            Assertion::Status(_) => "Status".to_string(),
            Assertion::Duration(_) => "Response Duration".to_string(),
            Assertion::Headers(_) => "Headers".to_string(),
            Assertion::Body(_) => "Body".to_string(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Assertions(pub Vec<Assertion>);

impl<'de> Deserialize<'de> for Assertions {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer).map_err(serde::de::Error::custom)?;
        parse(value).map_err(serde::de::Error::custom)
    }
}

fn parse_conditions(value: &Value) -> anyhow::Result<Vec<Condition>> {
    let mut conditions = Vec::new();

    fn parse_condition(cons: &mut Vec<Condition>, key: &str, obj: &Value) -> anyhow::Result<()> {
        let object = obj.as_object().context("Expected Object")?;
        for (op, value) in object {
            let matcher = match op.as_str() {
                "eq" => Matcher::Eq(value.to_owned()),
                "ne" => Matcher::Ne(value.to_owned()),
                "gt" => Matcher::Gt(as_f64(value)?),
                "gte" => Matcher::Gte(as_f64(value)?),
                "lt" => Matcher::Lt(as_f64(value)?),
                "lte" => Matcher::Lte(as_f64(value)?),
                "contains" => Matcher::Contains(as_string(value)?),
                "not_contains" => Matcher::NotContains(as_string(value)?),
                "starts_with" => Matcher::StartsWith(as_string(value)?),
                "ends_with" => Matcher::EndsWith(as_string(value)?),
                "matches" => Matcher::Matches(as_string(value)?),
                "not_matches" => Matcher::NotMatches(as_string(value)?),
                "in" => Matcher::In(as_array(value)?),
                "not_in" => Matcher::NotIn(as_array(value)?),
                "is" => Matcher::Is(as_string(value)?.parse()?),
                "is_not" => Matcher::IsNot(as_string(value)?.parse()?),
                _ => continue, // Ignored
            };
            let key = key.to_owned();
            cons.push(Condition { key, matcher });
        }
        Ok(())
    }

    let object = value.as_object().context("Expected Object")?;
    for (key, value) in object {
        match value {
            Value::Array(array) => {
                for value in array {
                    parse_condition(&mut conditions, key, value)?;
                }
            }
            Value::Object(_) => parse_condition(&mut conditions, key, value)?,
            _ => return Err(anyhow::anyhow!("Expected Array or Object")),
        }
    }

    Ok(conditions)
}

fn as_f64(value: &Value) -> anyhow::Result<f64> {
    value.as_f64().context("Expected Number")
}

fn as_string(value: &Value) -> anyhow::Result<String> {
    Ok(value.as_str().context("Expected String")?.to_string())
}

fn as_array(value: &Value) -> anyhow::Result<Vec<Value>> {
    Ok(value.as_array().context("Expected Array")?.to_owned())
}

pub fn parse(body: Value) -> anyhow::Result<Assertions> {
    let mut assertions = Vec::new();

    let root = body.as_object().context("Expected Object")?;
    for (key, value) in root {
        let matchers = parse_conditions(value)?;
        let condition = match key.as_str() {
            "status" => Assertion::Status(matchers),
            "duration" => Assertion::Duration(matchers),
            "header" => Assertion::Headers(matchers),
            "body" => Assertion::Body(matchers),
            _ => continue, // Ignored
        };

        assertions.push(condition);
    }

    Ok(Assertions(assertions))
}

fn encode_condition_block(
    builder: BlockBuilder,
    name: &'static str,
    conditions: Vec<Condition>,
) -> BlockBuilder {
    let mut root = builder;
    let mut blocks = HashMap::new();

    for condition in conditions.into_iter() {
        let Condition { key, matcher } = condition;
        let op = matcher.to_string();

        // Handle multiline strings with heredoc
        let value = match matcher {
            Matcher::Eq(v) => v,
            Matcher::Ne(v) => v,
            Matcher::Gt(v) => v.into(),
            Matcher::Gte(v) => v.into(),
            Matcher::Lt(v) => v.into(),
            Matcher::Lte(v) => v.into(),
            Matcher::Contains(v) => v.into(),
            Matcher::NotContains(v) => v.into(),
            Matcher::StartsWith(v) => v.into(),
            Matcher::EndsWith(v) => v.into(),
            Matcher::Matches(v) => v.into(),
            Matcher::NotMatches(v) => v.into(),
            Matcher::In(v) => v.into(),
            Matcher::NotIn(v) => v.into(),
            Matcher::Is(v) => v.to_string().into(),
            Matcher::IsNot(v) => v.to_string().into(),
        };

        let mut block = blocks.remove(&key).unwrap_or_else(|| {
            let label = Identifier::new(key.clone())
                .map(BlockLabel::Identifier)
                .unwrap_or(BlockLabel::String(key.clone()));

            BlockBuilder::new(name).add_label(label)
        });

        block = block.add_attribute((Identifier::sanitized(op), value));
        blocks.insert(key, block);
    }

    for (_, block) in blocks {
        root = root.add_block(block.build());
    }

    root
}

pub fn encode(builder: BodyBuilder, assertions: Assertions) -> BodyBuilder {
    if assertions.0.is_empty() {
        return builder;
    }

    let mut root = BlockBuilder::new("assertions");

    for assertion in assertions.0.into_iter() {
        root = match assertion {
            Assertion::Status(status) => encode_condition_block(root, "status", status),
            Assertion::Duration(duration) => encode_condition_block(root, "duration", duration),
            Assertion::Headers(headers) => encode_condition_block(root, "header", headers),
            Assertion::Body(body) => encode_condition_block(root, "body", body),
        };
    }

    builder.add_block(root.build())
}
