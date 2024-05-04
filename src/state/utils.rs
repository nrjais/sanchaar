use core::http::{self, KeyValue};

use components::KeyValList;

pub fn from_core_kv_list(values: http::KeyValList, fixed: bool) -> KeyValList {
    let values = values
        .into_iter()
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
