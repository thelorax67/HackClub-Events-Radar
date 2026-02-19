//! HackClub Events Radar - A tool for discovering hackathons through DNS subdomains.
//!
//! This library provides functionality to:
//! 1. Fetch and parse DNS records from HackClub's DNS repository
//! 2. Probe subdomains for active web endpoints
//! 3. Extract hackathon information from HTML using LLM analysis

pub mod config;
pub mod llm;
pub mod probe;
pub mod types;

pub use types::{EntryJson, Hackathon, ProbeResult, SuccessJson};
