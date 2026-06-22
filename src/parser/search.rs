use crate::types::{PaginationInfo, Torrent};
use chrono::{DateTime, Utc};
use quick_xml::events::Event;
use quick_xml::Reader;
use scraper::{Html, Selector};

pub fn parse_search_results(html: &str) -> Vec<Torrent> {
    let document = Html::parse_document(html);
    let sel = Selector::parse("tr.default").unwrap();
    let mut torrents = Vec::new();

    for row in document.select(&sel) {
        if let Some(torrent) = parse_torrent_row(&row) {
            torrents.push(torrent);
        }
    }

    torrents
}

fn parse_torrent_row(row: &scraper::ElementRef) -> Option<Torrent> {
    let id_link_sel = Selector::parse("td:nth-child(2) > a").unwrap();
    let id_link = row.select(&id_link_sel).next()?.attr("href")?.to_string();
    let view_id = id_link
        .trim_start_matches("/view/")
        .trim_end_matches('#')
        .to_string();
    let id = view_id.parse().ok()?;

    let name = row
        .select(&id_link_sel)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let magnet_sel = Selector::parse("td:nth-child(3) a:nth-child(2)").unwrap();
    let magnet = row
        .select(&magnet_sel)
        .next()
        .and_then(|a| a.attr("href").map(|s| s.to_string()))
        .unwrap_or_default();

    let size_sel = Selector::parse("td:nth-child(4)").unwrap();
    let size = row
        .select(&size_sel)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let cat_sel = Selector::parse("td:nth-child(1) > a").unwrap();
    let category = row
        .select(&cat_sel)
        .next()
        .and_then(|a| a.attr("title").map(|s| s.to_string()))
        .unwrap_or_default();

    let ts_sel = Selector::parse("td:nth-child(5)").unwrap();
    let timestamp = row
        .select(&ts_sel)
        .next()?
        .attr("data-timestamp")
        .and_then(|s| s.parse::<i64>().ok())?;

    let date = DateTime::from_timestamp(timestamp, 0)?;

    let seeders = row
        .select(&Selector::parse("td:nth-child(6)").unwrap())
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .parse()
        .ok()
        .unwrap_or(0);
    let leechers = row
        .select(&Selector::parse("td:nth-child(7)").unwrap())
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .parse()
        .ok()
        .unwrap_or(0);
    let downloads = row
        .select(&Selector::parse("td:nth-child(8)").unwrap())
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .parse()
        .ok()
        .unwrap_or(0);

    let comment_sel = Selector::parse("td:nth-child(2) > a.comments").unwrap();
    let comments_title = row
        .select(&comment_sel)
        .next()
        .and_then(|a| a.attr("title"))
        .unwrap_or_default();
    let comments = comments_title
        .split_whitespace()
        .find_map(|s| s.parse().ok())
        .unwrap_or(0);

    Some(Torrent {
        id,
        name,
        magnet,
        size,
        category,
        sub_category: None,
        date,
        seeders,
        leechers,
        downloads,
        hash: None,
        submitter: None,
        submitter_id: None,
        information: None,
        completed: None,
        description: None,
        torrent_url: Some(format!("/download/{}.torrent", view_id)),
        view_url: Some(format!("/view/{}", view_id)),
        comments: Some(comments as u64),
    })
}

pub fn parse_search_results_rss(xml: &str) -> Vec<Torrent> {
    use std::collections::HashMap;

    let mut reader = Reader::from_str(xml);
    reader.config_mut().trim_text(true);
    let mut buf = Vec::new();
    let mut items = Vec::new();
    let mut in_item = false;
    let mut current: HashMap<String, String> = HashMap::new();
    let mut current_tag = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let tag = tag.strip_prefix("nyaa:").unwrap_or(&tag).to_string();
                if tag == "item" {
                    in_item = true;
                    current.clear();
                }
                if in_item {
                    current_tag = tag;
                }
            }
            Ok(Event::Text(e)) => {
                if in_item && !current_tag.is_empty() {
                    let text = e.unescape().unwrap_or_default().to_string();
                    current.insert(current_tag.clone(), text);
                }
            }
            Ok(Event::End(e)) => {
                let tag = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let tag = tag.strip_prefix("nyaa:").unwrap_or(&tag).to_string();
                if in_item && tag == "item" {
                    items.push(torrent_from_map(&current));
                    in_item = false;
                }
                current_tag.clear();
            }
            Ok(Event::Eof) => break,
            Err(_) => {}
            _ => {}
        }
        buf.clear();
    }

    items
}

fn strip_domain(url: &str) -> String {
    if let Some(idx) = url.find("://") {
        let after_prot = &url[idx + 3..];
        if let Some(slash_idx) = after_prot.find('/') {
            return after_prot[slash_idx..].to_string();
        }
    }
    url.to_string()
}

fn torrent_from_map(map: &std::collections::HashMap<String, String>) -> Torrent {
    let guid = map.get("guid").cloned().unwrap_or_default();
    let view_url = strip_domain(&guid);
    let id = view_url
        .trim_start_matches("/view/")
        .trim_end_matches('#')
        .parse()
        .unwrap_or(0);

    let name = map.get("title").cloned().unwrap_or_default();
    let date_str = map.get("pubDate").cloned().unwrap_or_default();
    let date = DateTime::parse_from_rfc2822(&date_str)
        .map(|dt| dt.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let hash = map.get("infoHash").cloned();
    let category = map.get("category").cloned().unwrap_or_default();
    let category_id = map.get("categoryId").cloned();
    let size = map.get("size").cloned().unwrap_or_default();
    let link = map.get("link").cloned().unwrap_or_default();

    let torrent_url = if link.is_empty() {
        format!("/download/{}.torrent", id)
    } else {
        link
    };

    let magnet = hash
        .as_ref()
        .map(|h| {
            format!(
                "magnet:?xt=urn:btih:{}&dn={}",
                h,
                percent_encoding::percent_encode(
                    name.as_bytes(),
                    percent_encoding::NON_ALPHANUMERIC
                )
            )
        })
        .unwrap_or_default();

    Torrent {
        id,
        name,
        magnet,
        size,
        category: category_id.unwrap_or(category),
        sub_category: None,
        date,
        seeders: map.get("seeders").and_then(|s| s.parse().ok()).unwrap_or(0),
        leechers: map
            .get("leechers")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        downloads: map
            .get("downloads")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0),
        hash,
        comments: map.get("comments").and_then(|s| s.parse().ok()),
        torrent_url: Some(torrent_url),
        view_url: Some(view_url),
        submitter: None,
        submitter_id: None,
        information: None,
        completed: None,
        description: None,
    }
}

pub fn parse_pagination(html: &str) -> Option<PaginationInfo> {
    let document = Html::parse_document(html);

    let last_page_sel = Selector::parse(".pagination > li:nth-last-child(2)").ok()?;
    let total_page = document
        .select(&last_page_sel)
        .next()?
        .text()
        .collect::<String>()
        .replace(",", "")
        .trim()
        .parse::<u64>()
        .ok();

    let page_info_sel = Selector::parse(".pagination-page-info").ok()?;
    let page_info_text = document
        .select(&page_info_sel)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();

    let mut total = None;
    let mut range = None;

    if !page_info_text.is_empty() {
        if let Some(idx) = page_info_text.find(" results") {
            total = page_info_text[..idx]
                .split_whitespace()
                .last()
                .and_then(|s| s.parse().ok());
        }

        if let Some(dash_idx) = page_info_text.find(" - ") {
            let start_str = page_info_text[..dash_idx].trim();
            let rest = &page_info_text[dash_idx + 3..];
            if let Some(space_idx) = rest.find(' ') {
                let end_str = rest[..space_idx].trim();
                range = Some(format!("{}-{}", start_str, end_str));
            } else {
                range = Some(format!("{}-{}", start_str, rest.trim()));
            }
        } else if let Some(dash_idx) = page_info_text.find('-') {
            let start_str = page_info_text[..dash_idx].trim();
            let end_str = page_info_text[dash_idx + 1..].trim();
            range = Some(format!("{}-{}", start_str, end_str));
        }
    }

    let next_page_sel = Selector::parse(".pagination > li:last-child > a").ok()?;
    let next_page = document.select(&next_page_sel).next().is_some();

    Some(PaginationInfo {
        total,
        total_page,
        page: 0,
        per_page: 0,
        range,
        next_page,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_search_results_minimal() {
        let html = r#"
        <html><body>
        <table><tbody>
        <tr class="default">
            <td><a title="Anime"><img src="/static/img/icons/anime.png"/></a></td>
            <td><a href="/view/12345">One Piece</a> <a class="comments" title="12 comments">12 comments</a></td>
            <td><a href="magnet:?xt=urn:btih:abc123"><img src="static/img/icons/magnet.png"/></a></td>
            <td>1.2 GiB</td>
            <td data-timestamp="1719000000"></td>
            <td>150</td>
            <td>30</td>
            <td>5000</td>
        </tr>
        </tbody></table>
        </body></html>
        "#;

        let results = parse_search_results(html);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 12345);
        assert_eq!(results[0].name, "One Piece");
        assert_eq!(results[0].size, "1.2 GiB");
        assert_eq!(results[0].seeders, 150);
        assert_eq!(results[0].leechers, 30);
        assert_eq!(results[0].downloads, 5000);
        assert_eq!(results[0].comments, Some(12));
        assert_eq!(results[0].view_url, Some("/view/12345".to_string()));
    }

    #[test]
    fn test_parse_pagination() {
        let html = r#"
        <ul class="pagination">
            <li><a href="?p=1">1</a></li>
            <li><a href="?p=2">2</a></li>
            <li>5</li>
            <li><a href="?p=6"><i class="fa fa-chevron-right"></i></a></li>
        </ul>
        <div class="pagination-page-info">1 - 75 of 372 results</div>
        "#;

        let page = parse_pagination(html);
        assert!(page.is_some());
        let p = page.unwrap();
        assert_eq!(p.total, Some(372));
        assert_eq!(p.total_page, Some(5));
        assert_eq!(p.range, Some("1-75".to_string()));
        assert!(p.next_page);
    }

    #[test]
    fn test_parse_search_results_rss_minimal() {
        let xml = r#"<?xml version="1.0" encoding="UTF-8"?>
        <rss version="2.0" xmlns:nyaa="https://nyaa.si/">
        <channel>
        <item>
            <title>Test Anime Torrent</title>
            <link>https://nyaa.si/download/99999.torrent</link>
            <guid>https://nyaa.si/view/99999</guid>
            <pubDate>Mon, 22 Jun 2026 10:00:00 +0000</pubDate>
            <nyaa:infoHash>abc123def456</nyaa:infoHash>
            <nyaa:categoryId>1_2</nyaa:categoryId>
            <nyaa:category>Anime - English-translated</nyaa:category>
            <nyaa:size>1.5 GiB</nyaa:size>
            <nyaa:seeders>100</nyaa:seeders>
            <nyaa:leechers>10</nyaa:leechers>
            <nyaa:downloads>500</nyaa:downloads>
            <nyaa:comments>3</nyaa:comments>
        </item>
        </channel>
        </rss>
        "#;

        let results = parse_search_results_rss(xml);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 99999);
        assert_eq!(results[0].name, "Test Anime Torrent");
        assert_eq!(results[0].size, "1.5 GiB");
        assert_eq!(results[0].seeders, 100);
        assert_eq!(results[0].leechers, 10);
        assert_eq!(results[0].downloads, 500);
        assert_eq!(results[0].comments, Some(3));
        assert!(results[0].magnet.contains("abc123def456"));
    }
}
