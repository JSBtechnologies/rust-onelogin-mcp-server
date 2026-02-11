use governor::{
    clock::DefaultClock,
    state::{InMemoryState, NotKeyed},
    Quota, RateLimiter as GovernorRateLimiter,
};
use std::num::NonZeroU32;

pub struct RateLimiter {
    limiter: GovernorRateLimiter<NotKeyed, InMemoryState, DefaultClock>,
}

impl RateLimiter {
    pub fn new(requests_per_second: u32) -> Self {
        let quota = Quota::per_second(
            NonZeroU32::new(requests_per_second).expect("Rate limit must be > 0"),
        );
        let limiter = GovernorRateLimiter::direct(quota);

        Self { limiter }
    }

    pub async fn wait(&self) {
        while self.limiter.check().is_err() {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rate_limiter() {
        let limiter = RateLimiter::new(10);
        // Should not block
        limiter.wait().await;
    }
}
