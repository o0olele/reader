//! Fixture export for turning a failing source into a regression test.

use crate::{error::AppError, repository::SourceRepository};

use super::SourceDebugService;

impl SourceDebugService {
    /// Writes the current source definition and a captured response into a
    /// fixture directory so a real failure can be turned into a regression
    /// test. Credentials are cleared before serialization.
    pub async fn export_fixture(
        &self,
        source_id: i64,
        html: &str,
        out_dir: &std::path::Path,
    ) -> Result<String, AppError> {
        let mut source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        source.access_token = None;
        source.session_cookie = None;
        source.session_expires_at = None;
        std::fs::create_dir_all(out_dir).map_err(AppError::io)?;
        let slug = slugify_source_name(&source.name);
        let source_path = out_dir.join(format!("{slug}.source.json"));
        let html_path = out_dir.join(format!("{slug}.response.html"));
        let source_json = serde_json::to_string_pretty(&source).map_err(AppError::parse)?;
        std::fs::write(&source_path, source_json).map_err(AppError::io)?;
        std::fs::write(&html_path, html).map_err(AppError::io)?;
        Ok(source_path.display().to_string())
    }
}

fn slugify_source_name(name: &str) -> String {
    let mut slug = String::new();
    for character in name.chars() {
        if character.is_ascii_alphanumeric() || character == '-' || character == '_' {
            slug.push(character.to_ascii_lowercase());
        } else if character.is_whitespace() || matches!(character, '·' | '（' | '）' | '(' | ')') {
            if !slug.ends_with('-') {
                slug.push('-');
            }
        } else if slug.is_empty() {
            slug.push_str("source");
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "source".into()
    } else {
        slug.to_owned()
    }
}
