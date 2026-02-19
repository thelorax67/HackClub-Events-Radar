//! Git history tracking for DNS YAML entries.

use std::collections::HashMap;
use std::process::Command;

/// Represents git history information for a subdomain.
#[derive(Debug, Clone)]
pub struct GitInfo {
    /// ISO 8601 timestamp of first commit adding this subdomain.
    pub first_added: Option<String>,
    /// ISO 8601 timestamp of last commit modifying this subdomain.
    pub last_modified: Option<String>,
}

/// Fetches git history for a YAML file and builds a map of subdomain -> git info.
///
/// This uses `git log` to get the history of the YAML file and determines when each
/// subdomain was first added and last modified.
///
/// # Arguments
/// * `yaml_path` - Local path to the YAML file
/// * `repo_path` - Path to the local git repository
///
/// # Returns
/// HashMap mapping subdomain names to GitInfo (first_added, last_modified)
pub fn get_yaml_git_history(
    yaml_path: &str,
    repo_path: &str,
) -> Result<HashMap<String, GitInfo>, Box<dyn std::error::Error>> {
    let mut history_map: HashMap<String, GitInfo> = HashMap::new();

    // Get the full diff history (the timestamps are extracted from the diff content below)
    let diff_output = Command::new("git")
        .args(&["-C", repo_path, "log", "-p", "--", yaml_path])
        .output()?;

    if !diff_output.status.success() {
        return Err("Failed to get git diff".into());
    }

    let diff_content = String::from_utf8(diff_output.stdout)?;

    // Parse the diff to track each subdomain's history
    parse_git_diff(&diff_content, &mut history_map)?;

    Ok(history_map)
}

/// Parses git diff output to track when subdomains were added/modified.
fn parse_git_diff(
    diff_content: &str,
    history_map: &mut HashMap<String, GitInfo>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut current_timestamp: Option<String> = None;
    let mut current_commit_subdomains: std::collections::HashSet<String> = std::collections::HashSet::new();

    for line in diff_content.lines() {
        // Extract timestamp from commit line
        if line.starts_with("Date:") {
            // First, apply the previous commit's changes
            for subdomain in current_commit_subdomains.drain() {
                if let Some(ts) = &current_timestamp {
                    let entry = history_map
                        .entry(subdomain)
                        .or_insert(GitInfo {
                            first_added: None,
                            last_modified: None,
                        });

                    // First encounter is first_added
                    if entry.first_added.is_none() {
                        entry.first_added = Some(ts.clone());
                    }
                    // Every encounter updates last_modified
                    entry.last_modified = Some(ts.clone());
                }
            }

            // Parse date line like "Date:   Tue Feb 19 10:30:00 2024 +0000"
            if let Some(date_str) = line.strip_prefix("Date:") {
                current_timestamp = format_date_to_iso8601(date_str.trim()).ok();
            }
        }

        // Track additions of YAML keys (subdomains) - these indicate a change to that subdomain
        if line.starts_with('+') && !line.starts_with("+++") {
            if let Some(subdomain) = parse_subdomain_from_yaml_line(&line[1..]) {
                current_commit_subdomains.insert(subdomain);
            }
        }

        // Track deletions - also indicate a change to that subdomain
        if line.starts_with('-') && !line.starts_with("---") {
            if let Some(subdomain) = parse_subdomain_from_yaml_line(&line[1..]) {
                current_commit_subdomains.insert(subdomain);
            }
        }
    }

    // Process final commit's changes
    for subdomain in current_commit_subdomains {
        if let Some(ts) = &current_timestamp {
            let entry = history_map
                .entry(subdomain)
                .or_insert(GitInfo {
                    first_added: None,
                    last_modified: None,
                });

            if entry.first_added.is_none() {
                entry.first_added = Some(ts.clone());
            }
            entry.last_modified = Some(ts.clone());
        }
    }

    Ok(())
}

/// Extracts a subdomain key from a YAML line.
/// Returns the subdomain name (without the colon) if this is a top-level YAML key.
fn parse_subdomain_from_yaml_line(line: &str) -> Option<String> {
    // YAML format is "subdomain: <content>"
    // Top-level keys have no leading whitespace
    if line.starts_with(' ') || line.is_empty() {
        return None;
    }

    if let Some(colon_pos) = line.find(':') {
        let key = line[..colon_pos].trim();
        if !key.is_empty() && key.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Some(key.to_string());
        }
    }

    None
}

/// Converts git date format to ISO 8601.
fn format_date_to_iso8601(date_str: &str) -> Result<String, String> {
    // Try to parse common git date formats and convert to ISO 8601
    // For simplicity, if it already looks like ISO 8601, return as-is
    if date_str.len() >= 10 && date_str.chars().nth(4) == Some('-') {
        return Ok(date_str.to_string());
    }

    // Try parsing RFC 2822 format (standard git output)
    use chrono::prelude::*;
    
    // Handle Git's date format: "Tue Feb 19 10:30:00 2024 +0000"
    // or "Fri, 19 Feb 2024 10:30:00 +0000"
    let cleaned = date_str
        .replace("  ", " ")
        .trim()
        .to_string();

    // Try RFC 2822 first
    if let Ok(dt) = DateTime::parse_from_rfc2822(&cleaned) {
        return Ok(dt.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
    }

    // Try to parse manually for Git's default format
    // Format: "Fri Feb 19 10:30:00 2024 +0000"
    let parts: Vec<&str> = cleaned.split_whitespace().collect();
    if parts.len() >= 5 {
        // Try constructing a RFC 2822 compatible string
        // Git format: DoW Mon DD HH:MM:SS YYYY +ZZZZ
        // RFC 2822: DoW, DD Mon YYYY HH:MM:SS +ZZZZ
        if let Some(rfc2822_str) = format!("{}, {} {} {} {} {}",
            parts[0],  // DoW
            parts[2],  // DD
            parts[1],  // Mon
            parts[4],  // YYYY
            parts[3],  // HH:MM:SS
            parts.get(5).map(|s| *s).unwrap_or("+0000")  // Timezone
        ).as_str() 
            .split_once(',')
            .and_then(|(dow, rest)| {
                let rfc_format = format!("{},{}", dow, rest);
                DateTime::parse_from_rfc2822(&rfc_format).ok()
            }) {
            return Ok(rfc2822_str.to_rfc3339_opts(chrono::SecondsFormat::Secs, true));
        }
    }

    Err(format!("Could not parse date: {}", date_str))
}
