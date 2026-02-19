//! LLM-based hackathon extraction from HTML content.

use reqwest::Client;
use serde_json::{json, Value as JsonValue};

use crate::config::{HTML_TRUNCATE_CHARS, LLM_MAX_TOKENS, LLM_TEMPERATURE, NIM_API_URL, NIM_MODEL};
use crate::types::Hackathon;

/// Extract hackathons from HTML content using the NVIDIA NIM LLM.
///
/// # Arguments
/// * `client` - HTTP client for making LLM API requests
/// * `api_key` - NVIDIA API key for authentication
/// * `url` - The source URL (used as context and fallback)
/// * `html` - HTML content to analyze
///
/// # Returns
/// A vector of extracted hackathons, or an error if the request fails
pub async fn extract_hackathons(
    client: &Client,
    api_key: &str,
    url: &str,
    html: &str,
) -> Result<Vec<Hackathon>, Box<dyn std::error::Error + Send + Sync>> {
    // Truncate HTML to avoid blowing the context window
    let truncated: String = html.chars().take(HTML_TRUNCATE_CHARS).collect();

    let prompt = format!(
        r#"You are a hackathon finder. Given HTML from the page "{url}", extract any hackathons mentioned.

For each hackathon found, respond with a JSON array. Each object must have exactly these fields:
- "name": hackathon name
- "url": most specific URL for the hackathon (use "{url}" if no better link found)
- "dates": date or date range as a string (e.g. "March 15–17, 2025"), or "Unknown" if not found
- "summary": one sentence describing the hackathon

If there are no hackathons on this page, respond with an empty array: []
Respond with ONLY the JSON array, no other text.

HTML:
{truncated}"#
    );

    let body = json!({
        "model": NIM_MODEL,
        "messages": [{ "role": "user", "content": prompt }],
        "temperature": LLM_TEMPERATURE,
        "max_tokens": LLM_MAX_TOKENS,
    });

    let resp = client
        .post(NIM_API_URL)
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let json: JsonValue = resp.json().await?;
    let text = json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("[]");

    // Strip markdown fences if the model wrapped it anyway
    let clean = text
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let hackathons: Vec<Hackathon> = serde_json::from_str(clean).unwrap_or_default();
    Ok(hackathons)
}
