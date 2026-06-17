pub fn is_concrete_language(value: &str) -> bool {
    !value.trim().is_empty() && value != "auto"
}

pub fn is_valid_language_pair(value: &str) -> bool {
    value
        .split_once('|')
        .is_some_and(|(first, second)| is_concrete_language(first) && is_concrete_language(second))
}
