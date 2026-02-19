//! HTTP probing functionality for fetching and analyzing DNS subdomains.

use crate::types::ProbeResult;
use reqwest::Client;

/// Probe a single URL and return the result.
///
/// # Arguments
/// * `client` - HTTP client to use for the request
/// * `url` - URL to probe
///
/// # Returns
/// A `ProbeResult` containing status code, content, and/or error information
pub async fn probe(client: &Client, url: &str) -> ProbeResult {
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            match resp.text().await {
                Ok(body) => ProbeResult {
                    subdomain: url.to_string(),
                    status: Some(status),
                    content: Some(body),
                    error: None,
                },
                Err(e) => ProbeResult {
                    subdomain: url.to_string(),
                    status: Some(status),
                    content: None,
                    error: Some(e.to_string()),
                },
            }
        }
        Err(e) => ProbeResult {
            subdomain: url.to_string(),
            status: None,
            content: None,
            error: Some(e.to_string()),
        },
    }
}
