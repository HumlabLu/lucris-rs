use serde::Serialize;

// Errors thrown when doing a try_from_with_locale().
#[derive(Debug, Serialize)]
pub enum CleanError {
    MissingUUID,
    MissingNameField,
    MissingFirstName,
    MissingLastName,
}

// Errors thrown when working with Combined.
#[derive(Debug, Serialize)]
pub enum CombinedError {
    NoSuchUUID,
}
