# HackClub Events Radar

A high-performance Rust tool that discovers hackathons by probing HackClub's DNS subdomains and analyzing HTML content using an AI language model.

## Features

- **Concurrent Subdomain Probing**: Efficiently probes 20+ subdomains in parallel via HTTP
- **LLM-Powered Extraction**: Uses NVIDIA NIM's GLM 4.7 model to intelligently extract hackathon information from HTML
- **Robust Error Handling**: Gracefully handles network timeouts, parsing errors, and API failures
- **Progress Tracking**: Real-time console feedback on probing and LLM query progress
- **JSON Output**: Generates structured results for further processing:
  - `results.json`: All probe attempts with status codes
  - `successes.json`: Successfully retrieved HTML content
  - `summary.json`: Extracted hackathon data (names, dates, URLs, descriptions)

## Prerequisites

- **Rust 1.70+** (for features used in dependencies)
- **NVIDIA API Key**: Required for LLM queries. Get one at [NVIDIA's platform](https://www.nvidia.com/)
- **.env file**: Store your API key privately

## Installation

```bash
# Clone the repository
git clone https://github.com/thelorax67/HackClub-Events-Radar.git
cd HackClub-Events-Radar

# Create .env file with your API key
echo "NVIDIA_API_KEY=your_api_key_here" > .env

# Build the project
cargo build --release
```

## Usage

### Quick Start

```bash
# Run the scanner
cargo run --release

# Run with verbose output
cargo run --release -- -v
```

### Output Files

After running, three JSON files are created:

- **results.json**: Detailed probe results for all subdomains
- **successes.json**: Successfully retrieved HTML content (for debugging)
- **summary.json**: Final hackathon list with names, dates, and URLs

### Example Output

```
╔══════════════════════════════════════════════════════════════╗
║                    HACKATHON SUMMARY                        ║
╚══════════════════════════════════════════════════════════════╝

▸ HackMIT
  Dates:   September 20–21, 2025
  URL:     https://hackmit.org
  Summary: Harvard's flagship hackathon bringing together 1000+ hackers.

▸ Hack the North
  Dates:   September 12–14, 2025
  URL:     https://hackthenorth.com
  Summary: Canada's largest hackathon hosted at the University of Waterloo.

Found 2 hackathon(s) total. Full details in summary.json.
```

## Project Structure

```
HackClub-Events-Radar/
├── src/
│   ├── main.rs         # CLI entry point and orchestration
│   ├── lib.rs          # Library root with public API
│   ├── config.rs       # Configuration constants
│   ├── types.rs        # Data structure definitions
│   ├── probe.rs        # HTTP probing functionality
│   └── llm.rs          # LLM-based extraction logic
├── Cargo.toml          # Project manifest
├── .env.example        # Environment variable template
├── .gitignore          # Git ignore rules
├── README.md           # This file
└── LICENSE             # MIT License
```

## Configuration

Edit constants in [src/config.rs](src/config.rs):

| Constant | Default | Purpose |
|----------|---------|---------|
| `CONCURRENCY` | 20 | Parallel requests for probing and LLM queries |
| `REQUEST_TIMEOUT_SECS` | 15 | HTTP request timeout |
| `HTML_TRUNCATE_CHARS` | 12,000 | Max HTML characters to send to LLM |
| `LLM_MAX_TOKENS` | 1024 | Maximum tokens in LLM response |
| `LLM_TEMPERATURE` | 0.1 | LLM sampling temperature (lower = more deterministic) |

## Development

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests (if available)
cargo test
```

### Code Organization

- **config.rs**: Constants and configuration
- **types.rs**: Serializable data structures with documentation
- **probe.rs**: HTTP client functionality
- **llm.rs**: NVIDIA NIM API integration
- **main.rs**: CLI and orchestration logic

## API Details

### Probing Phase

1. Fetches HackClub's DNS records from their GitHub repository
2. Constructs full URLs (http://{subdomain}.hackclub.com)
3. Concurrently probes each URL with a 15-second timeout
4. Collects status codes and HTML content

### LLM Extraction Phase

1. Sends HTML content to NVIDIA's GLM 4.7 model
2. Provides structured JSON format requirements
3. Parses responses and validates data
4. Handles malformed responses gracefully

## Error Handling

The tool gracefully handles:

- Network timeouts (individual failures don't stop the process)
- Invalid HTML or YAML parsing
- LLM API failures and malformed responses
- Missing environment variables (with clear error messages)

## Performance

- **Probing**: ~50-100 subdomains per second (with 20 concurrent requests)
- **LLM Queries**: ~1 query per second (rate-limited by API)
- **Total Runtime**: Typically 2–5 minutes for complete scan

## Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure code compiles: `cargo check`
5. Format code: `cargo fmt`
6. Submit a pull request

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.

## Acknowledgments

- [HackClub](https://hackclub.com) for the DNS data
- [NVIDIA NIM](https://cloud.nvidia.com/nim) for LLM access
- [Tokio](https://tokio.rs) for async runtime
- [Reqwest](https://github.com/seanmonstar/reqwest) for HTTP client