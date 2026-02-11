use moka::future::Cache as MokaCache;
use serde::{de::DeserializeOwned, Serialize};
use std::time::Duration;

#[allow(dead_code)]
pub struct CacheManager {
    cache: MokaCache<String, Vec<u8>>,
}

#[allow(dead_code)]
impl CacheManager {
    pub fn new(ttl_seconds: u64, max_capacity: u64) -> Self {
        let cache = MokaCache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_seconds))
            .build();

        Self { cache }
    }

    pub async fn get<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let bytes = self.cache.get(key).await?;
        serde_json::from_slice(&bytes).ok()
    }

    pub async fn set<T: Serialize>(&self, key: String, value: &T) {
        if let Ok(bytes) = serde_json::to_vec(value) {
            self.cache.insert(key, bytes).await;
        }
    }

    pub async fn invalidate(&self, key: &str) {
        self.cache.invalidate(key).await;
    }

    pub async fn invalidate_all(&self) {
        self.cache.invalidate_all();
    }

    pub fn build_key(prefix: &str, parts: &[&str]) -> String {
        let mut key = prefix.to_string();
        for part in parts {
            key.push(':');
            key.push_str(part);
        }
        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        value: String,
    }

    #[tokio::test]
    async fn test_cache_operations() {
        let cache = CacheManager::new(300, 1000);
        let key = "test:key";
        let data = TestData {
            value: "test".to_string(),
        };

        // Set value
        cache.set(key.to_string(), &data).await;

        // Get value
        let retrieved: Option<TestData> = cache.get(key).await;
        assert_eq!(retrieved, Some(data));

        // Invalidate
        cache.invalidate(key).await;
        let retrieved: Option<TestData> = cache.get(key).await;
        assert_eq!(retrieved, None);
    }

    #[test]
    fn test_build_key() {
        let key = CacheManager::build_key("user", &["123", "profile"]);
        assert_eq!(key, "user:123:profile");
    }
}
