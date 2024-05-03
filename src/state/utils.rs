use core::http::KeyValue;

use components::KeyValList;

pub fn from_core_kv_list(values: Vec<KeyValue>, fixed: bool) -> KeyValList {
    let values = values
        .into_iter()
        .map(|kv| components::KeyValue::new(&kv.name, &kv.value, kv.disabled))
        .collect();
    KeyValList::from(values, fixed)
}

pub fn to_core_kv_list(list: &KeyValList) -> Vec<KeyValue> {
    list.values()
        .iter()
        .map(|kv| KeyValue {
            disabled: kv.disabled,
            name: kv.name().trim().to_owned(),
            value: kv.value().trim().to_owned(),
        })
        .collect()
}