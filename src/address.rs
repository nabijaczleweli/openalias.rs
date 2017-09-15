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
/// # use openalias::alias_to_fqdn;
/// assert_eq!(alias_to_fqdn("donate.getmonero.org"),
///            Some("donate.getmonero.org.".to_string()));
/// assert_eq!(alias_to_fqdn("donate@nabijaczleweli.xyz"),
///            Some("donate.nabijaczleweli.xyz.".to_string()));
/// assert_eq!(alias_to_fqdn("nabijaczleweli.xyz."),
///            Some("nabijaczleweli.xyz.".to_string()));
///
/// assert_eq!(alias_to_fqdn("nabijaczleweli"), None);
/// ```
pub fn alias_to_fqdn(alias: &str) -> Option<String> {
    let mut alias = alias.replace("@", ".");
    if alias.contains('.') {
        if !alias.ends_with('.') {
            alias.push('.');
        }
        Some(alias)
    } else {
        None
    }
}
