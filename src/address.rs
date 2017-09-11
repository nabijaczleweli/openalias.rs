pub fn parse_openalias(alias: &str) -> Option<String> {
    let mut alias = alias.replace("@", ".");
    if alias.contains(".") {
        if !alias.ends_with(".") {
            alias.push('.');
        }
        Some(alias)
    } else {
        None
    }
}
