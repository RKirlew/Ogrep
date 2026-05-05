pub fn search<'a>(contents: &'a str, query: &str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
