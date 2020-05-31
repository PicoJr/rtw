pub mod activity;
pub mod clock;
pub mod datetimew;
pub mod durationw;
pub mod repository;
pub mod service;

/// Absolute dates are parsed and displayed using this format
///
/// e.g. 2019-12-25T18:43:00
pub const DATETIME_FMT: &str = "%Y-%m-%dT%H:%M:%S";

/// `Tag` = `String`
pub type Tag = String;
/// `Tags` = `Vec<Tag>`
pub type Tags = Vec<Tag>;
/// `ActivityId` = `usize`
pub type ActivityId = usize;
