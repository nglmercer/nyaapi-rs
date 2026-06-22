use crate::types::{TorrentDetail, TorrentFile};
use chrono::DateTime;
use scraper::{Html, Selector};

pub fn parse_view_page(html: &str, id: u64) -> Option<TorrentDetail> {
    let document = Html::parse_document(html);

    let title_sel = Selector::parse(".panel h3").ok()?;
    let title = document
        .select(&title_sel)
        .next()?
        .text()
        .collect::<String>()
        .trim()
        .to_string();
    if title.is_empty() {
        return None;
    }

    let clean_title = title.split('\n').next()?.trim().to_string();

    let row_sel = Selector::parse(".panel-body .row").unwrap();
    let col_sel = Selector::parse(".col-md-5").unwrap();
    let col_a_sel = Selector::parse(".col-md-5 a").unwrap();

    let get_row_text = |label: &str| -> Option<String> {
        document.select(&row_sel).find_map(|el| {
            if el.text().collect::<String>().contains(label) {
                el.select(&col_sel)
                    .next()
                    .map(|e| e.text().collect::<String>().trim().to_string())
            } else {
                None
            }
        })
    };

    let get_row_link = |label: &str| -> Option<String> {
        document.select(&row_sel).find_map(|el| {
            if el.text().collect::<String>().contains(label) {
                el.select(&col_a_sel)
                    .next()
                    .and_then(|a| a.attr("href").map(|s| s.to_string()))
            } else {
                None
            }
        })
    };

    let parse_category = || {
        let cat_text = get_row_text("Category:")?;
        let parts: Vec<&str> = cat_text.split(" - ").collect();
        let cat = parts.first()?.trim().to_string();
        let sub_cat = parts.get(1)?.trim().to_string();
        Some((cat, sub_cat))
    };

    let (category, sub_category) = parse_category().unwrap_or_default();

    let ts_sel = Selector::parse("[data-timestamp]").unwrap();
    let ts_str = document
        .select(&ts_sel)
        .next()?
        .attr("data-timestamp")
        .unwrap_or("0");
    let ts = ts_str.parse::<i64>().ok()?;
    let date = DateTime::from_timestamp(ts, 0)?;

    let submitter = get_row_text("Submitter:");
    let submitter_link_sel = Selector::parse("a.text-default").unwrap();
    let submitter_id = document
        .select(&submitter_link_sel)
        .next()
        .and_then(|a| a.attr("href"))
        .map(|s| {
            s.trim_start_matches("/user/")
                .trim_end_matches('/')
                .to_string()
        });

    let information = get_row_link("Information:");
    let file_size = get_row_text("File size:").unwrap_or_default();
    let completed = get_row_text("Completed:").and_then(|s| s.parse().ok());

    let green_sel = Selector::parse("[style*=\"color: green\"]").unwrap();
    let seeders = document
        .select(&green_sel)
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse().ok())
        .unwrap_or(0);

    let red_sel = Selector::parse("[style*=\"color: red\"]").unwrap();
    let leechers = document
        .select(&red_sel)
        .next()
        .and_then(|e| e.text().collect::<String>().trim().parse().ok())
        .unwrap_or(0);

    let hash_sel = Selector::parse("kbd").unwrap();
    let hash = document
        .select(&hash_sel)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string());

    let magnet_sel = Selector::parse("a[href^=\"magnet:\"]").unwrap();
    let magnet = document
        .select(&magnet_sel)
        .next()
        .and_then(|a| a.attr("href").map(|s| s.to_string()));

    let magnet_url_sel = Selector::parse("a[href^=\"magnet?\"]").unwrap();
    let magnet_url = document
        .select(&magnet_url_sel)
        .next()
        .and_then(|a| a.attr("href").map(|s| s.to_string()));

    let torrent_url_sel = Selector::parse("a[href$=\".torrent\"]").unwrap();
    let torrent_url = document
        .select(&torrent_url_sel)
        .next()
        .and_then(|a| a.attr("href").map(|s| s.to_string()));

    let download_link = if let Some(m) = &magnet {
        m.clone()
    } else if let Some(mu) = &magnet_url {
        let query = mu.split('?').nth(1).unwrap_or_default();
        format!("magnet:?{}", query)
    } else {
        torrent_url.unwrap_or_default()
    };

    let description_sel = Selector::parse("#torrent-description").unwrap();
    let description = document
        .select(&description_sel)
        .next()
        .map(|e| e.inner_html())
        .unwrap_or_default();

    let mut files = Vec::new();
    let file_list_sel = Selector::parse(".torrent-file-list li").unwrap();
    for li in document.select(&file_list_sel) {
        let mut name = String::new();
        for child in li.children() {
            if let Some(text) = child.value().as_text() {
                name.push_str(text.trim());
            }
        }
        let name = name.trim().to_string();
        let size = li
            .select(&Selector::parse(".file-size").unwrap())
            .next()
            .map(|e| e.text().collect::<String>().trim().to_string())
            .unwrap_or_default();
        if !name.is_empty() {
            files.push(TorrentFile { name, size });
        }
    }

    let comments_heading_sel =
        Selector::parse(".panel-heading a[data-toggle=\"collapse\"]").unwrap();
    let comments_text = document
        .select(&comments_heading_sel)
        .next()
        .map(|e| e.text().collect::<String>().trim().to_string())
        .unwrap_or_default();

    let comments = if let Some(idx) = comments_text.find("Comments") {
        let rest = &comments_text[idx..];
        if let Some(dash_idx) = rest.find('-') {
            let after_dash = rest[dash_idx + 1..].trim_start();
            after_dash
                .split_whitespace()
                .next()?
                .parse()
                .ok()
                .unwrap_or(0)
        } else {
            0
        }
    } else {
        0
    };

    Some(TorrentDetail {
        id,
        title: clean_title.clone(),
        name: clean_title,
        category,
        sub_category,
        date,
        seeders,
        leechers,
        downloads: completed.unwrap_or(0),
        completed,
        magnet: download_link,
        size: file_size,
        hash,
        submitter,
        submitter_id,
        information,
        description,
        files,
        comments,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_view_page_minimal() {
        let html = r#"
        <html><body>
        <div class="panel">
        <h3>Test Torrent Title
</h3>
        <div class="panel-body">
            <div class="row">
                <div>Category:</div>
                <div class="col-md-5">Anime - English-translated</div>
            </div>
            <div class="row">
                <div>Submitter:</div>
                <div class="col-md-5">testuser</div>
            </div>
            <div class="row">
                <div>File size:</div>
                <div class="col-md-5">1.5 GiB</div>
            </div>
            <div class="row">
                <div>Completed:</div>
                <div class="col-md-5">100</div>
            </div>
        </div>
        </div>
        <div data-timestamp="1719000000"></div>
        <span style="color: green">50</span>
        <span style="color: red">5</span>
        <kbd>abc123def456</kbd>
        <a href="magnet:?xt=urn:btih:abc123">magnet</a>
        <div id="torrent-description">&lt;p&gt;Test desc&lt;/p&gt;</div>
        <ul class="torrent-file-list">
            <li>episode01.mp4 <span class="file-size">300 MiB</span></li>
            <li>episode02.mp4 <span class="file-size">285 MiB</span></li>
        </ul>
        <div class="panel-heading">
            <a data-toggle="collapse" href="comments">Comments - 5</a>
        </div>
        </body></html>
        "#;

        let detail = parse_view_page(html, 99999);
        assert!(detail.is_some());
        let d = detail.unwrap();
        assert_eq!(d.id, 99999);
        assert_eq!(d.title, "Test Torrent Title");
        assert_eq!(d.category, "Anime");
        assert_eq!(d.sub_category, "English-translated");
        assert_eq!(d.size, "1.5 GiB");
        assert_eq!(d.seeders, 50);
        assert_eq!(d.leechers, 5);
        assert_eq!(d.completed, Some(100));
        assert_eq!(d.files.len(), 2);
        assert_eq!(d.files[0].name, "episode01.mp4");
        assert_eq!(d.comments, 5);
    }

    #[test]
    fn test_parse_view_page_no_title() {
        let html = r#"<html><body><div class="panel"><h3></h3></div></body></html>"#;
        assert!(parse_view_page(html, 1).is_none());
    }
}
