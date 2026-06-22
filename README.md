<h1 align="center">nyaapi-rs</h1>

This is an unofficial API for nyaa - https://nyaa.si or https://nyaa.land or whatever domain you want to use. This allows you to search for torrents by name, category, or even user. Use at your own risk.

<div align="center">

[![npm](https://img.shields.io/crates/v/nyaapi-rs?style=flat-square)](https://crates.io/crates/nyaapi-rs)
[![npm](https://img.shields.io/crates/d/nyaapi-rs?style=flat-square)](https://crates.io/crates/nyaapi-rs)
![NPM](https://img.shields.io/crates/l/nyaapi-rs)

</div>

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

    println!("{:#?}", result.data);
}
```

## API

### `new Nyaa(options)`

Create a new Nyaa instance.

#### `options`

```js
{
    baseUrl: 'https://nyaa.si', // The base URL of the nyaa instance
    mode: 'html', // 'html' or 'rss'
}
```

### `search(query, options)`

Search for torrents.

#### `query`

Type: `string`

The search query.

#### `options`

```jsonc
{
    "page": 1,
    "category": "all", // all, anime, audio, literature, live-action, pictures, software, games
    "filter": "no filter", // no filter, trusted only, no remakes
    "sort": "date", // date, downloads, size, seeders, leechers, comments
    "order": "desc" // desc, asc
}
```

#### Returns

```typescript
interface SearchResult {
    data: Torrent[];
    total: number | null;
    page: number;
    totalPage: number | null;
    perPage: number;
    nextPage: boolean;
    range: string | null;
    timeTaken: number;
}

interface Torrent {
    id: number;
    name: string;
    magnet: string;
    size: string;
    category: string;
    date: Date;
    seeders: number;
    leechers: number;
    downloads: number;
    viewUrl: string;
    torrentUrl: string;
    comments: number;
}
```

### `searchByUser(username, options)`

Search for torrents by user.

#### `username`

Type: `string`

The username.

#### `options`

```jsonc
{
    "page": 1,
    "category": "all",
    "filter": "no filter",
    "sort": "date",
    "order": "desc",
    "query": "One Piece"
}
```

### `view(id)`

Get torrent details by ID.

#### `id`

Type: `number`

The torrent ID.

#### Returns

```typescript
interface TorrentDetail {
    id: number;
    title: string;
    name: string;
    category: string;
    subCategory: string;
    date: Date;
    seeders: number;
    leechers: number;
    downloads: number;
    completed: number;
    magnet: string;
    size: string;
    hash: string;
    submitter: string;
    submitterId?: string;
    information?: string;
    description: string;
    files: TorrentFile[];
    comments: number;
}

interface TorrentFile {
    name: string;
    size: string;
}
```

### `viewFromTorrent(torrent)`

Get torrent details from a Torrent object.

#### `torrent`

Type: `Torrent`

### `getCategories()`

Get the list of categories.

```typescript
interface Category {
    id: string;
    name: string;
    subCategories?: Category[];
}
```

## RSS Mode

```js
const nyaa = new Nyaa({
    baseUrl: 'https://nyaa.si',
    mode: 'rss',
});

const result = await nyaa.search('One Piece');
```

## License

[MIT ©](/LICENSE)

## Disclaimer

This is an unofficial API for nyaa. I am not affiliated with nyaa in any way. Use at your own risk.

## Contributing

Pull requests are welcome.
