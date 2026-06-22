use clap::{Parser, Subcommand};
use nyaa_api::{Nyaa, NyaaMode, NyaaOptions};

#[derive(Parser)]
#[command(name = "nyaa-cli")]
#[command(about = "CLI for nyaa.si / nyaa.land torrent search", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(long, default_value = "https://nyaa.si")]
    base_url: String,

    #[arg(long, default_value = "html")]
    mode: String,
}

#[derive(Subcommand)]
enum Commands {
    Search {
        query: String,

        #[arg(long)]
        category: Option<String>,

        #[arg(long)]
        filter: Option<String>,

        #[arg(long)]
        sort: Option<String>,

        #[arg(long)]
        order: Option<String>,

        #[arg(long, default_value_t = 1)]
        page: u64,
    },
    SearchByUser {
        username: String,

        #[arg(long)]
        query: Option<String>,

        #[arg(long)]
        category: Option<String>,

        #[arg(long)]
        filter: Option<String>,

        #[arg(long)]
        sort: Option<String>,

        #[arg(long)]
        order: Option<String>,

        #[arg(long, default_value_t = 1)]
        page: u64,
    },
    View {
        id: u64,
    },
    Categories,
}

fn parse_mode(mode: &str) -> NyaaMode {
    match mode {
        "rss" => NyaaMode::Rss,
        _ => NyaaMode::Html,
    }
}

fn to_category(s: Option<&str>) -> Option<nyaa_api::CategoryFilter> {
    use nyaa_api::CategoryFilter;
    s.and_then(|v| match v {
        "anime" => Some(CategoryFilter::Anime),
        "audio" => Some(CategoryFilter::Audio),
        "literature" => Some(CategoryFilter::Literature),
        "live-action" => Some(CategoryFilter::LiveAction),
        "pictures" => Some(CategoryFilter::Pictures),
        "software" => Some(CategoryFilter::Software),
        "games" => Some(CategoryFilter::Games),
        "all" => Some(CategoryFilter::All),
        _ => None,
    })
}

fn to_filter(s: Option<&str>) -> Option<nyaa_api::TrustedFilter> {
    use nyaa_api::TrustedFilter;
    s.and_then(|v| match v {
        "no-filter" | "no filter" => Some(TrustedFilter::NoFilter),
        "trusted-only" | "trusted only" => Some(TrustedFilter::TrustedOnly),
        "no-remakes" | "no remakes" => Some(TrustedFilter::NoRemakes),
        _ => None,
    })
}

fn to_sort(s: Option<&str>) -> Option<nyaa_api::SortBy> {
    use nyaa_api::SortBy;
    s.and_then(|v| match v {
        "comments" => Some(SortBy::Comments),
        "size" => Some(SortBy::Size),
        "date" => Some(SortBy::Date),
        "seeders" => Some(SortBy::Seeders),
        "leechers" => Some(SortBy::Leechers),
        "downloads" => Some(SortBy::Downloads),
        _ => None,
    })
}

fn to_order(s: Option<&str>) -> Option<nyaa_api::Order> {
    use nyaa_api::Order;
    s.and_then(|v| match v {
        "asc" => Some(Order::Asc),
        "desc" => Some(Order::Desc),
        _ => None,
    })
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let nyaa = Nyaa::new(NyaaOptions {
        base_url: cli.base_url,
        mode: parse_mode(&cli.mode),
    });

    match cli.command {
        Commands::Search {
            query,
            category,
            filter,
            sort,
            order,
            page,
        } => {
            let opts = nyaa_api::SearchOptions {
                page: Some(page),
                category: to_category(category.as_deref()),
                filter: to_filter(filter.as_deref()),
                sort: to_sort(sort.as_deref()),
                order: to_order(order.as_deref()),
            };
            match nyaa.search(query, opts).await {
                Ok(search) => {
                    println!("Page: {}", search.page);
                    if let Some(total) = search.total {
                        println!("Total: {}", total);
                    }
                    if let Some(range) = &search.range {
                        println!("Range: {}", range);
                    }
                    println!("Time: {}ms", search.time_taken);
                    for t in search.data {
                        println!(
                            "[{}] {} | {} | S:{} L:{} | {}",
                            t.id, t.name, t.size, t.seeders, t.leechers, t.date
                        );
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::SearchByUser {
            username,
            query,
            category,
            filter,
            sort,
            order,
            page,
        } => {
            let opts = nyaa_api::SearchByUserOptions {
                page: Some(page),
                category: to_category(category.as_deref()),
                filter: to_filter(filter.as_deref()),
                sort: to_sort(sort.as_deref()),
                order: to_order(order.as_deref()),
                query,
            };
            match nyaa.search_by_user(username, opts).await {
                Ok(torrents) => {
                    println!("Found {} torrents", torrents.len());
                    for t in torrents {
                        println!(
                            "[{}] {} | {} | S:{} L:{} | {}",
                            t.id, t.name, t.size, t.seeders, t.leechers, t.date
                        );
                    }
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::View { id } => match nyaa.view(id).await {
            Ok(Some(detail)) => {
                println!("Title: {}", detail.title);
                println!("Category: {} - {}", detail.category, detail.sub_category);
                println!("Size: {}", detail.size);
                println!("Date: {}", detail.date);
                println!(
                    "Seeders: {} | Leechers: {}",
                    detail.seeders, detail.leechers
                );
                println!("Downloads: {}", detail.downloads);
                if let Some(c) = detail.completed {
                    println!("Completed: {}", c);
                }
                println!("Magnet: {}", detail.magnet);
                if let Some(s) = detail.submitter {
                    println!("Submitter: {}", s);
                }
                if let Some(info) = detail.information {
                    println!("Information: {}", info);
                }
                if !detail.description.is_empty() {
                    println!("Description: {}", detail.description);
                }
                if !detail.files.is_empty() {
                    println!("Files:");
                    for f in &detail.files {
                        println!("  {} ({})", f.name, f.size);
                    }
                }
                println!("Comments: {}", detail.comments);
            }
            Ok(None) => {
                eprintln!("Torrent not found.");
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        },
        Commands::Categories => match nyaa.get_categories().await {
            Ok(cats) => {
                for cat in cats {
                    println!("{} ({})", cat.name, cat.id);
                    for sub in cat.sub_categories {
                        println!("  - {} ({})", sub.name, sub.id);
                    }
                }
            }
            Err(e) => eprintln!("Error: {}", e),
        },
    }
}
