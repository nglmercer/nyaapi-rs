# nyaapi-rs

[![Crates.io](https://img.shields.io/crates/v/nyaapi-rs.svg)](https://crates.io/crates/nyaapi-rs)
[![Docs.rs](https://img.shields.io/docsrs/nyaapi-rs/latest)](https://docs.rs/nyaapi-rs)
[![Crates.io license](https://img.shields.io/crates/l/nyaapi-rs.svg)](LICENSE)

Unofficial async API wrapper for [nyaa.si](https://nyaa.si) and compatible instances. Search torrents by query, category, user, or fetch details directly.

Works with any nyaa instance (e.g. `nyaa.si`, `nyaa.land`) via the `base_url` option.

<details>
<summary>Table of Contents</summary>

- [Install](#install)
- [Usage](#usage)
- [CLI Example](#cli-example)
- [API Reference](#api-reference)
  - [`Nyaa`](#nyaasearchoptions-)
  - [`SearchOptions`](#searchoptions)
  - [`SearchByUserOptions`](#searchbyuseroptions)
  - [`Torrent`](#torrent)
  - [`TorrentDetail`](#torrentdetail)
  - [`SearchResult`](#searchresult)
  - [`Category`](#category)
  - [`NyaaOptions` and modes](#nyaaoptions-and-modes)
- [RSS Mode](#rss-mode)
- [Features](#features)
- [Contributing](#contributing)
- [License](#license)
- [Disclaimer](#disclaimer)

</details>

## Install

```toml
[dependencies]
nyaapi-rs = "0.1"
```

```bash
cargo add nyaapi-rs
```

## Usage

```rust
use nyaapi_rs::{Nyaa, NyaaOptions, NyaaMode};

#[tokio::main]
async fn main() {
    let nyaa = Nyaa::new(NyaaOptions {
        base_url: "https://nyaa.si".to_string(),
        mode: NyaaMode::Html,
    });

    let result = nyaa.search("One Piece").await.unwrap();

    println!("Page {}", result.page);
    if let Some(total) = result.total {
        println!("Total: {}", total);
    }
    for torrent in result.data {
        println!(
            "[{}] {} | S:{} L:{} | {}",
            torrent.id, torrent.name, torrent.seeders, torrent.leechers, torrent.date
        );
    }
}
```

## CLI Example

A CLI is included under `examples/`. Run it with:

```bash
cargo run --example example -- search "One Piece"
cargo run --example example -- search "One Piece" --category anime --sort seeders --order desc --page 2
cargo run --example example -- search-by-user Fan-Kai --query "One Piece"
cargo run --example example -- view 2099890
cargo run --example example -- categories
cargo run --example example -- search "Anime" --mode rss --base-url https://nyaa.land
```

## API Reference

### `Nyaa::search(query, options)`

Search for torrents.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `query` | `impl Into<String>` | `""` | Search query string |
| `page` | `Option<u64>` | `1` | Page number |
| `category` | `Option<CategoryFilter>` | `All` | Category filter |
| `filter` | `Option<TrustedFilter>` | `NoFilter` | Trust filter |
| `sort` | `Option<SortBy>` | `Date` | Sort field |
| `order` | `Option<Order>` | `Desc` | Sort order |

Returns `Result<SearchResult, NyaaError>`.

### `Nyaa::search_by_user(username, options)`

Search for torrents uploaded by a user.

| Field | Type | Default | Description |
|-------|------|---------|-------------|
| `username` | `impl Into<String>` | — | Uploader username |
| `page` | `Option<u64>` | `1` | Page number |
| `category` | `Option<CategoryFilter>` | `All` | Category filter |
| `filter` | `Option<TrustedFilter>` | `NoFilter` | Trust filter |
| `sort` | `Option<SortBy>` | `Date` | Sort field |
| `order` | `Option<Order>` | `Desc` | Sort order |
| `query` | `Option<String>` | `None` | Optional search query |

Returns `Result<Vec<Torrent>, NyaaError>`.

### `Nyaa::view(id)`

Get full torrent details by ID.

Returns `Result<Option<TorrentDetail>, NyaaError>`.

### `Nyaa::view_from_torrent(torrent)`

Fetch details using a `Torrent` object's `view_url`.

Returns `Result<Option<TorrentDetail>, NyaaError>`.

### `Nyaa::get_categories()`

Fetch the category tree from the instance root.

Returns `Result<Vec<Category>, NyaaError>`.

---

### `SearchOptions`

```rust
pub struct SearchOptions {
    pub page: Option<u64>,
    pub category: Option<CategoryFilter>,
    pub filter: Option<TrustedFilter>,
    pub sort: Option<SortBy>,
    pub order: Option<Order>,
}
```

### `SearchByUserOptions`

```rust
pub struct SearchByUserOptions {
    pub page: Option<u64>,
    pub category: Option<CategoryFilter>,
    pub filter: Option<TrustedFilter>,
    pub sort: Option<SortBy>,
    pub order: Option<Order>,
    pub query: Option<String>,
}
```

### `SearchResult`

```rust
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
```

### `Torrent`

```rust
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
```

### `TorrentDetail`

```rust
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
```

### `TorrentFile`

```rust
pub struct TorrentFile {
    pub name: String,
    pub size: String,
}
```

### `Category`

```rust
pub struct Category {
    pub id: String,
    pub name: String,
    pub sub_categories: Vec<Category>,
}
```

### `CategoryFilter`

```rust
pub enum CategoryFilter {
    All,
    Anime,
    Audio,
    Literature,
    LiveAction,
    Pictures,
    Software,
    Games,
}
```

### `TrustedFilter`

```rust
pub enum TrustedFilter {
    NoFilter,
    TrustedOnly,
    NoRemakes,
}
```

### `SortBy`

```rust
pub enum SortBy {
    Comments,
    Size,
    Date,
    Seeders,
    Leechers,
    Downloads,
}
```

### `Order`

```rust
pub enum Order {
    Desc,
    Asc,
}
```

### `NyaaOptions` and modes

```rust
pub enum NyaaMode {
    Html,
    Rss,
}

pub struct NyaaOptions {
    pub base_url: String,
    pub mode: NyaaMode,
}
```

`NyaaOptions` implements `Default` with `base_url = "https://nyaa.si"` and `mode = NyaaMode::Html`.

---

## RSS Mode

```rust
use nyaapi_rs::{Nyaa, NyaaMode, NyaaOptions};

#[tokio::main]
async fn main() {
    let nyaa = Nyaa::new(NyaaOptions {
        base_url: "https://nyaa.si".to_string(),
        mode: NyaaMode::Rss,
    });

    let result = nyaa.search("Anime").await.unwrap();
    // `total`, `page`, and `total_page` are not available in RSS mode
    for torrent in result.data {
        println!("{}", torrent.name);
    }
}
```

## Features

- Async-first API built on `reqwest` and `tokio`
- HTML and RSS parsing
- Search, user search, view, and categories
- Strongly typed options and results
- `DateTime<Utc>` for all timestamps

## Contributing

Pull requests are welcome. Please open an issue first to discuss major changes.

## License

[MIT](/LICENSE)

## Disclaimer

This is an unofficial API for nyaa. I am not affiliated with nyaa in any way. Use at your own risk.
