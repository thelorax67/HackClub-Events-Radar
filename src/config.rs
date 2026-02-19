//! Configuration constants for the HackClub Events Radar.

/// Concurrency level for parallel HTTP requests (DNS probing).
pub const HTTP_CONCURRENCY: usize = 20;

/// Concurrency level for LLM queries (limited by rate limit).
/// Set to 1 to serialize LLM requests and respect rate limits.
pub const LLM_CONCURRENCY: usize = 4;

/// NVIDIA NIM API rate limit: requests per minute.
/// Ensure concurrency * ~(60 / requests_per_minute) >= 1
pub const LLM_RATE_LIMIT_PER_MINUTE: u32 = 40;

/// NVIDIA NIM API endpoint for chat completions.
pub const NIM_API_URL: &str = "https://integrate.api.nvidia.com/v1/chat/completions";

/// LLM model identifier (GPT OSS via NVIDIA NIM).
pub const NIM_MODEL: &str = "openai/gpt-oss-120b";

/// HTTP request timeout duration in seconds.
pub const REQUEST_TIMEOUT_SECS: u64 = 15;

/// Maximum characters from HTML to send to the LLM (to avoid context window limits).
pub const HTML_TRUNCATE_CHARS: usize = 12_000;

/// Maximum tokens to request from the LLM.
pub const LLM_MAX_TOKENS: u32 = 1024;

/// Temperature parameter for LLM sampling (lower = more deterministic).
pub const LLM_TEMPERATURE: f32 = 0.1;
