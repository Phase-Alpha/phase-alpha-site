#[allow(unused)]
use chrono::NaiveDate;
use gray_matter::{engine::YAML, Matter};
use leptos::prelude::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::fs;

// Define a struct for the metadata
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PostMetadata {
    pub title: String,
    pub description: String,
    pub date: String,
    pub image_path: String,
}

impl PostMetadata {
    pub fn create_href(&self) -> String {
        self.title.replace(' ', "-").to_lowercase()
    }
}

// Define a struct representing a blog post
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Post {
    pub meta_data: PostMetadata,
    pub content: String,
}

#[server(GetPostBySlug, "/api")]
pub async fn get_post_by_slug(folder_path: String, slug: String) -> Result<Option<Post>, ServerFnError> {
    let posts = read_markdown_files(folder_path);
    let posts = order_posts(posts);
    
    let post = posts.into_iter()
        .find(|p| p.meta_data.create_href() == slug);
    
    Ok(post)
}


// Read Markdown files from a folder and convert them to a vector of posts
#[server(GetPosts, "/api")]
pub async fn get_posts(folder_path: String) -> Result<Vec<Post>, ServerFnError> {
    let posts = read_markdown_files(folder_path);
    Ok(order_posts(posts))
}

pub fn read_markdown_files(folder_path: String) -> Vec<Post> {
    let mut posts = Vec::new();
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    for entry in fs::read_dir(folder_path).expect("Error reading folder").flatten() {
        let file_path = entry.path();
        let mut file_content: String = String::new();
        if file_path.is_file() && file_path.extension().is_some_and(|ext| ext == "md") {
            file_content = fs::read_to_string(&file_path).expect("Error reading file");
        }
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
    posts
}

pub fn order_posts(mut posts: Vec<Post>) -> Vec<Post> {
    posts.sort_by(|a, b| {
        let date_a = NaiveDate::parse_from_str(&a.meta_data.date, "%Y-%m-%d").unwrap();
        let date_b = NaiveDate::parse_from_str(&b.meta_data.date, "%Y-%m-%d").unwrap();
        date_b.cmp(&date_a)
    });
    posts
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_POSTS_PATH: &str = "test-posts/";
    const INCORRECT_PATH: &str = "./error-path/";

    #[test]
    fn test_read_pass() {
        let posts = read_markdown_files(TEST_POSTS_PATH.to_string());
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
    fn test_sort_post() {
        let posts = vec![
            Post {
                
                meta_data: PostMetadata {
                                title: String::from("Test Post 1"),
                                description: String::from("Some more testing"),
                                date: String::from("2022-10-15"),
                                image_path: String::from("./public/pic01.jpg"),
                },
                content: String::from(
                    "<h1>Test</h1>\n<p>Post content</p>\n<p><img src=\"./public/pic01.jpg\" alt=\"pic\" /></p>\n",
                ),
            },
            Post {
                
                meta_data: PostMetadata {
                                title: String::from("Test Post 2"),
                                description: String::from("Some testing"),
                                date: String::from("2023-10-14"),
                                image_path: String::from("./public/pic01.jpg"),
                },
                content: String::from(
                    "<h1>Test</h1>\n<p>Post content</p>\n<p><img src=\"./public/pic01.jpg\" alt=\"pic\" /></p>\n",
                ),
            },

        ];

        let expected = vec![
            Post {
                
                meta_data: PostMetadata {
                                title: String::from("Test Post 2"),
                                description: String::from("Some testing"),
                                date: String::from("2023-10-14"),
                                image_path: String::from("./public/pic01.jpg"),
                },
                content: String::from(
                    "<h1>Test</h1>\n<p>Post content</p>\n<p><img src=\"./public/pic01.jpg\" alt=\"pic\" /></p>\n",
                ),
            },
            Post {
                
                meta_data: PostMetadata {
                                title: String::from("Test Post 1"),
                                description: String::from("Some more testing"),
                                date: String::from("2022-10-15"),
                                image_path: String::from("./public/pic01.jpg"),
                },
                content: String::from(
                    "<h1>Test</h1>\n<p>Post content</p>\n<p><img src=\"./public/pic01.jpg\" alt=\"pic\" /></p>\n",
                ),
            },

        ];
        assert_eq!(expected, order_posts(posts))
    }

    #[test]
    #[should_panic]
    fn test_read_incorrect_dir() {
        let _posts = read_markdown_files(INCORRECT_PATH.to_string());
    }
}
