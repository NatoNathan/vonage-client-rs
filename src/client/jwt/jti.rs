#[cfg(not(test))]
use nanoid::nanoid;

/// Generate a unique JTI for the JWT
/// @return A unique JTI
#[cfg(not(test))]
pub(crate) fn generate_jti() -> String {
    nanoid!()
}

#[cfg(test)]
thread_local! {
    static MOCK_JTI: std::cell::RefCell<String> = std::cell::RefCell::new("".to_string());
}

#[cfg(test)]
pub(crate) fn set_mock_jti(jti: String) {
    MOCK_JTI.with(|mock_jti| {
        *mock_jti.borrow_mut() = jti;
    });
}

/// Generate a unique JTI for the JWT
/// For testing purposes, this function can be mocked
/// @return A unique JTI
#[cfg(test)]
pub(crate) fn generate_jti() -> String {
    MOCK_JTI.with(|mock_jti| mock_jti.borrow().clone())
}
