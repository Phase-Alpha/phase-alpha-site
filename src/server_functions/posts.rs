use chrono::Utc;
use gray_matter::{engine::TOML, Matter};
use leptos::*;
use pulldown_cmark::{html, Parser};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Result}; // for getting the current date
use std::fs;

// Define a struct for the metadata
#[derive(Debug, Serialize, Deserialize)]
pub struct PostMetadata {
    title: String,
    date: String,
    image_path: String,
}

impl PostMetadata {
    fn create_href(self) -> String {
        let link = self.title.replace(' ', "-").to_lowercase();
        link
    }
}

// Define a struct representing a blog post
#[derive(Debug, sqlx::FromRow)]
pub struct Post {
    title: String,
    content: String,
}

// Read Markdown files from a folder and convert them to a vector of posts
fn read_markdown_files(folder_path: &str) -> Vec<Post> {
    let mut posts = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    for entry in fs::read_dir(folder_path).expect("Error reading folder") {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            let file_content = fs::read_to_string(&file_path).expect("Error reading file");
            let matter = Matter::<TOML>::new();
            let metadata = matter.parse_with_struct::<PostMetadata>(&file_content);
            let parser = Parser::new_ext(&file_content, &options);
            let mut title = String::new();
            let mut content = String::new();
            let mut date = String::new();
            // let mut is_title_found = false;
            // for event in parser {
            //     match event {
            //         pulldown_cmark::Event::Text(text) => {
            //             if !is_title_found {
            //                 title = text.to_string();
            //                 is_title_found = true;
            //             }
            //             content.push_str(&text.to_string());
            //         }
            //         _ => {}
            //     }
            // }
            // Write to String buffer.
            html::push_html(&mut content, parser);
            posts.push(Post { title, content });
        }
    }
    posts
}

// Function to retrieve posts from the database
#[server]
async fn retrieve_posts(pool: &Pool<Postgres>) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>("SELECT id, title, content, date FROM posts")
        .fetch_all(pool)
        .await?;
    Ok(posts)
}
