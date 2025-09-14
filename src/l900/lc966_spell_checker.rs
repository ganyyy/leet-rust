pub fn spellchecker(wordlist: Vec<String>, queries: Vec<String>) -> Vec<String> {
    use std::collections::{HashMap, HashSet};

    let mut exact = HashSet::new();
    let mut case_insensitive = HashMap::new();
    let mut vowel_error = HashMap::new();

    for word in &wordlist {
        exact.insert(word.clone());
        let lower = word.to_lowercase();
        case_insensitive
            .entry(lower.clone())
            .or_insert(word.clone());
        let devoweled = lower
            .chars()
            .map(|c| if "aeiou".contains(c) { '*' } else { c })
            .collect::<String>();
        vowel_error.entry(devoweled).or_insert(word.clone());
    }

    let mut result = Vec::with_capacity(queries.len());
    for query in &queries {
        if exact.contains(query) {
            result.push(query.clone());
            continue;
        }
        let lower = query.to_lowercase();
        if let Some(word) = case_insensitive.get(&lower) {
            result.push(word.clone());
            continue;
        }
        let devoweled = lower
            .chars()
            .map(|c| if "aeiou".contains(c) { '*' } else { c })
            .collect::<String>();
        if let Some(word) = vowel_error.get(&devoweled) {
            result.push(word.clone());
            continue;
        }
        result.push("".to_string());
    }

    result
}
