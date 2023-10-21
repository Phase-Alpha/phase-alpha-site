#[allow(unused)]
use chrono::Utc;
use gray_matter::{engine::YAML, Matter};
use leptos::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;

// Define a struct for the metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PostMetadata {
    title: String,
    description: String,
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
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Post {
    meta_data: PostMetadata,
    content: String,
}

// Read Markdown files from a folder and convert them to a vector of posts
#[server(GetPosts, "/api")]
pub async fn get_posts(folder_path: String) -> Result<Vec<Post>, ServerFnError> {
    let posts = read_markdown_files(folder_path);
    Ok(posts)
}

pub fn read_markdown_files(folder_path: &str) -> Vec<Post> {
    let mut posts = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    for entry in fs::read_dir(folder_path).expect("Error reading folder") {
        if let Ok(entry) = entry {
            let file_path = entry.path();
            let file_content = fs::read_to_string(&file_path).expect("Error reading file");
            let matter = Matter::<YAML>::new();
            let metadata = matter
                .parse_with_struct::<PostMetadata>(&file_content)
                .expect("Failed to parse front matter");
            let content = metadata.content;
            let parser = Parser::new_ext(&content, options);
            let mut html_content = String::new();
            html::push_html(&mut html_content, parser);
            posts.push(Post {
                meta_data: metadata.data,
                content: html_content,
            });
        }
    }
    posts
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_POSTS_PATH: &str = "./test-posts/";
    const INCORRECT_PATH: &str = "./error-path/";
    #[test]
    fn test_read_pass() {
        let posts = read_markdown_files(&TEST_POSTS_PATH);
        let expected = vec![Post {
            meta_data: PostMetadata {
                title: String::from("Test Post"),
                description: String::from("Some testing"),
                date: String::from("2023-10-15"),
                image_path: String::from("./public/pic01.jpg"),
            },
            content: String::from(
                "<h1>Test</h1>\n<p>Post content</p>\n<p><img src=\"./public/pic01.jpg\" alt=\"pic\" /></p>\n",
            ),
        }];
        assert_eq!(expected, posts)
    }

    #[test]
    #[should_panic]
    fn test_read_incorrect_dir() {
        let posts = read_markdown_files(&INCORRECT_PATH);
    }
}
