use std::env;
use std::io::Write;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use futures::stream::{self, StreamExt};
use reqwest::Client;
use serde_yaml::Value;
use tokio::fs;

use hackclub_dns_fetcher::config::*;
use hackclub_dns_fetcher::llm::extract_hackathons;
use hackclub_dns_fetcher::probe::probe;
use hackclub_dns_fetcher::types::{EntryJson, Hackathon, ProbeResult, SuccessJson};

// ── Main ─────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let verbose = env::args().any(|a| a == "-v");
    let api_key = env::var("NVIDIA_API_KEY").expect("NVIDIA_API_KEY env var not set");

    let yaml_url =
        "https://raw.githubusercontent.com/hackclub/dns/refs/heads/main/hackclub.com.yaml";

    if verbose {
        println!("Fetching YAML from: {}", yaml_url);
    }

    let client = Arc::new(
        Client::builder()
            .timeout(std::time::Duration::from_secs(REQUEST_TIMEOUT_SECS))
            .build()?,
    );

    // ── Fetch & parse DNS YAML ───────────────────────────────────────────────
    let content = client.get(yaml_url).send().await?.text().await?;
    let parsed: Value = serde_yaml::from_str(&content)?;
    let map = parsed
        .as_mapping()
        .ok_or("Expected a YAML mapping at root")?;

    let subdomains: Vec<String> = map
        .iter()
        .filter_map(|(k, _)| k.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| format!("http://{}.hackclub.com", s))
        .collect();

    let total = subdomains.len();
    let done = Arc::new(AtomicUsize::new(0));

    if verbose {
        println!(
            "Probing {} subdomains (concurrency {})...\n",
            total, CONCURRENCY
        );
    } else {
        print!("Probing subdomains  0/{}", total);
        let _ = std::io::Write::flush(&mut std::io::stdout());
    }

    // ── Probe all subdomains concurrently ────────────────────────────────────
    let probes: Vec<ProbeResult> = stream::iter(subdomains)
        .map(|url| {
            let client = Arc::clone(&client);
            let done = Arc::clone(&done);
            async move {
                let result = probe(&client, &url).await;
                let n = done.fetch_add(1, Ordering::Relaxed) + 1;

                if verbose {
                    match (&result.status, &result.content, &result.error) {
                        (Some(s), Some(c), _) => {
                            println!("[{}/{}] {} → {} {}b", n, total, url, s, c.len())
                        }
                        (_, _, Some(e)) => println!("[{}/{}] {} → ✗ {}", n, total, url, e),
                        _ => println!("[{}/{}] {} → ✗ unknown", n, total, url),
                    }
                } else {
                    print!("\rProbing subdomains  {}/{}", n, total);
                    let _ = std::io::stdout().flush();
                }

                result
            }
        })
        .buffer_unordered(CONCURRENCY)
        .collect()
        .await;

    println!();

    // ── Write debug JSONs ────────────────────────────────────────────────────
    {
        let results_json: Vec<EntryJson> = probes
            .iter()
            .map(|p| EntryJson {
                subdomain: p.subdomain.clone(),
                status: p.status,
                bytes: p.content.as_ref().map(|c| c.len()),
                error: p.error.clone(),
            })
            .collect();

        let successes_json: Vec<SuccessJson> = probes
            .iter()
            .filter_map(|p| match (p.status, p.content.as_ref()) {
                (Some(s), Some(c)) if s < 400 => Some(SuccessJson {
                    url: p.subdomain.clone(),
                    content: c.clone(),
                }),
                _ => None,
            })
            .collect();

        fs::write("results.json", serde_json::to_string_pretty(&results_json)?).await?;
        fs::write(
            "successes.json",
            serde_json::to_string_pretty(&successes_json)?,
        )
        .await?;

        if verbose {
            println!(
                "Debug: results.json ({} entries), successes.json ({} successes)",
                results_json.len(),
                successes_json.len()
            );
        }
    }

    // ── Ask the LLM about each success ───────────────────────────────────────
    let successes: Vec<(String, String)> = probes
        .into_iter()
        .filter_map(|p| match (p.status, p.content) {
            (Some(s), Some(c)) if s < 400 => Some((p.subdomain, c)),
            _ => None,
        })
        .collect();

    let success_count = successes.len();
    let llm_done = Arc::new(AtomicUsize::new(0));

    if verbose {
        println!("\nQuerying LLM for {} successful pages...\n", success_count);
    } else {
        print!("Querying LLM        0/{}", success_count);
        let _ = std::io::stdout().flush();
    }

    let api_key = Arc::new(api_key);

    let hackathons: Vec<Hackathon> = stream::iter(successes)
        .map(|(url, html)| {
            let client = Arc::clone(&client);
            let api_key = Arc::clone(&api_key);
            let llm_done = Arc::clone(&llm_done);
            async move {
                let result = extract_hackathons(&client, &api_key, &url, &html).await;
                let n = llm_done.fetch_add(1, Ordering::Relaxed) + 1;

                if verbose {
                    match &result {
                        Ok(h) => println!(
                            "[{}/{}] {} → {} hackathon(s) found",
                            n,
                            success_count,
                            url,
                            h.len()
                        ),
                        Err(e) => {
                            println!("[{}/{}] {} → ✗ LLM error: {}", n, success_count, url, e)
                        }
                    }
                } else {
                    print!("\rQuerying LLM        {}/{}", n, success_count);
                    let _ = std::io::stdout().flush();
                }

                result.unwrap_or_default()
            }
        })
        .buffer_unordered(CONCURRENCY)
        .collect::<Vec<Vec<Hackathon>>>()
        .await
        .into_iter()
        .flatten()
        .collect();

    println!();

    // ── Write & print summary ────────────────────────────────────────────────
    fs::write("summary.json", serde_json::to_string_pretty(&hackathons)?).await?;

    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║                    HACKATHON SUMMARY                        ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    if hackathons.is_empty() {
        println!("No hackathons found.");
    } else {
        for h in &hackathons {
            println!("▸ {}", h.name);
            println!("  Dates:   {}", h.dates);
            println!("  URL:     {}", h.url);
            println!("  Summary: {}", h.summary);
            println!();
        }
    }

    println!(
        "Found {} hackathon(s) total. Full details in summary.json.",
        hackathons.len()
    );

    Ok(())
}
