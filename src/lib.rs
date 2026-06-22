pub mod client;
pub mod error;
pub mod parser;
pub mod types;

pub use client::Nyaa;
pub use error::{NyaaError, Result};
pub use types::{
    Category, CategoryFilter, NyaaMode, NyaaOptions, Order, SearchByUserOptions, SearchOptions,
    SearchResult, SortBy, Torrent, TorrentDetail, TorrentFile, TrustedFilter,
};
