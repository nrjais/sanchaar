use std::{collections::HashMap, str::from_utf8};

use regex::Regex;
use serde_json::{Number, Value};

use crate::client::Response;

use super::{Assertion, Assertions, Condition, MatchType, Matcher};

#[derive(Debug, Clone)]
pub enum MatcherResult {
    Passed,
    Failed(Description),
}

#[derive(Debug, Clone)]
pub struct Description {
    pub summary: String,
    pub expected: Value,
    pub actual: Option<Value>,
}

#[derive(Debug, Clone)]
pub struct ConditionResult {
    pub name: String,
    pub result: MatcherResult,
}

#[derive(Debug, Clone)]
pub struct AssertionOutcome {
    pub name: String,
    pub results: Vec<ConditionResult>,
}

pub fn run(response: &Response, assertions: &Assertions) -> Vec<AssertionOutcome> {
    let mut report = Vec::new();

    for assertion in assertions.0.iter() {
        let results = match assertion {
            Assertion::Status(conditions) => {
                let status = response.status;
                match_conditions(conditions, |key| match key {
                    "code" => Some(Value::Number(Number::from(status.as_u16()))),
                    "text" => Some(status.to_string().into()),
                    _ => None,
                })
            }
            Assertion::Duration(conditions) => {
                let duration = response.duration;
                let millis = duration.as_millis() as f64;
                match_conditions(conditions, |key| match key {
                    "seconds" => Some(Value::Number(Number::from(duration.as_secs()))),
                    "millis" | "ms" => Some(millis.into()),
                    _ => None,
                })
            }
            Assertion::Headers(conditions) => {
                let headers = &response.headers;
                let headers = headers
                    .iter()
                    .map(|(k, v)| (k.as_str(), v.to_str().unwrap_or("<Not UTF8>")))
                    .map(|(k, v)| (k.to_ascii_lowercase(), Value::from(v)))
                    .collect::<HashMap<_, _>>();

                match_conditions(conditions, |key| {
                    let lower = key.to_ascii_lowercase();
                    headers.get(&lower).cloned()
                })
            }
            Assertion::Body(conditions) => match_conditions(conditions, |key| match key {
                "string" => from_utf8(&response.body.data).ok().map(Value::from),
                _ => None,
            }),
        };

        report.push(AssertionOutcome {
            name: assertion.name(),
            results,
        });
    }

    report
}

fn to_value(value: &toml::Value) -> Value {
    match value {
        toml::Value::Boolean(b) => Value::Bool(*b),
        toml::Value::Float(f) => Value::Number(Number::from_f64(*f).unwrap()),
        toml::Value::Integer(i) => Value::Number(Number::from(*i)),
        toml::Value::String(s) => Value::String(s.clone()),
        toml::Value::Array(a) => Value::Array(a.iter().map(to_value).collect()),
        toml::Value::Table(t) => {
            Value::Object(t.iter().map(|(k, v)| (k.clone(), to_value(v))).collect())
        }
        toml::Value::Datetime(datetime) => Value::String(datetime.to_string()),
    }
}

fn match_conditions(
    conditions: &[Condition],
    get_value: impl Fn(&str) -> Option<Value>,
) -> Vec<ConditionResult> {
    let mut results = Vec::new();
    for condition in conditions {
        let Condition { key, matcher } = condition;
        let actual = get_value(key);
        let actual = actual.as_ref();

        let result = match matcher {
            Matcher::Eq(expected) => equal(actual, &to_value(expected)),
            Matcher::Ne(expected) => not_equal(actual, &to_value(expected)),
            Matcher::Gt(expected) => greater_than(actual, *expected),
            Matcher::Gte(expected) => greater_than_or_equal(actual, *expected),
            Matcher::Lt(expected) => less_than(actual, *expected),
            Matcher::Lte(expected) => less_than_or_equal(actual, *expected),
            Matcher::In(expected) => {
                in_list(actual, &expected.iter().map(to_value).collect::<Vec<_>>())
            }
            Matcher::NotIn(expected) => {
                not_in_list(actual, &expected.iter().map(to_value).collect::<Vec<_>>())
            }
            Matcher::Contains(expected) => contains(actual, expected),
            Matcher::NotContains(expected) => not_contains(actual, expected),
            Matcher::StartsWith(expected) => starts_with(actual, expected),
            Matcher::EndsWith(expected) => ends_with(actual, expected),
            Matcher::Matches(expected) => matches(actual, expected),
            Matcher::NotMatches(expected) => not_matches(actual, expected),
            Matcher::Is(expected) => is_matching(actual, *expected),
            Matcher::IsNot(expected) => is_not_type(actual, *expected),
        };

        results.push(ConditionResult {
            name: format!("that {} {}", key, matcher.describe()),
            result,
        })
    }

    results
}

fn description(op: &str, expected: &Value, actual: Option<&Value>) -> Description {
    Description {
        summary: op.to_string(),
        expected: expected.clone(),
        actual: actual.cloned(),
    }
}

fn equal(actual: Option<&Value>, expected: &Value) -> MatcherResult {
    match actual {
        Some(actual) if actual == expected => MatcherResult::Passed,
        Some(actual) => {
            MatcherResult::Failed(description("to be equal to", expected, Some(actual)))
        }
        None => MatcherResult::Failed(description("to be equal to", expected, None)),
    }
}

fn not_equal(actual: Option<&Value>, expected: &Value) -> MatcherResult {
    match actual {
        Some(actual) if actual != expected => MatcherResult::Passed,
        Some(actual) => {
            MatcherResult::Failed(description("does not equal", expected, Some(actual)))
        }
        None => MatcherResult::Failed(description("does not equal", expected, None)),
    }
}

fn value_to_f64(v: &Value) -> Option<f64> {
    match v {
        Value::Number(n) => Some(n.as_f64().unwrap_or(f64::NAN)),
        _ => None,
    }
}

fn greater_than(actual: Option<&Value>, expected: f64) -> MatcherResult {
    let Some(actual) = actual.and_then(value_to_f64) else {
        return MatcherResult::Failed(description(
            "to be a number",
            &Value::from(expected),
            actual,
        ));
    };

    if actual > expected {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to be greater than",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn greater_than_or_equal(actual: Option<&Value>, expected: f64) -> MatcherResult {
    let Some(actual) = actual.and_then(value_to_f64) else {
        return MatcherResult::Failed(description(
            "to be a number",
            &Value::from(expected),
            actual,
        ));
    };

    if actual >= expected {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to be greater than or equal to",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn less_than(actual: Option<&Value>, expected: f64) -> MatcherResult {
    let Some(actual) = actual.and_then(value_to_f64) else {
        return MatcherResult::Failed(description(
            "to be a number",
            &Value::from(expected),
            actual,
        ));
    };

    if actual < expected {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to be less than",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn less_than_or_equal(actual: Option<&Value>, expected: f64) -> MatcherResult {
    let Some(actual) = actual.and_then(value_to_f64) else {
        return MatcherResult::Failed(description(
            "to be a number",
            &Value::from(expected),
            actual,
        ));
    };

    if actual <= expected {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to be less than or equal to",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn in_list(actual: Option<&Value>, expected: &[Value]) -> MatcherResult {
    let null_value = Value::Null;
    let actual = actual.unwrap_or(&null_value);

    if expected.contains(actual) {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to be in",
            &Value::Array(expected.to_owned()),
            Some(actual),
        ))
    }
}

fn not_in_list(actual: Option<&Value>, expected: &[Value]) -> MatcherResult {
    let null_value = Value::Null;
    let actual = actual.unwrap_or(&null_value);

    if !expected.contains(actual) {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to not be in",
            &Value::Array(expected.to_owned()),
            Some(actual),
        ))
    }
}

fn contains(actual: Option<&Value>, expected: &str) -> MatcherResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return MatcherResult::Failed(description(
            "to be a string",
            &Value::from(expected),
            actual,
        ));
    };

    if actual.contains(expected) {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to contain",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn not_contains(actual: Option<&Value>, expected: &str) -> MatcherResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return MatcherResult::Failed(description(
            "to be a string",
            &Value::from(expected),
            actual,
        ));
    };

    if !actual.contains(expected) {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to not contain",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn starts_with(actual: Option<&Value>, expected: &str) -> MatcherResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return MatcherResult::Failed(description(
            "to be a string",
            &Value::from(expected),
            actual,
        ));
    };

    if actual.starts_with(expected) {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to start with",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn ends_with(actual: Option<&Value>, expected: &str) -> MatcherResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return MatcherResult::Failed(description(
            "to be a string",
            &Value::from(expected),
            actual,
        ));
    };

    if actual.ends_with(expected) {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to end with",
            &Value::from(expected),
            Some(&Value::from(actual)),
        ))
    }
}

fn matches(actual: Option<&Value>, expected: &str) -> MatcherResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return MatcherResult::Failed(description(
            "to be a string",
            &Value::from(expected),
            actual,
        ));
    };

    Regex::new(expected)
        .map(|re| re.is_match(actual))
        .map_or_else(
            |_err| {
                MatcherResult::Failed(description(
                    "to be a valid regex",
                    &Value::from(expected),
                    Some(&actual.into()),
                ))
            },
            |matches| {
                if matches {
                    MatcherResult::Passed
                } else {
                    MatcherResult::Failed(description(
                        "to match",
                        &Value::from(expected),
                        Some(&actual.into()),
                    ))
                }
            },
        )
}

fn not_matches(actual: Option<&Value>, expected: &str) -> MatcherResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return MatcherResult::Failed(description(
            "to be a string",
            &Value::from(expected),
            actual,
        ));
    };

    Regex::new(expected)
        .map(|re| !re.is_match(actual))
        .map_or_else(
            |_err| {
                MatcherResult::Failed(description(
                    "to be a valid regex",
                    &Value::from(expected),
                    Some(&actual.into()),
                ))
            },
            |matches| {
                if matches {
                    MatcherResult::Passed
                } else {
                    MatcherResult::Failed(description(
                        "to not match",
                        &Value::from(expected),
                        Some(&actual.into()),
                    ))
                }
            },
        )
}

fn is_matching(actual_opt: Option<&Value>, expected: MatchType) -> MatcherResult {
    let null_value = Value::Null;
    let actual = actual_opt.unwrap_or(&null_value);

    let pass = match expected {
        MatchType::Undefined => actual_opt.is_none(),
        MatchType::Null => actual.is_null(),
        MatchType::Bool => actual.is_boolean(),
        MatchType::Number => actual.is_number(),
        MatchType::String => actual.is_string(),
        MatchType::Array => actual.is_array(),
        MatchType::Object => actual.is_object(),
        MatchType::Empty => match actual {
            Value::Array(arr) => arr.is_empty(),
            Value::String(s) => s.is_empty(),
            _ => false,
        },
    };

    if pass {
        MatcherResult::Passed
    } else {
        MatcherResult::Failed(description(
            "to be of type",
            &Value::from(expected.to_string()),
            actual_opt,
        ))
    }
}

fn is_not_type(actual_opt: Option<&Value>, expected: MatchType) -> MatcherResult {
    let matching = is_matching(actual_opt, expected);

    match matching {
        MatcherResult::Passed => MatcherResult::Failed(description(
            "to not be",
            &Value::from(expected.to_string()),
            actual_opt,
        )),
        MatcherResult::Failed(_) => MatcherResult::Passed,
    }
}
