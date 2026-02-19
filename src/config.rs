//! Configuration constants for the HackClub Events Radar.

/// Concurrency level for parallel HTTP requests and LLM queries.
pub const CONCURRENCY: usize = 20;

/// NVIDIA NIM API endpoint for chat completions.
pub const NIM_API_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

/// LLM model identifier (GLM 4.7 via NVIDIA NIM).
pub const NIM_MODEL: &str = "z-ai/glm4.7";

/// HTTP request timeout duration in seconds.
pub const REQUEST_TIMEOUT_SECS: u64 = 15;

/// Maximum characters from HTML to send to the LLM (to avoid context window limits).
pub const HTML_TRUNCATE_CHARS: usize = 12_000;

/// Maximum tokens to request from the LLM.
pub const LLM_MAX_TOKENS: u32 = 1024;

/// Temperature parameter for LLM sampling (lower = more deterministic).
pub const LLM_TEMPERATURE: f32 = 0.1;
