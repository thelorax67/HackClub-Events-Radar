//! Data structures for HackClub Events Radar.

use serde::{Deserialize, Serialize};

/// Represents the result of probing a single URL.
#[derive(Debug, Clone)]
pub struct ProbeResult {
    /// The full URL that was probed.
    pub subdomain: String,
    /// HTTP status code, if the request succeeded.
    pub status: Option<u16>,
    /// Response body content, if available.
    pub content: Option<String>,
    /// Error message, if the request or parsing failed.
    pub error: Option<String>,
}

/// JSON representation of a probe result for debugging.
#[derive(Serialize)]
pub struct EntryJson {
    pub subdomain: String,
    pub status: Option<u16>,
    pub bytes: Option<usize>,
    pub error: Option<String>,
}

/// JSON representation of a successful probe result (status < 400).
#[derive(Serialize)]
pub struct SuccessJson {
    pub url: String,
    pub content: String,
}

/// Represents a hackathon event extracted from HTML content.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Hackathon {
    /// Name of the hackathon.
    pub name: String,
    /// URL for the hackathon (or the page it was found on).
    pub url: String,
    /// Date or date range (e.g., "March 15–17, 2025").
    pub dates: String,
    /// One-sentence description of the hackathon.
    pub summary: String,
}
