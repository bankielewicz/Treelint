//! Query filter builders for symbol search.
//!
//! This module provides the [`QueryFilters`] struct for building combined
//! search queries with multiple filter criteria.

use crate::parser::{Language, SymbolType, Visibility};

/// Builder for constructing symbol query filters.
///
/// `QueryFilters` allows combining multiple search criteria for flexible
/// symbol queries. Filters are applied using AND logic.
///
/// # Example
///
/// ```
/// use treelint::index::QueryFilters;
/// use treelint::parser::SymbolType;
///
/// let filters = QueryFilters::new()
///     .with_type(SymbolType::Function)
///     .with_file("src/auth.rs");
/// ```
#[derive(Debug, Default, Clone)]
pub struct QueryFilters {
    /// Exact name match (case-sensitive).
    pub(crate) name: Option<String>,
    /// Name match (case-insensitive).
    pub(crate) name_case_insensitive: Option<String>,
    /// Name pattern match using SQL LIKE (contains pattern).
    pub(crate) name_pattern: Option<String>,
    /// Symbol type filter.
    pub(crate) symbol_type: Option<SymbolType>,
    /// File path filter.
    pub(crate) file_path: Option<String>,
    /// Visibility filter.
    pub(crate) visibility: Option<Visibility>,
    /// Language filter.
    pub(crate) language: Option<Language>,
}

impl QueryFilters {
    /// Create a new empty filter set.
    ///
    /// # Returns
    ///
    /// A new `QueryFilters` with no filters applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    ///
    /// let filters = QueryFilters::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by exact symbol name (case-sensitive).
    ///
    /// # Arguments
    ///
    /// * `name` - The exact symbol name to match
    ///
    /// # Returns
    ///
    /// Self with the name filter applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    ///
    /// let filters = QueryFilters::new()
    ///     .with_name("validateUser");
    /// ```
    pub fn with_name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    /// Filter by symbol name (case-insensitive).
    ///
    /// # Arguments
    ///
    /// * `name` - The symbol name to match (ignoring case)
    ///
    /// # Returns
    ///
    /// Self with the case-insensitive name filter applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    ///
    /// let filters = QueryFilters::new()
    ///     .with_name_case_insensitive("VALIDATEUSER");
    /// ```
    pub fn with_name_case_insensitive(mut self, name: &str) -> Self {
        self.name_case_insensitive = Some(name.to_string());
        self
    }

    /// Filter by symbol name pattern (contains match).
    ///
    /// Uses SQL LIKE with wildcards for partial matching.
    /// The pattern is wrapped with '%' on both sides automatically.
    ///
    /// # Arguments
    ///
    /// * `pattern` - The pattern to search for within symbol names
    ///
    /// # Returns
    ///
    /// Self with the name pattern filter applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    ///
    /// // Matches "foo_function", "bar_foo", "foobar", etc.
    /// let filters = QueryFilters::new()
    ///     .with_name_pattern("foo");
    /// ```
    pub fn with_name_pattern(mut self, pattern: &str) -> Self {
        self.name_pattern = Some(pattern.to_string());
        self
    }

    /// Filter by symbol type.
    ///
    /// # Arguments
    ///
    /// * `symbol_type` - The type of symbols to match
    ///
    /// # Returns
    ///
    /// Self with the type filter applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    /// use treelint::parser::SymbolType;
    ///
    /// let filters = QueryFilters::new()
    ///     .with_type(SymbolType::Function);
    /// ```
    pub fn with_type(mut self, symbol_type: SymbolType) -> Self {
        self.symbol_type = Some(symbol_type);
        self
    }

    /// Filter by file path.
    ///
    /// # Arguments
    ///
    /// * `file_path` - The file path to match
    ///
    /// # Returns
    ///
    /// Self with the file path filter applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    ///
    /// let filters = QueryFilters::new()
    ///     .with_file("src/auth.rs");
    /// ```
    pub fn with_file(mut self, file_path: &str) -> Self {
        self.file_path = Some(file_path.to_string());
        self
    }

    /// Filter by visibility.
    ///
    /// # Arguments
    ///
    /// * `visibility` - The visibility level to match
    ///
    /// # Returns
    ///
    /// Self with the visibility filter applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    /// use treelint::parser::Visibility;
    ///
    /// let filters = QueryFilters::new()
    ///     .with_visibility(Visibility::Public);
    /// ```
    pub fn with_visibility(mut self, visibility: Visibility) -> Self {
        self.visibility = Some(visibility);
        self
    }

    /// Filter by programming language.
    ///
    /// # Arguments
    ///
    /// * `language` - The programming language to match
    ///
    /// # Returns
    ///
    /// Self with the language filter applied.
    ///
    /// # Example
    ///
    /// ```
    /// use treelint::index::QueryFilters;
    /// use treelint::parser::Language;
    ///
    /// let filters = QueryFilters::new()
    ///     .with_language(Language::Rust);
    /// ```
    pub fn with_language(mut self, language: Language) -> Self {
        self.language = Some(language);
        self
    }

    /// Check if any filters are set.
    ///
    /// # Returns
    ///
    /// `true` if at least one filter is set, `false` if all filters are empty.
    pub fn has_filters(&self) -> bool {
        self.name.is_some()
            || self.name_case_insensitive.is_some()
            || self.name_pattern.is_some()
            || self.symbol_type.is_some()
            || self.file_path.is_some()
            || self.visibility.is_some()
            || self.language.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_filters() {
        let filters = QueryFilters::new();
        assert!(!filters.has_filters());
    }

    #[test]
    fn test_with_name() {
        let filters = QueryFilters::new().with_name("test");
        assert!(filters.has_filters());
        assert_eq!(filters.name, Some("test".to_string()));
    }

    #[test]
    fn test_with_type() {
        let filters = QueryFilters::new().with_type(SymbolType::Function);
        assert!(filters.has_filters());
        assert_eq!(filters.symbol_type, Some(SymbolType::Function));
    }

    #[test]
    fn test_chained_filters() {
        let filters = QueryFilters::new()
            .with_name("test")
            .with_type(SymbolType::Function)
            .with_file("src/lib.rs");

        assert!(filters.has_filters());
        assert_eq!(filters.name, Some("test".to_string()));
        assert_eq!(filters.symbol_type, Some(SymbolType::Function));
        assert_eq!(filters.file_path, Some("src/lib.rs".to_string()));
    }

    #[test]
    fn test_clone() {
        let filters = QueryFilters::new().with_name("test");
        let cloned = filters.clone();
        assert_eq!(cloned.name, filters.name);
    }
}
