use std::{collections::HashMap, fmt::format};

use hcl::Value;
use regex::Regex;

use crate::client::Response;

use super::{Assertion, Assertions, Condition, MatchType, Matcher};

pub enum ConditionResult {
    Passed,
    Failed(String),
}

pub struct AssertionReport {
    name: String,
    result: ConditionResult,
}

pub fn run(response: &Response, assertions: &Assertions) -> Vec<AssertionReport> {
    let mut report = Vec::new();

    for assertion in assertions.0.iter() {
        let results = match assertion {
            Assertion::Status(conditions) => {
                let status = response.status;
                match_conditions(conditions, {
                    let mut map = HashMap::new();
                    map.insert("code".into(), status.as_u16().into());
                    map.insert("text".into(), status.to_string().into());
                    map.insert("success".into(), status.is_success().into());
                    map
                })
            }
            Assertion::Duration(_) => todo!(),
            Assertion::Headers(_) => todo!(),
            Assertion::Body(_) => todo!(),
        };

        report.extend(results);
    }

    report
}

fn match_conditions(
    conditions: &[Condition],
    values: HashMap<String, Value>,
) -> Vec<AssertionReport> {
    let mut results = Vec::new();
    for condition in conditions {
        let Condition { key, matcher } = condition;
        let actual = values.get(key);

        let result = match matcher {
            Matcher::Eq(expected) => equal(actual, expected),
            Matcher::Ne(expected) => not_equal(actual, expected),
            Matcher::Gt(expected) => greater_than(actual, *expected),
            Matcher::Gte(expected) => greater_than_or_equal(actual, *expected),
            Matcher::Lt(expected) => less_than(actual, *expected),
            Matcher::Lte(expected) => less_than_or_equal(actual, *expected),
            Matcher::In(expected) => in_list(actual, expected),
            Matcher::NotIn(expected) => not_in_list(actual, expected),
            Matcher::Contains(expected) => contains(actual, expected),
            Matcher::NotContains(expected) => not_contains(actual, expected),
            Matcher::StartsWith(expected) => starts_with(actual, expected),
            Matcher::EndsWith(expected) => ends_with(actual, expected),
            Matcher::Matches(expected) => matches(actual, expected),
            Matcher::NotMatches(expected) => not_matches(actual, expected),
            Matcher::Is(expected) => is_matching(actual, *expected),
            Matcher::IsNot(expected) => is_not_type(actual, *expected),
        };

        results.push(AssertionReport {
            name: format!("{} - {}", key, matcher),
            result,
        })
    }

    results
}

fn equal(actual: Option<&Value>, expected: &Value) -> ConditionResult {
    match actual {
        Some(actual) if actual == expected => ConditionResult::Passed,
        Some(actual) => ConditionResult::Failed(format!("Expected {} == {}", expected, actual)),
        None => ConditionResult::Failed(format!("Expected {} but got nothing", expected)),
    }
}

fn not_equal(actual: Option<&Value>, expected: &Value) -> ConditionResult {
    match actual {
        Some(actual) if actual != expected => ConditionResult::Passed,
        Some(actual) => ConditionResult::Failed(format!("Expected {} != {}", expected, actual)),
        None => ConditionResult::Failed(format!("Expected {} but got nothing", expected)),
    }
}

fn greater_than(actual: Option<&Value>, expected: f64) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_f64()) else {
        return ConditionResult::Failed(format!(
            "Expected {} > {}",
            expected,
            actual.unwrap_or(&Value::Null)
        ));
    };

    if actual > expected {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} > {}", expected, actual))
    }
}

fn greater_than_or_equal(actual: Option<&Value>, expected: f64) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_f64()) else {
        return ConditionResult::Failed(format!(
            "Expected {} >= {}",
            expected,
            actual.unwrap_or(&Value::Null)
        ));
    };

    if actual >= expected {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} >= {}", expected, actual))
    }
}

fn less_than(actual: Option<&Value>, expected: f64) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_f64()) else {
        return ConditionResult::Failed(format!(
            "Expected {} < {}",
            expected,
            actual.unwrap_or(&Value::Null)
        ));
    };

    if actual < expected {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} < {}", expected, actual))
    }
}

fn less_than_or_equal(actual: Option<&Value>, expected: f64) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_f64()) else {
        return ConditionResult::Failed(format!(
            "Expected {} <= {}",
            expected,
            actual.unwrap_or(&Value::Null)
        ));
    };

    if actual <= expected {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} <= {}", expected, actual))
    }
}

fn in_list(actual: Option<&Value>, expected: &Vec<Value>) -> ConditionResult {
    let actual = actual.unwrap_or(&Value::Null);

    if expected.contains(actual) {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} in {:?}", actual, expected))
    }
}

fn not_in_list(actual: Option<&Value>, expected: &Vec<Value>) -> ConditionResult {
    let actual = actual.unwrap_or(&Value::Null);

    if !expected.contains(actual) {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} not in {:?}", actual, expected))
    }
}

fn contains(actual: Option<&Value>, expected: &str) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return ConditionResult::Failed(format!(
            "Expected {} to be a string",
            actual.unwrap_or(&Value::Null)
        ));
    };

    if actual.contains(expected) {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} to contain {}", actual, expected))
    }
}

fn not_contains(actual: Option<&Value>, expected: &str) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return ConditionResult::Failed(format!(
            "Expected {} to be a string",
            actual.unwrap_or(&Value::Null)
        ));
    };

    if !actual.contains(expected) {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} not to contain {}", actual, expected))
    }
}

fn starts_with(actual: Option<&Value>, expected: &str) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return ConditionResult::Failed(format!(
            "Expected {} to be a string",
            actual.unwrap_or(&Value::Null)
        ));
    };

    if actual.starts_with(expected) {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} to start with {}", actual, expected))
    }
}

fn ends_with(actual: Option<&Value>, expected: &str) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return ConditionResult::Failed(format!(
            "Expected {} to be a string",
            actual.unwrap_or(&Value::Null)
        ));
    };

    if actual.ends_with(expected) {
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} to end with {}", actual, expected))
    }
}

fn matches(actual: Option<&Value>, expected: &str) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return ConditionResult::Failed(format!(
            "Expected {} to be a string",
            actual.unwrap_or(&Value::Null)
        ));
    };

    Regex::new(expected)
        .map(|re| re.is_match(actual))
        .map_or_else(
            |err| ConditionResult::Failed(format!("Invalid regex: {}", err)),
            |matches| {
                if matches {
                    ConditionResult::Passed
                } else {
                    ConditionResult::Failed(format!("Expected {} to match {}", actual, expected))
                }
            },
        )
}

fn not_matches(actual: Option<&Value>, expected: &str) -> ConditionResult {
    let Some(actual) = actual.and_then(|v| v.as_str()) else {
        return ConditionResult::Failed(format!(
            "Expected {} to be a string",
            actual.unwrap_or(&Value::Null)
        ));
    };

    Regex::new(expected)
        .map(|re| !re.is_match(actual))
        .map_or_else(
            |err| ConditionResult::Failed(format!("Invalid regex: {}", err)),
            |matches| {
                if matches {
                    ConditionResult::Passed
                } else {
                    ConditionResult::Failed(format!(
                        "Expected {} not to match {}",
                        actual, expected
                    ))
                }
            },
        )
}

fn is_matching(actual_opt: Option<&Value>, expected: MatchType) -> ConditionResult {
    let actual = actual_opt.unwrap_or(&Value::Null);

    let pass = match expected {
        MatchType::Undefined => actual_opt.is_none(),
        MatchType::Null => actual_opt.is_some() || actual.is_null(),
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
        ConditionResult::Passed
    } else {
        ConditionResult::Failed(format!("Expected {} to be of type {}", actual, expected))
    }
}

fn is_not_type(actual_opt: Option<&Value>, expected: MatchType) -> ConditionResult {
    let matching = is_matching(actual_opt, expected);

    match matching {
        ConditionResult::Passed => ConditionResult::Failed(format!(
            "Expected {} not to be of type {}",
            actual_opt.unwrap_or(&Value::Null),
            expected
        )),
        ConditionResult::Failed(_) => ConditionResult::Passed,
    }
}
