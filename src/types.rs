use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Torrent {
    pub id: u64,
    pub name: String,
    pub magnet: String,
    pub size: String,
    pub category: String,
    pub sub_category: Option<String>,
    pub date: DateTime<Utc>,
    pub seeders: u64,
    pub leechers: u64,
    pub downloads: u64,
    pub hash: Option<String>,
    pub submitter: Option<String>,
    pub submitter_id: Option<String>,
    pub information: Option<String>,
    pub completed: Option<u64>,
    pub description: Option<String>,
    pub torrent_url: Option<String>,
    pub view_url: Option<String>,
    pub comments: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TorrentFile {
    pub name: String,
    pub size: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct TorrentDetail {
    pub id: u64,
    pub title: String,
    pub name: String,
    pub category: String,
    pub sub_category: String,
    pub date: DateTime<Utc>,
    pub seeders: u64,
    pub leechers: u64,
    pub downloads: u64,
    pub completed: Option<u64>,
    pub magnet: String,
    pub size: String,
    pub hash: Option<String>,
    pub submitter: Option<String>,
    pub submitter_id: Option<String>,
    pub information: Option<String>,
    pub description: String,
    pub files: Vec<TorrentFile>,
    pub comments: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct SearchResult {
    pub data: Vec<Torrent>,
    pub total: Option<u64>,
    pub page: u64,
    pub total_page: Option<u64>,
    pub per_page: usize,
    pub range: Option<String>,
    pub next_page: bool,
    pub time_taken: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginationInfo {
    pub total: Option<u64>,
    pub total_page: Option<u64>,
    pub page: u64,
    pub per_page: usize,
    pub range: Option<String>,
    pub next_page: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub sub_categories: Vec<Category>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
pub enum CategoryFilter {
    #[default]
    All,
    Anime,
    Audio,
    Literature,
    LiveAction,
    Pictures,
    Software,
    Games,
}

impl CategoryFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::All => "all",
            Self::Anime => "anime",
            Self::Audio => "audio",
            Self::Literature => "literature",
            Self::LiveAction => "live-action",
            Self::Pictures => "pictures",
            Self::Software => "software",
            Self::Games => "games",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
pub enum TrustedFilter {
    #[default]
    NoFilter,
    TrustedOnly,
    NoRemakes,
}

impl TrustedFilter {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::NoFilter => "no filter",
            Self::TrustedOnly => "trusted only",
            Self::NoRemakes => "no remakes",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
pub enum SortBy {
    Comments,
    Size,
    #[default]
    Date,
    Seeders,
    Leechers,
    Downloads,
}

impl SortBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Comments => "comments",
            Self::Size => "size",
            Self::Date => "date",
            Self::Seeders => "seeders",
            Self::Leechers => "leechers",
            Self::Downloads => "downloads",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq, Default)]
pub enum Order {
    #[default]
    Desc,
    Asc,
}

impl Order {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Desc => "desc",
            Self::Asc => "asc",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchOptions {
    pub page: Option<u64>,
    pub category: Option<CategoryFilter>,
    pub filter: Option<TrustedFilter>,
    pub sort: Option<SortBy>,
    pub order: Option<Order>,
}

#[derive(Debug, Clone, Default)]
pub struct SearchByUserOptions {
    pub page: Option<u64>,
    pub category: Option<CategoryFilter>,
    pub filter: Option<TrustedFilter>,
    pub sort: Option<SortBy>,
    pub order: Option<Order>,
    pub query: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NyaaMode {
    Html,
    Rss,
}

#[derive(Debug, Clone)]
pub struct NyaaOptions {
    pub base_url: String,
    pub mode: NyaaMode,
}

impl Default for NyaaOptions {
    fn default() -> Self {
        Self {
            base_url: "https://nyaa.si/".to_string(),
            mode: NyaaMode::Html,
        }
    }
}
