use std::collections::HashMap;

pub fn option_into<T, U: From<T>>(option: Option<T>) -> Option<U> {
    option.map(|value| value.into())
}

pub fn option_fold_empty_object(
    value: Vec<String>,
) -> Option<HashMap<std::string::String, HashMap<(), ()>>> {
    Some(value.into_iter().fold(HashMap::new(), |mut acc, item| {
        acc.insert(item, HashMap::default());

        acc
    }))
}
