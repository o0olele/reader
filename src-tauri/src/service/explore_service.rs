use crate::{
    domain::source::BookSearchResult,
    error::AppError,
    infrastructure::http::{client::build_source_client, request::response_error},
    repository::{source::SqliteSourceRepository, SourceRepository},
    service::settings_service::SettingsService,
    source_engine::{
        pipeline::parse_explore,
        url::{build as build_url_request, decode_text, send},
    },
};
use serde::Serialize;

#[derive(Clone)]
pub struct ExploreService {
    sources: SqliteSourceRepository,
    settings: SettingsService,
}

#[derive(Debug, Serialize)]
pub struct ExploreCategory {
    pub source_id: i64,
    pub source_name: String,
    pub title: String,
    pub url: String,
}

impl ExploreService {
    pub fn new(pool: sqlx::SqlitePool) -> Self {
        Self {
            sources: SqliteSourceRepository::new(pool.clone()),
            settings: SettingsService::new(pool),
        }
    }

    pub async fn categories(&self) -> Result<Vec<ExploreCategory>, AppError> {
        let mut categories = Vec::new();
        for source in self
            .sources
            .list()
            .await?
            .into_iter()
            .filter(|source| source.enabled)
        {
            let Some(raw) = source.explore_url.as_deref() else {
                continue;
            };
            for (title, url) in parse_categories(raw, &source.name) {
                categories.push(ExploreCategory {
                    source_id: source.id,
                    source_name: source.name.clone(),
                    title,
                    url,
                });
            }
        }
        Ok(categories)
    }

    pub async fn books(
        &self,
        source_id: i64,
        raw_url: &str,
    ) -> Result<Vec<BookSearchResult>, AppError> {
        let source = self
            .sources
            .get(source_id)
            .await?
            .ok_or_else(|| AppError::Source("书源不存在".into()))?;
        let configured = source
            .explore_url
            .as_deref()
            .ok_or_else(|| AppError::Source("该书源未配置发现页".into()))?;
        if !parse_categories(configured, &source.name)
            .iter()
            .any(|(_, url)| url == raw_url)
        {
            return Err(AppError::InvalidArgument("发现页 URL 不属于该书源".into()));
        }
        let client = build_source_client(&source, 15, self.settings.proxy_url().await?.as_deref())?;
        let request = build_url_request(&source, raw_url, None, "发现 URL")?;
        let response = send(&client, &source, &request).await?;
        if !response.status().is_success() {
            return Err(AppError::Network(
                response_error(response, &source.name).await,
            ));
        }
        parse_explore(&source, &decode_text(response, &request, &source).await?)
    }
}

fn parse_categories(raw: &str, fallback: &str) -> Vec<(String, String)> {
    raw.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_once("::").map_or_else(
                || (fallback.to_owned(), line.to_owned()),
                |(title, url)| (title.trim().to_owned(), url.trim().to_owned()),
            )
        })
        .filter(|(_, url)| !url.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_categories;

    #[test]
    fn parses_named_and_single_explore_urls() {
        assert_eq!(
            parse_categories("热门::/hot\n完结::/done", "源"),
            vec![
                ("热门".into(), "/hot".into()),
                ("完结".into(), "/done".into())
            ]
        );
        assert_eq!(
            parse_categories("/all", "源"),
            vec![("源".into(), "/all".into())]
        );
    }
}
