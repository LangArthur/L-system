pub fn count_chars(string: &str, needle: char) -> usize {
    string
        .chars()
        .filter(|c| c == &needle)
        .count()
}