//! URL resolution shared by the source, search and reader workflows.

use crate::error::AppError;

/// Resolves `value` as an absolute URL, falling back to joining it onto `base`.
///
/// `label` names the URL in the error message, e.g. `"搜索 URL"`.
pub fn resolve_url(base: &str, value: &str, label: &str) -> Result<reqwest::Url, AppError> {
    reqwest::Url::parse(value)
        .or_else(|_| reqwest::Url::parse(base).and_then(|base| base.join(value)))
        .map_err(|error| AppError::InvalidArgument(format!("{label} 无效: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_absolute_url_ignores_the_base() {
        let url = resolve_url("https://base.test", "https://other.test/x", "搜索 URL").unwrap();
        assert_eq!(url.as_str(), "https://other.test/x");
    }

    #[test]
    fn a_relative_url_is_joined_onto_the_base() {
        let url = resolve_url("https://base.test/a/b", "../c/d.html", "目录 URL").unwrap();
        assert_eq!(url.as_str(), "https://base.test/c/d.html");
    }

    #[test]
    fn an_unresolvable_url_reports_the_label() {
        let error = resolve_url("not a url", "also/not/absolute", "分页 URL").unwrap_err();
        assert!(matches!(error, AppError::InvalidArgument(_)));
        assert!(error.to_string().contains("分页 URL 无效"));
    }
}
