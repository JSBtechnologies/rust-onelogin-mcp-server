// Allow dead code - pagination utilities available for future use
#![allow(dead_code)]

use crate::core::error::Result;
use serde::de::DeserializeOwned;
use tracing::debug;

/// Pagination result with metadata
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total_fetched: usize,
    pub pages_fetched: usize,
    pub has_more: bool,
    pub next_cursor: Option<String>,
}

/// Fetch all pages of a paginated endpoint
///
/// # Arguments
/// * `fetch_page` - Async function that fetches a single page given a cursor
/// * `max_pages` - Maximum number of pages to fetch (None = unlimited)
/// * `max_items` - Maximum number of items to fetch (None = unlimited)
///
/// # Example
/// ```no_run
/// use onelogin_mcp_server::utils::pagination::fetch_all_pages;
///
/// let result = fetch_all_pages(
///     |cursor| async move {
///         // Fetch page with cursor
///         api.list_users(cursor).await
///     },
///     Some(10),  // max 10 pages
///     Some(100), // max 100 items
/// ).await?;
/// ```
pub async fn fetch_all_pages<T, F, Fut>(
    mut fetch_page: F,
    max_pages: Option<usize>,
    max_items: Option<usize>,
) -> Result<PaginationResult<T>>
where
    T: DeserializeOwned,
    F: FnMut(Option<String>) -> Fut,
    Fut: std::future::Future<Output = Result<PageResponse<T>>>,
{
    let mut all_items = Vec::new();
    let mut cursor: Option<String> = None;
    let mut pages_fetched = 0;
    let max_pages_limit = max_pages.unwrap_or(usize::MAX);
    let max_items_limit = max_items.unwrap_or(usize::MAX);

    loop {
        if pages_fetched >= max_pages_limit {
            debug!(
                "Reached max pages limit: {} pages fetched",
                pages_fetched
            );
            break;
        }

        if all_items.len() >= max_items_limit {
            debug!(
                "Reached max items limit: {} items fetched",
                all_items.len()
            );
            break;
        }

        // Fetch next page
        let page = fetch_page(cursor.clone()).await?;
        pages_fetched += 1;

        debug!(
            "Fetched page {} with {} items",
            pages_fetched,
            page.items.len()
        );

        // Add items (respecting max_items limit)
        let remaining_capacity = max_items_limit.saturating_sub(all_items.len());
        let items_to_take = page.items.len().min(remaining_capacity);
        all_items.extend(page.items.into_iter().take(items_to_take));

        // Check if there are more pages
        cursor = page.next_cursor;
        if cursor.is_none() {
            debug!("No more pages available");
            break;
        }

        if all_items.len() >= max_items_limit {
            break;
        }
    }

    Ok(PaginationResult {
        total_fetched: all_items.len(),
        pages_fetched,
        has_more: cursor.is_some(),
        next_cursor: cursor,
        items: all_items,
    })
}

/// Response from a single page fetch
pub struct PageResponse<T> {
    pub items: Vec<T>,
    pub next_cursor: Option<String>,
}

impl<T> PageResponse<T> {
    pub fn new(items: Vec<T>, next_cursor: Option<String>) -> Self {
        Self { items, next_cursor }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_all_pages_with_limit() {
        let mut call_count = 0;

        let fetch = |cursor: Option<String>| async move {
            call_count += 1;
            if call_count <= 3 {
                Ok(PageResponse::new(
                    vec![call_count],
                    Some(format!("cursor_{}", call_count)),
                ))
            } else {
                Ok(PageResponse::new(vec![call_count], None))
            }
        };

        let result = fetch_all_pages(fetch, Some(2), None).await.unwrap();

        assert_eq!(result.pages_fetched, 2);
        assert_eq!(result.total_fetched, 2);
        assert!(result.has_more);
    }

    #[tokio::test]
    async fn test_fetch_all_pages_with_max_items() {
        let mut call_count = 0;

        let fetch = |_cursor: Option<String>| async move {
            call_count += 1;
            Ok(PageResponse::new(
                vec![1, 2, 3, 4, 5],
                Some(format!("cursor_{}", call_count)),
            ))
        };

        let result = fetch_all_pages(fetch, None, Some(12)).await.unwrap();

        assert_eq!(result.total_fetched, 12); // 3 pages of 5 items, but limited to 12
        assert_eq!(result.pages_fetched, 3);
    }
}
