#[cfg(not(test))]
use time::OffsetDateTime;

/// Get the current time as unix timestamp
/// @return The current time as a unix timestamp
#[cfg(not(test))]
pub(crate) fn now_timestamp() -> usize {
    OffsetDateTime::now_utc().unix_timestamp() as usize
}

#[cfg(test)]
thread_local! {
    static MOCK_TIMESTAMP: std::cell::Cell<usize> = const {std::cell::Cell::new(0)};
}
#[cfg(test)]
pub(crate) fn set_mock_time(timestamp: usize) {
    MOCK_TIMESTAMP.with(|mock_timestamp| {
        mock_timestamp.set(timestamp);
    });
}


/// Get the current time as unix timestamp
/// For testing purposes, this function can be mocked
/// @return The current time as a unix timestamp
#[cfg(test)]
pub(crate) fn now_timestamp() -> usize {
    MOCK_TIMESTAMP.with(|mock_timestamp| {
        mock_timestamp.get()
    })
}


