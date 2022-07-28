pub(crate) fn get_title_pattern(title_depth: u64) -> String {
    let base_pattern: &str = r"^\d\.";
    let pattern_extension: &str = r"\d\.";
    let mut title_pattern: String = String::from(base_pattern);
    for _ in 1..title_depth + 1 {
        title_pattern.push_str(pattern_extension);
    }
    title_pattern.push_str(" ");
    title_pattern
}
