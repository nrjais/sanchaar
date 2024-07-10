use std::collections::{HashMap, HashSet};

use anyhow::Context;
use hcl::{
    structure::{BlockBuilder, BodyBuilder},
    BlockLabel, Identifier, Number, Value,
};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Condition {
    Eq(String, Value),
    Ne(String, Value),

    Contains(String, String),
    NotContains(String, String),
    StartsWith(String, String),
    EndsWith(String, String),
    Matches(String, String),
}

impl Condition {
    fn to_string(&self) -> &'static str {
        match self {
            Condition::Eq(_, _) => "eq",
            Condition::Ne(_, _) => "ne",
            Condition::Contains(_, _) => "contains",
            Condition::NotContains(_, _) => "not_contains",
            Condition::StartsWith(_, _) => "starts_with",
            Condition::EndsWith(_, _) => "ends_with",
            Condition::Matches(_, _) => "matches",
        }
    }
}

#[derive(Debug, Clone)]
pub enum Assertion {
    Status(Vec<Condition>),
    Duration(Vec<Condition>),
    Headers(Vec<Condition>),
    Body(Vec<Condition>),
}

#[derive(Debug, Clone, Default)]
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
                "contains" => Condition::Contains(key, as_string(value)?),
                "not_contains" => Condition::NotContains(key, as_string(value)?),
                "starts_with" => Condition::StartsWith(key, as_string(value)?),
                "ends_with" => Condition::EndsWith(key, as_string(value)?),
                "matches" => Condition::Matches(key, as_string(value)?),
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

fn as_string(value: &Value) -> anyhow::Result<String> {
    Ok(value.as_str().context("Expected String")?.to_string())
}

pub fn parse(body: Value) -> anyhow::Result<Assertions> {
    let mut assertions = Vec::new();

    let root = body.as_object().context("Expected Object")?;

    for (key, value) in root {
        let condition = match key.as_str() {
            "status" => Assertion::Status(parse_conditions(value)?),
            "duration" => Assertion::Duration(parse_conditions(value)?),
            "header" => Assertion::Headers(parse_conditions(value)?),
            "body" => Assertion::Body(parse_conditions(value)?),
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
        let (key, value) = match condition {
            Condition::Eq(k, v) => (k, v),
            Condition::Ne(k, v) => (k, v),
            Condition::Contains(k, v) => (k, v.into()),
            Condition::NotContains(k, v) => (k, v.into()),
            Condition::StartsWith(k, v) => (k, v.into()),
            Condition::EndsWith(k, v) => (k, v.into()),
            Condition::Matches(k, v) => (k, v.into()),
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

pub fn encode_body(builder: BodyBuilder, assertions: Assertions) -> BodyBuilder {
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
