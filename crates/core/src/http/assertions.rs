use std::collections::HashMap;

use anyhow::Context;
use hcl::{
    structure::{BlockBuilder, BodyBuilder},
    BlockLabel, Identifier, Value,
};
use serde::Deserialize;
use strum::{Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, Display, EnumString)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum Condition {
    Eq(String, Value),
    Ne(String, Value),

    Gt(String, f64),
    Gte(String, f64),
    Lt(String, f64),
    Lte(String, f64),

    In(String, Vec<Value>),
    NotIn(String, Vec<Value>),

    Contains(String, String),
    NotContains(String, String),
    StartsWith(String, String),
    EndsWith(String, String),
    Matches(String, String),
    NotMatches(String, String),

    Is(String, MatchType),
    IsNot(String, MatchType),
}

impl Condition {
    fn to_string(&self) -> &'static str {
        match self {
            Condition::Eq(_, _) => "eq",
            Condition::Ne(_, _) => "ne",
            Condition::Gt(_, _) => "gt",
            Condition::Gte(_, _) => "gte",
            Condition::Lt(_, _) => "lt",
            Condition::Lte(_, _) => "lte",
            Condition::Contains(_, _) => "contains",
            Condition::NotContains(_, _) => "not_contains",
            Condition::StartsWith(_, _) => "starts_with",
            Condition::EndsWith(_, _) => "ends_with",
            Condition::Matches(_, _) => "matches",
            Condition::NotMatches(_, _) => "not_matches",
            Condition::In(_, _) => "in",
            Condition::NotIn(_, _) => "not_in",
            Condition::Is(_, _) => "is",
            Condition::IsNot(_, _) => "is_not",
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
            let key = key.to_owned();
            let condition = match op.as_str() {
                "eq" => Condition::Eq(key, value.to_owned()),
                "ne" => Condition::Ne(key, value.to_owned()),
                "gt" => Condition::Gt(key, as_f64(value)?),
                "gte" => Condition::Gte(key, as_f64(value)?),
                "lt" => Condition::Lt(key, as_f64(value)?),
                "lte" => Condition::Lte(key, as_f64(value)?),
                "contains" => Condition::Contains(key, as_string(value)?),
                "not_contains" => Condition::NotContains(key, as_string(value)?),
                "starts_with" => Condition::StartsWith(key, as_string(value)?),
                "ends_with" => Condition::EndsWith(key, as_string(value)?),
                "matches" => Condition::Matches(key, as_string(value)?),
                "not_matches" => Condition::NotMatches(key, as_string(value)?),
                "in" => Condition::In(key, as_array(value)?),
                "not_in" => Condition::NotIn(key, as_array(value)?),
                "is" => Condition::Is(key, as_string(value)?.parse()?),
                "is_not" => Condition::IsNot(key, as_string(value)?.parse()?),
                _ => continue, // Ignored
            };
            cons.push(condition);
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
        let op = condition.to_string();

        // Handle multiline strings with heredoc
        let (key, value) = match condition {
            Condition::Eq(k, v) => (k, v),
            Condition::Ne(k, v) => (k, v),
            Condition::Gt(k, v) => (k, v.into()),
            Condition::Gte(k, v) => (k, v.into()),
            Condition::Lt(k, v) => (k, v.into()),
            Condition::Lte(k, v) => (k, v.into()),
            Condition::Contains(k, v) => (k, v.into()),
            Condition::NotContains(k, v) => (k, v.into()),
            Condition::StartsWith(k, v) => (k, v.into()),
            Condition::EndsWith(k, v) => (k, v.into()),
            Condition::Matches(k, v) => (k, v.into()),
            Condition::NotMatches(k, v) => (k, v.into()),
            Condition::In(k, v) => (k, v.into()),
            Condition::NotIn(k, v) => (k, v.into()),
            Condition::Is(k, v) => (k, v.to_string().into()),
            Condition::IsNot(k, v) => (k, v.to_string().into()),
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
