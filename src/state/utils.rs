use lib::http::{self, KeyFile, KeyValue};

use crate::components::{self, KeyFileList, KeyValList};

pub fn from_core_kv_list(values: &http::KeyValList, fixed: bool) -> KeyValList {
    let values = values
        .iter()
        .map(|kv| components::KeyValue::new(&kv.name, &kv.value, kv.disabled))
        .collect();
    KeyValList::from(values, fixed)
}

pub fn to_core_kv_list(list: &KeyValList) -> http::KeyValList {
    let vals = list
        .values()
        .iter()
        .map(|kv| KeyValue {
            disabled: kv.disabled,
            name: kv.name().trim().to_owned(),
            value: kv.value().trim().to_owned(),
        })
        .filter(|kv| !kv.name.is_empty() || !kv.value.is_empty())
        .collect();
    http::KeyValList::from(vals)
}

pub fn from_core_kf_list(values: http::KeyFileList) -> KeyFileList {
    let values = values
        .into_iter()
        .map(|kv| components::KeyFile::new(&kv.name, kv.path, kv.disabled))
        .collect();
    KeyFileList::from(values, false)
}

pub fn to_core_kf_list(list: &KeyFileList) -> http::KeyFileList {
    let vals = list
        .values()
        .iter()
        .map(|kv| KeyFile {
            disabled: kv.disabled,
            name: kv.name().trim().to_owned(),
            path: kv.path.to_owned(),
        })
        .filter(|kv| !kv.name.is_empty() || kv.path.is_some())
        .collect();

    http::KeyFileList::from(vals)
}

pub fn headers_to_string(headers: &reqwest::header::HeaderMap) -> String {
    headers
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v.to_str().unwrap_or_default()))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn key_value_to_text(list: &KeyValList) -> String {
    list.values()
        .iter()
        .filter(|kv| !kv.name().is_empty() || !kv.value().is_empty())
        .map(|kv| format!("{}: {}", kv.name(), kv.value()))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn key_value_from_text(text: &str) -> KeyValList {
    let lines = text.lines();
    let mut key_val_list = Vec::new();
    for line in lines {
        let (key, value) = line.split_once(':').unwrap_or_default();
        key_val_list.push(components::KeyValue::new(key.trim(), value.trim(), false));
    }
    KeyValList::from(key_val_list, false)
}
