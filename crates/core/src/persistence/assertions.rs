use anyhow::Context;
use hcl::Value;
use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum NumberCondition {
    Eq(f64),
    Ne(f64),
    Gt(f64),
    Gte(f64),
    Lt(f64),
    Lte(f64),
}

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

#[derive(Debug, Clone)]
pub enum Assertion {
    Status(Vec<NumberCondition>),
    Duration(Vec<NumberCondition>),
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

fn parse_number_conditions(value: &Value) -> anyhow::Result<Vec<NumberCondition>> {
    let mut conditions = Vec::new();

    fn parse_condition(conditions: &mut Vec<NumberCondition>, value: &Value) -> anyhow::Result<()> {
        let object = value.as_object().context("Expected Object")?;

        for (op, value) in object {
            let number = value.as_f64().context("Expected Number")?;

            let condition = match op.as_str() {
                "eq" => NumberCondition::Eq(number),
                "ne" => NumberCondition::Ne(number),
                "gt" => NumberCondition::Gt(number),
                "gte" => NumberCondition::Gte(number),
                "lt" => NumberCondition::Lt(number),
                "lte" => NumberCondition::Lte(number),
                _ => return Err(anyhow::anyhow!("Unknown condition: {}", op)),
            };

            conditions.push(condition);
        }
        Ok(())
    }

    match value {
        Value::Array(array) => {
            for value in array {
                parse_condition(&mut conditions, value)?;
            }
        }
        Value::Object(_) => parse_condition(&mut conditions, value)?,
        _ => return Err(anyhow::anyhow!("Expected Array or Object")),
    }

    Ok(conditions)
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
                _ => return Err(anyhow::anyhow!("Unknown condition: {}", key)),
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
        match key.as_str() {
            "status" => {
                let status = parse_number_conditions(value)?;
                assertions.push(Assertion::Status(status));
            }
            "duration" => {
                let duration = parse_number_conditions(value)?;
                assertions.push(Assertion::Duration(duration));
            }
            "header" => {
                let headers = parse_conditions(value)?;
                assertions.push(Assertion::Headers(headers));
            }
            "body" => {
                let body = parse_conditions(value)?;
                assertions.push(Assertion::Body(body));
            }
            _ => {
                return Err(anyhow::anyhow!("Unknown assertion: {}", key));
            }
        }
    }

    Ok(Assertions(assertions))
}
