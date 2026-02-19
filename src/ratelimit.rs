//! Rate limiting functionality for API requests.

use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::time::interval;

/// A token bucket rate limiter that respects request rate limits.
pub struct RateLimiter {
    /// Semaphore that represents available tokens (request slots).
    semaphore: Arc<Semaphore>,
    /// Maximum number of requests per minute.
    requests_per_minute: u32,
}

impl RateLimiter {
    /// Create a new rate limiter with the given request rate limit.
    ///
    /// # Arguments
    /// * `requests_per_minute` - Maximum number of requests allowed per minute
    ///
    /// # Example
    /// ```ignore
    /// let limiter = RateLimiter::new(40); // Allow 40 requests per minute
    /// let permit = limiter.acquire().await;
    /// // Make request...
    /// drop(permit); // Release permit
    /// ```
    pub fn new(requests_per_minute: u32) -> Self {
        let limiter = RateLimiter {
            semaphore: Arc::new(Semaphore::new(1)),
            requests_per_minute,
        };

        // Start background task to refill tokens
        let semaphore = Arc::clone(&limiter.semaphore);
        let rpm = requests_per_minute;

        tokio::spawn(async move {
            let tokens_per_second = rpm as f64 / 60.0;
            let mut interval = interval(Duration::from_millis((1000.0 / tokens_per_second) as u64));

            loop {
                interval.tick().await;
                // Add a permit if semaphore is below max
                if semaphore.available_permits() < rpm as usize {
                    semaphore.add_permits(1);
                }
            }
        });

        limiter
    }

    /// Acquire a permit to make a request.
    /// This will block until a permit is available.
    pub async fn acquire(&self) -> tokio::sync::SemaphorePermit<'_> {
        self.semaphore.acquire().await.unwrap()
    }
}

impl Clone for RateLimiter {
    fn clone(&self) -> Self {
        RateLimiter {
            semaphore: Arc::clone(&self.semaphore),
            requests_per_minute: self.requests_per_minute,
        }
    }
}
