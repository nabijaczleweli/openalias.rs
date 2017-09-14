/// Convert an OpenAlias to an FQDN.
///
/// Paraphrasing [OpenAlias](https://openalias.org#implement):
///
/// 1. If the alias contains an `@` character, replace it with a `.` (period) character to allow for email-style addressing.
///
/// 2. Check that the alias contains a `.` (period) character, if not then it is an address and not an FQDN.
///
/// 3. Append, if one doesn't exist, a dot to the end of the alias, to ensure it's an FQDN.
///
/// # Examples
///
/// ```
/// # use openalias::parse_openalias;
/// assert_eq!(parse_openalias("donate.getmonero.org"),
///            Some("donate.getmonero.org.".to_string()));
/// assert_eq!(parse_openalias("donate@nabijaczleweli.xyz"),
///            Some("donate.nabijaczleweli.xyz.".to_string()));
/// assert_eq!(parse_openalias("nabijaczleweli.xyz."),
///            Some("nabijaczleweli.xyz.".to_string()));
///
/// assert_eq!(parse_openalias("nabijaczleweli"), None);
/// ```
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
