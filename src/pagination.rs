use serde::Serialize;

/// A paginated response containing a slice of items.
#[derive(Debug, Clone, Serialize)]
pub struct Page<T> {
    items: Vec<T>,
    offset: usize,
    limit: usize,
    total: usize,
}

impl<T> Page<T> {
    /// Create a page from a full result set.
    ///
    /// # Examples
    ///
    /// ```
    /// use github_actions_demo::pagination::Page;
    ///
    /// let all_items: Vec<i32> = (0..100).collect();
    /// let page = Page::from_vec(all_items, 10, 5);
    /// assert_eq!(page.items().len(), 5);
    /// assert_eq!(page.total(), 100);
    /// assert!(page.has_next());
    /// ```
    pub fn from_vec(all: Vec<T>, offset: usize, limit: usize) -> Self {
        let total = all.len();
        let items: Vec<T> = all.into_iter().skip(offset).take(limit).collect();

        Self {
            items,
            offset,
            limit,
            total,
        }
    }

    pub fn items(&self) -> &[T] {
        &self.items
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn has_next(&self) -> bool {
        self.offset + self.items.len() < self.total
    }

    pub fn has_previous(&self) -> bool {
        self.offset > 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_data() -> Vec<String> {
        (0..25).map(|i| format!("item-{i}")).collect()
    }

    #[test]
    fn first_page() {
        let page = Page::from_vec(sample_data(), 0, 10);
        assert_eq!(page.items().len(), 10);
        assert_eq!(page.offset(), 0);
        assert_eq!(page.total(), 25);
        assert!(page.has_next());
        assert!(!page.has_previous());
    }

    #[test]
    fn middle_page() {
        let page = Page::from_vec(sample_data(), 10, 10);
        assert_eq!(page.items().len(), 10);
        assert!(page.has_next());
        assert!(page.has_previous());
    }

    #[test]
    fn last_page() {
        let page = Page::from_vec(sample_data(), 20, 10);
        assert_eq!(page.items().len(), 5);
        assert!(!page.has_next());
        assert!(page.has_previous());
    }

    #[test]
    fn offset_beyond_total() {
        let page = Page::from_vec(sample_data(), 100, 10);
        assert!(page.items().is_empty());
        assert!(!page.has_next());
    }

    #[test]
    fn zero_limit() {
        let page = Page::from_vec(sample_data(), 0, 0);
        assert!(page.items().is_empty());
        assert!(page.has_next());
    }

    #[test]
    fn empty_source() {
        let page: Page<String> = Page::from_vec(vec![], 0, 10);
        assert!(page.items().is_empty());
        assert_eq!(page.total(), 0);
        assert!(!page.has_next());
        assert!(!page.has_previous());
    }

    #[test]
    fn serializes_to_json() {
        let page = Page::from_vec(vec![1, 2, 3], 0, 10);
        let json = serde_json::to_string(&page).unwrap();
        assert!(json.contains("\"total\":3"));
        assert!(json.contains("\"items\":[1,2,3]"));
    }
}
