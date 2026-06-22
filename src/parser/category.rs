use crate::types::Category;
use scraper::{Html, Selector};

pub fn parse_categories(html: &str) -> Vec<Category> {
    let document = Html::parse_document(html);
    let sel = Selector::parse("select[name=\"c\"] option").unwrap();
    let mut categories = Vec::new();
    let mut seen_ids = std::collections::HashSet::new();

    for el in document.select(&sel) {
        let value = match el.attr("value") {
            Some(v) => v,
            None => continue,
        };
        let text = el.text().collect::<String>().trim().to_string();
        let value_str = value.to_string();

        if value_str == "0_0" || text.is_empty() || seen_ids.contains(&value_str) {
            continue;
        }
        seen_ids.insert(value_str.clone());

        let is_sub = text.starts_with("- ");
        let name = if is_sub { text[2..].to_string() } else { text };

        if !is_sub {
            categories.push(Category {
                id: value_str,
                name,
                sub_categories: Vec::new(),
            });
        } else {
            let parent_idx = categories.len() - 1;
            if let Some(parent) = categories.get_mut(parent_idx) {
                parent.sub_categories.push(Category {
                    id: value_str,
                    name,
                    sub_categories: Vec::new(),
                });
            }
        }
    }

    categories
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_categories() {
        let html = r#"
        <html><body>
        <select name="c">
            <option value="0_0">All categories</option>
            <option value="1_0">Anime</option>
            <option value="1_1">- Anime Music Video</option>
            <option value="1_2">- English-translated</option>
            <option value="2_0">Audio</option>
            <option value="3_0">Literature</option>
        </select>
        </body></html>
        "#;

        let cats = parse_categories(html);
        assert_eq!(cats.len(), 3);
        assert_eq!(cats[0].id, "1_0");
        assert_eq!(cats[0].name, "Anime");
        assert_eq!(cats[0].sub_categories.len(), 2);
        assert_eq!(cats[0].sub_categories[0].id, "1_1");
        assert_eq!(cats[0].sub_categories[0].name, "Anime Music Video");
        assert_eq!(cats[1].id, "2_0");
        assert_eq!(cats[1].name, "Audio");
        assert_eq!(cats[2].id, "3_0");
        assert_eq!(cats[2].name, "Literature");
    }
}
