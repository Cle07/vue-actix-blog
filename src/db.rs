use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub id: Option<i64>,
    pub name: String,
    pub content: String,
    pub links: Vec<String>,
}

impl Article {
    pub fn new(name: String, content: String, available_articles: &[String]) -> Self {
        let raw_links = extract_obsidian_links(&content);
        println!("Article '{}' - Raw links extracted: {:?}", name, raw_links);
        println!("Available articles: {:?}", available_articles);
        let links = raw_links
            .into_iter()
            .filter(|link| available_articles.contains(link))
            .collect();
        println!("Article '{}' - Filtered links: {:?}", name, links);
        Self {
            id: None,
            name,
            content,
            links,
        }
    }

    pub fn get_from_name(name: &str) -> Option<Self> {
        // Open the database connection
        let connection = sqlite::open("articles.db")
            .expect("Database should already has been created at that point.");

        let mut article = None;

        // Prepare the query with proper parameter binding
        let mut statement = connection
            .prepare("SELECT id, name, content, links FROM articles WHERE name = ?")
            .expect("Failed to prepare statement");

        // Bind the name parameter
        statement.bind((1, name)).expect("Failed to bind parameter");

        // Execute the query
        if statement.next().expect("Failed to execute query") == sqlite::State::Row {
            // Extract data from the row
            let id = statement.read::<i64, _>(0).ok();
            let article_name = statement.read::<String, _>(1).expect("Failed to read name");
            let content = statement
                .read::<String, _>(2)
                .expect("Failed to read content");

            // Parse the links JSON
            let links_json = statement
                .read::<String, _>(3)
                .expect("Failed to read links");
            let links =
                serde_json::from_str::<Vec<String>>(&links_json).unwrap_or_else(|_| Vec::new());

            article = Some(Self {
                id,
                name: article_name,
                content,
                links,
            });
        }

        article
    }
}

// Extract Obsidian-style links from markdown content
pub fn extract_obsidian_links(content: &str) -> Vec<String> {
    let link_pattern = Regex::new(r"\[\[(.*?)\]\]").unwrap();
    link_pattern
        .captures_iter(content)
        .map(|cap| {
            let link_text = cap[1].trim();
            // Handle [[text|alias]] format - take only the text before the pipe
            if let Some(pipe_pos) = link_text.find('|') {
                link_text[..pipe_pos].trim().to_string()
            } else {
                link_text.to_string()
            }
        })
        .collect()
}

pub fn init_db() -> sqlite::Result<()> {
    println!("Initializing database at articles.db...");
    let connection = sqlite::open("articles.db")?;

    println!("Database connection established");
    connection.execute("DROP TABLE IF EXISTS articles;")?;
    println!("Cleaned up existing tables");

    connection.execute(
        "
        CREATE TABLE IF NOT EXISTS articles (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL UNIQUE,
            content TEXT NOT NULL,
            links TEXT
        );
        ",
    )?;
    println!("Table schema created/verified");

    let articles_path = std::path::Path::new("articles");
    if articles_path.exists() && articles_path.is_dir() {
        // First pass: collect all article names
        let mut article_names = Vec::new();
        let mut articles_data = Vec::new();

        if let Ok(entries) = std::fs::read_dir(articles_path) {
            for entry in entries.filter_map(|e| e.ok()) {
                if entry
                    .path()
                    .extension()
                    .and_then(|ext| ext.to_str())
                    .map(|ext| ext == "md")
                    .unwrap_or(false)
                {
                    if let Some(file_name) = entry.path().file_stem().and_then(|s| s.to_str()) {
                        let file_path = format!("articles/{}.md", file_name);
                        match std::fs::read_to_string(&file_path) {
                            Ok(raw_content) => {
                                println!("Found article: {}", file_name);
                                article_names.push(file_name.to_string());
                                articles_data.push((file_name.to_string(), raw_content));
                            }
                            Err(e) => println!("Error reading file {}: {}", file_path, e),
                        }
                    }
                }
            }
        }

        // Second pass: create articles with resolved links
        for (name, content) in articles_data {
            let article = Article::new(name.clone(), content, &article_names);
            let links_json = serde_json::to_string(&article.links).unwrap_or_default();

            // Check if article already exists and delete it if it does
            let delete_query = format!("DELETE FROM articles WHERE name = '{}'", name);
            if let Err(e) = connection.execute(&delete_query) {
                eprintln!("Failed to delete existing article '{}': {}", name, e);
            }

            println!("Inserting article: {}", name);
            let mut statement = connection
                .prepare("INSERT INTO articles (name, content, links) VALUES (?, ?, ?)")?;
            if let Err(e) = statement
                .bind((1, article.name.as_str()))
                .and_then(|_| statement.bind((2, article.content.as_str())))
                .and_then(|_| statement.bind((3, links_json.as_str())))
                .and_then(|_| statement.next())
            {
                eprintln!("Failed to insert article '{}': {}", article.name, e);
            } else {
                println!("Article '{}' inserted successfully", name);
            }
        }
    }

    // Print all articles in the table for debugging
    println!("=== All Articles in Database ===");

    // Check if there are any articles in the database
    let count_query = "SELECT COUNT(*) as count FROM articles";
    let mut has_articles = false;

    println!("Counting articles in database...");
    match connection.iterate(count_query, |pairs| {
        if let Some((_, Some(count_str))) = pairs.first() {
            if let Ok(count) = count_str.parse::<i64>() {
                has_articles = count > 0;
                println!("Found {} articles in the database", count);
            }
        }
        true
    }) {
        Ok(_) => {}
        Err(e) => println!("Error counting articles: {}", e),
    };

    if has_articles {
        let query = "SELECT id, name, links FROM articles";
        println!("Listing all articles:");
        match connection.iterate(query, |pairs| {
            for &(column, value) in pairs.iter() {
                print!("{}: {:?}, ", column, value.unwrap_or("NULL"));
            }
            println!();
            true
        }) {
            Ok(_) => {}
            Err(e) => println!("Error listing articles: {}", e),
        };
    } else {
        println!(
            "No articles found. Check that the '/articles' directory exists and contains markdown files."
        );
    }

    println!("=== End of Articles ===");

    Ok(())
}
