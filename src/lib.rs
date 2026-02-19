//! HackClub Events Radar - A tool for discovering hackathons through DNS subdomains.
//!
//! This library provides functionality to:
//! 1. Fetch and parse DNS records from HackClub's DNS repository
//! 2. Probe subdomains for active web endpoints
//! 3. Extract hackathon information from HTML using LLM analysis
//! 4. Rate limit API requests to respect service limits
//! 5. Track git history to determine when subdomains were added/modified

pub mod config;
pub mod git_history;
pub mod llm;
pub mod probe;
pub mod ratelimit;
pub mod types;

pub use git_history::GitInfo;
pub use ratelimit::RateLimiter;
pub use types::{EntryJson, Hackathon, ProbeResult, SuccessJson};
