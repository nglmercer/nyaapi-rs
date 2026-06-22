use crate::error::{NyaaError, Result};
use crate::parser::{self};
use crate::types::{
    Category, NyaaMode, NyaaOptions, SearchByUserOptions, SearchOptions, SearchResult, SortBy,
    Torrent, TorrentDetail,
};
use url::Url;

#[derive(Debug, Clone, Default)]
pub struct Nyaa {
    options: NyaaOptions,
}

impl Nyaa {
    pub fn new(options: NyaaOptions) -> Self {
        Self { options }
    }

    pub async fn search(
        &self,
        query: impl Into<String>,
        options: SearchOptions,
    ) -> Result<SearchResult> {
        let query = query.into();
        let start = std::time::Instant::now();
        let page = options.page.unwrap_or(1).max(1);
        let category = options.category.unwrap_or_default();
        let filter = options.filter.unwrap_or_default();
        let sort = options.sort.unwrap_or_default();
        let order = options.order.unwrap_or_default();

        let category_code = map_category(category);
        let filter_code = map_filter(filter);
        let sort_code = if matches!(sort, SortBy::Date) {
            "id"
        } else {
            sort.as_str()
        };
        let order_code = order.as_str();

        if matches!(self.options.mode, NyaaMode::Rss) {
            let url = build_url(
                &self.options.base_url,
                &[
                    ("page", "rss"),
                    ("q", &query),
                    ("c", category_code),
                    ("f", filter_code),
                    ("p", &page.to_string()),
                    ("s", sort_code),
                    ("o", order_code),
                ],
            )?;
            let html = reqwest::get(&url).await?.text().await?;
            let torrents = parser::search::parse_search_results_rss(&html);
            return Ok(SearchResult {
                data: torrents,
                total: Some(0),
                page: 0,
                total_page: Some(0),
                per_page: 0,
                range: None,
                next_page: false,
                time_taken: start.elapsed().as_millis(),
            });
        }

        let url = build_url(
            &self.options.base_url,
            &[
                ("q", &query),
                ("c", category_code),
                ("f", filter_code),
                ("p", &page.to_string()),
                ("s", sort_code),
                ("o", order_code),
            ],
        )?;
        let html = reqwest::get(&url).await?.text().await?;
        let torrents = parser::search::parse_search_results(&html);
        let pagination = if !query.is_empty() {
            parser::search::parse_pagination(&html)
        } else {
            None
        };
        let per_page = torrents.len();

        Ok(SearchResult {
            data: torrents,
            total: pagination.as_ref().and_then(|p| p.total),
            page,
            total_page: pagination.as_ref().and_then(|p| p.total_page),
            per_page,
            range: pagination.as_ref().and_then(|p| p.range.clone()),
            next_page: pagination.as_ref().map(|p| p.next_page).unwrap_or(false),
            time_taken: start.elapsed().as_millis(),
        })
    }

    pub async fn search_by_user(
        &self,
        username: impl Into<String>,
        options: SearchByUserOptions,
    ) -> Result<Vec<Torrent>> {
        let username = username.into();
        let page = options.page.unwrap_or(1).max(1);
        let category = options.category.unwrap_or_default();
        let filter = options.filter.unwrap_or_default();
        let sort = options.sort.unwrap_or_default();
        let order = options.order.unwrap_or_default();
        let query = options.query.unwrap_or_default();

        let category_code = map_category(category);
        let filter_code = map_filter(filter);
        let sort_code = if matches!(sort, SortBy::Date) {
            "id"
        } else {
            sort.as_str()
        };
        let order_code = order.as_str();

        if matches!(self.options.mode, NyaaMode::Rss) {
            let url = build_url(
                &self.options.base_url,
                &[
                    ("page", "rss"),
                    ("u", &username),
                    ("q", &query),
                    ("c", category_code),
                    ("f", filter_code),
                    ("p", &page.to_string()),
                    ("s", sort_code),
                    ("o", order_code),
                ],
            )?;
            let html = reqwest::get(&url).await?.text().await?;
            return Ok(parser::search::parse_search_results_rss(&html));
        }

        let url = build_url(
            &self.options.base_url,
            &[
                ("u", &username),
                ("q", &query),
                ("c", category_code),
                ("f", filter_code),
                ("p", &page.to_string()),
                ("s", sort_code),
                ("o", order_code),
            ],
        )?;
        let html = reqwest::get(&url).await?.text().await?;
        Ok(parser::search::parse_search_results(&html))
    }

    pub async fn view(&self, id: u64) -> Result<Option<TorrentDetail>> {
        let base_url = self.options.base_url.trim_end_matches('/');
        let url = format!("{}/view/{}", base_url, id);
        let html = reqwest::get(&url).await?.text().await?;
        Ok(parser::view::parse_view_page(&html, id))
    }

    pub async fn view_from_torrent(&self, torrent: &Torrent) -> Result<Option<TorrentDetail>> {
        let base_url = self.options.base_url.trim_end_matches('/');
        let view_url = match &torrent.view_url {
            Some(url) if url.starts_with('/') => format!("{}{}", base_url, url),
            Some(url) => url.clone(),
            None => return self.view(torrent.id).await,
        };
        let html = reqwest::get(&view_url).await?.text().await?;
        Ok(parser::view::parse_view_page(&html, torrent.id))
    }

    pub async fn get_categories(&self) -> Result<Vec<Category>> {
        let base_url = self.options.base_url.trim_end_matches('/');
        let url = format!("{}/", base_url);
        let html = reqwest::get(&url).await?.text().await?;
        Ok(parser::category::parse_categories(&html))
    }
}

fn map_category(category: crate::types::CategoryFilter) -> &'static str {
    match category {
        crate::types::CategoryFilter::Anime => "1_0",
        crate::types::CategoryFilter::Audio => "2_0",
        crate::types::CategoryFilter::Literature => "3_0",
        crate::types::CategoryFilter::LiveAction => "4_0",
        crate::types::CategoryFilter::Pictures => "5_0",
        crate::types::CategoryFilter::Software => "6_0",
        crate::types::CategoryFilter::Games => "7_0",
        _ => "0_0",
    }
}

fn map_filter(filter: crate::types::TrustedFilter) -> &'static str {
    match filter {
        crate::types::TrustedFilter::TrustedOnly => "2",
        crate::types::TrustedFilter::NoRemakes => "1",
        _ => "0",
    }
}

fn build_url(base: &str, params: &[(&str, &str)]) -> Result<String> {
    let mut url = Url::parse(base).map_err(NyaaError::from)?;
    {
        let mut query = url.query_pairs_mut();
        for (k, v) in params {
            query.append_pair(k, v);
        }
    }
    Ok(url.to_string())
}
