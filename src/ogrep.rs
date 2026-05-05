pub fn search<'a>(
    contents: &'a str,
    query: &str,
    ignore_case: bool,
    whole_word: bool,
) -> Vec<&'a str> {
    let query_lower = query.to_lowercase();

    contents
        .lines()
        .filter(|line| {
            if whole_word {
                // Handle Whole Word (with optional Case Insensitivity)
                line.split(|c: char| !c.is_alphanumeric()).any(|word| {
                    if ignore_case {
                        word.to_lowercase() == query_lower
                    } else {
                        word == query
                    }
                })
            } else if ignore_case {
                // Handle Case Insensitive Substring
                line.to_lowercase().contains(&query_lower)
            } else {
                // Handle Standard Substring
                line.contains(query)
            }
        })
        .collect()
}
