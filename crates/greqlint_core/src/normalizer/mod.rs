mod providers;

pub trait Normalizer {
    fn normalize_issues(&self, issues: Vec<String>) -> Vec<Issue> {
        issues
            .into_iter()
            .map(|issue| self.normalize_issue(issue))
            .collect()
    }

    /// # normalize issue
    /// ```rust
    /// let normalizer = GitLabRunnerNormalizer::new();
    /// assert_eq!(
    ///    normalizer.normalize_issue("#42".to_string()),
    ///    Issue { number: 42 }
    /// );
    ///
    /// let normalizer = GitActionsNormalizer::new();
    /// assert_eq!(
    ///    normalizer.normalize_issue("#42".to_string()),
    ///    Issue { number: 42 }
    /// );
    /// ```
    fn normalize_issue(&self, issues: String) -> Issue;

    /// # normalize request
    /// ```rust
    /// let normalizer = GitLabRunnerNormalizer::new();
    /// assert_eq!(
    ///    normalizer.normalize_request("!42".to_string()),
    ///    Issue { number: 42 }
    /// );
    ///
    /// let normalizer = GitActionsNormalizer::new();
    /// assert_eq!(
    ///    normalizer.normalize_request("#42".to_string()),
    ///    Issue { number: 42 }
    /// );
    /// ```
    fn normalize_request(&self, request: String) -> String;
}

pub struct Issue {
    pub number: u32,
}

/// non user specific variables (from runner process env vars)
pub struct RunnerVariables {
    pub title: String,
    pub branch_name: String,
}
