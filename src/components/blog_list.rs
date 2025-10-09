use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use wasm_bindgen_futures::spawn_local;
use super::super::Route;

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub content: String,
    pub excerpt: String,
    pub author: String,
    pub created_at: String,
    pub updated_at: String,
    pub published: bool,
    pub tags: Vec<String>,
    pub sort_order: i32,
    pub header_image: Option<String>,
}

#[derive(Properties, PartialEq)]
pub struct BlogListProps {}

#[function_component(BlogList)]
pub fn blog_list(_props: &BlogListProps) -> Html {
    let posts = use_state(|| Vec::<BlogPost>::new());
    let loading = use_state(|| true);

    // Fetch blog posts on component mount
    {
        let posts = posts.clone();
        let loading = loading.clone();
        
        use_effect_with((), move |_| {
            spawn_local(async move {
                loading.set(true);
                
                let url = "https://firestore.googleapis.com/v1/projects/portfolio-7148b/databases/(default)/documents/blogs?key=AIzaSyAsmk3uImdPFOPLZrEsK6J1c20gk8S3hbY";
                
                match Request::get(url).send().await {
                    Ok(response) => {
                        if response.status() == 200 {
                            if let Ok(data) = response.json::<Value>().await {
                                let mut blog_posts = Vec::new();
                                
                                // Parse Firestore document format
                                if let Some(documents) = data.get("documents").and_then(|d| d.as_array()) {
                                    for doc in documents {
                                        if let Some(fields) = doc.get("fields") {
                                            if let Some(name) = doc.get("name").and_then(|n| n.as_str()) {
                                                let title = fields.get("title")
                                                    .and_then(|f| f.get("stringValue"))
                                                    .and_then(|v| v.as_str())
                                                    .unwrap_or("Untitled")
                                                    .to_string();
                                                    
                                                // Create ID from first 30 characters of title (URL-safe)
                                                let id = title.to_lowercase()
                                                    .chars()
                                                    .filter(|c| c.is_alphanumeric() || c.is_whitespace())
                                                    .collect::<String>()
                                                    .split_whitespace()
                                                    .collect::<Vec<&str>>()
                                                    .join("-")
                                                    .chars()
                                                    .take(30)
                                                    .collect::<String>()
                                                    .trim_end_matches('-')
                                                    .to_string();
                                                    
                                                let content = fields.get("main_content")
                                                    .and_then(|f| f.get("stringValue"))
                                                    .and_then(|v| v.as_str())
                                                    .or_else(|| fields.get("content")
                                                        .and_then(|f| f.get("stringValue"))
                                                        .and_then(|v| v.as_str()))
                                                    .unwrap_or("")
                                                    .to_string();
                                                
                                                // Try both 'excerpt' and content preview
                                                let excerpt = fields.get("excerpt")
                                                    .and_then(|f| f.get("stringValue"))
                                                    .and_then(|v| v.as_str())
                                                    .map(|s| s.to_string())
                                                    .unwrap_or_else(|| {
                                                        // If no excerpt, create one from content
                                                        let preview = content.chars().take(100).collect::<String>();
                                                        if preview.len() < content.len() {
                                                            format!("{}...", preview)
                                                        } else {
                                                            preview
                                                        }
                                                    });
                                                    
                                                // Try different field names for published status
                                                let published = fields.get("is_published")
                                                    .and_then(|f| f.get("booleanValue"))
                                                    .and_then(|v| v.as_bool())
                                                    .or_else(|| fields.get("published")
                                                        .and_then(|f| f.get("booleanValue"))
                                                        .and_then(|v| v.as_bool()))
                                                    .unwrap_or(false);
                                                
                                                // Get sort order (defaults to 999 if not specified)
                                                let sort_order = fields.get("sort_order")
                                                    .and_then(|f| f.get("integerValue"))
                                                    .and_then(|v| v.as_str())
                                                    .and_then(|s| s.parse::<i32>().ok())
                                                    .unwrap_or(999);
                                                
                                                if published {
                                                    // Get header image if present
                                                    let header_image = fields.get("header_image")
                                                        .and_then(|f| f.get("stringValue"))
                                                        .and_then(|v| v.as_str())
                                                        .map(|s| s.to_string());
                                                    
                                                    blog_posts.push(BlogPost {
                                                        id,
                                                        title,
                                                        content,
                                                        excerpt,
                                                        author: "Benjamin".to_string(),
                                                        created_at: "2024-01-01".to_string(),
                                                        updated_at: "2024-01-01".to_string(),
                                                        published,
                                                        tags: vec![],
                                                        sort_order,
                                                        header_image,
                                                    });
                                                }
                                            }
                                        }
                                    }
                                }
                                
                                // Sort blog posts by sort_order (ascending: 1, 2, 3, etc.)
                                blog_posts.sort_by(|a, b| a.sort_order.cmp(&b.sort_order));
                                
                                posts.set(blog_posts);
                            }
                        }
                    }
                    Err(_) => {
                        // Handle error silently for now
                    }
                }
                
                loading.set(false);
            });
            || ()
        });
    }

    let navigator = use_navigator().unwrap();

    html! {
        <section class="blog-section">
            <div class="terminal-window">
                <div class="terminal-header">
                    <div class="terminal-title">
                        {"📝 ~/blog"}
                    </div>
                </div>
                <div class="terminal-content">
                    <div class="command-line">
                        <span class="prompt">{"user@portfolio:~$ "}</span>
                        <span class="command">{"ls -la blog/"}</span>
                    </div>
                    
                    if *loading {
                        <div class="blog-loading">
                            <span class="blinking-cursor">{"Loading blog posts..."}</span>
                        </div>
                    } else if posts.is_empty() {
                        <div class="blog-empty">
                            <span class="info-text">{"No published blog posts found."}</span>
                        </div>
                    } else {
                        <div class="blog-posts">
                            {
                                posts.iter().enumerate().map(|(index, post)| {
                                    let post_clone = post.clone();
                                    let navigator = navigator.clone();
                                    let on_click = {
                                        let post_id = post_clone.id.clone();
                                        let navigator = navigator.clone();
                                        Callback::from(move |_| {
                                            navigator.push(&Route::BlogPost { id: post_id.clone() });
                                        })
                                    };
                                    
                                    html! {
                                        <div key={index} class="blog-post-item" onclick={on_click}>
                                            <div class="blog-post-line">
                                                <span class="post-number">{format!("{}.", index + 1)}</span>
                                                {if let Some(header_image) = &post.header_image {
                                                    html! {
                                                        <span class="blog-thumbnail">
                                                            <img src={header_image.clone()} alt="Post thumbnail" class="thumbnail-image" />
                                                        </span>
                                                    }
                                                } else {
                                                    html! {}
                                                }}
                                                <span class="file-name blog-link">
                                                    {&post.title}
                                                </span>
                                            </div>
                                            <div class="blog-post-description">
                                                <span class="file-description">
                                                    {format!("   # {}", post.excerpt)}
                                                </span>
                                            </div>
                                        </div>
                                    }
                                }).collect::<Html>()
                            }
                        </div>
                    }
                </div>
            </div>
        </section>
    }
}
