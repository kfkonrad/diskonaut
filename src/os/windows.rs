#[cfg(not(test))]
pub fn is_user_admin() -> bool {
    is_elevated::is_elevated()
}
#[cfg(test)]
pub fn is_user_admin() -> bool {
    false
}
