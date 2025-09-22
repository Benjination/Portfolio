use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use super::super::Route;

const FIREBASE_API_KEY: &str = "AIzaSyBxK-Lm3gXPWdKh-8Rb7xQLPO-8E_yGMuE";

#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogPostData {
    pub id: String,
    pub title: String,
    pub content: String,
    pub author: String,
    pub date_created: String,
    pub date_updated: String,
    pub is_published: bool,
    pub tags: Vec<String>,
}

fn parse_firestore_blog_post(doc: &serde_json::Value, doc_id: &str) -> Option<BlogPostData> {
    let fields = doc.get("fields")?;
    
    // More robust parsing with better error handling
    let title = fields
        .get("title")
        .and_then(|f| f.get("stringValue"))
        .and_then(|v| v.as_str())
        .unwrap_or("Untitled Post")
        .to_string();
        
    // Try multiple possible field names for content
    let content = fields
        .get("main_content")
        .and_then(|f| f.get("stringValue"))
        .and_then(|v| v.as_str())
        .or_else(|| fields
            .get("content")
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str()))
        .or_else(|| fields
            .get("body")
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str()))
        .or_else(|| fields
            .get("text")
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str()))
        .or_else(|| fields
            .get("description")
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str()))
        .unwrap_or("No content available")
        .to_string();
    
    Some(BlogPostData {
        id: doc_id.to_string(),
        title,
        content,
        author: fields
            .get("author")
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str())
            .unwrap_or("Benjamin")
            .to_string(),
        date_created: fields
            .get("date_created")
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str())
            .or_else(|| fields.get("created_at")
                .and_then(|f| f.get("stringValue"))
                .and_then(|v| v.as_str()))
            .unwrap_or("2024-01-01")
            .to_string(),
        date_updated: fields
            .get("date_updated")
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str())
            .or_else(|| fields.get("updated_at")
                .and_then(|f| f.get("stringValue"))
                .and_then(|v| v.as_str()))
            .unwrap_or("2024-01-01")
            .to_string(),
        is_published: fields
            .get("is_published")
            .and_then(|f| f.get("booleanValue"))
            .and_then(|v| v.as_bool())
            .or_else(|| fields.get("published")
                .and_then(|f| f.get("booleanValue"))
                .and_then(|v| v.as_bool()))
            .unwrap_or(false),
        tags: fields
            .get("tags")
            .and_then(|f| f.get("arrayValue"))
            .and_then(|arr| arr.get("values"))
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter()
                .filter_map(|v| v.get("stringValue")?.as_str().map(|s| s.to_string()))
                .collect())
            .unwrap_or_else(Vec::new),
    })
}

#[derive(Properties, PartialEq)]
pub struct BlogPostProps {
    pub post_id: String,
}

#[function_component]
pub fn BlogPost(props: &BlogPostProps) -> Html {
    let post = use_state(|| None::<BlogPostData>);
    let loading = use_state(|| true);
    let error = use_state(|| None::<String>);

    let post_id = props.post_id.clone();
    let post_clone = post.clone();
    let loading_clone = loading.clone();
    let error_clone = error.clone();

    use_effect_with(post_id.clone(), move |post_id| {
        let post_id = post_id.clone();
        let post = post_clone;
        let loading = loading_clone;
        let error = error_clone;

        // Scroll to top when blog post loads
        if let Some(window) = window() {
            let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
        }

        spawn_local(async move {
            // Fetch all blog posts to find the one with matching title-based ID
            let url = format!(
                "https://firestore.googleapis.com/v1/projects/portfolio-7148b/databases/(default)/documents/blogs?key={}",
                FIREBASE_API_KEY
            );

            match Request::get(&url).send().await {
                Ok(response) => {
                    if response.ok() {
                        match response.json::<serde_json::Value>().await {
                            Ok(data) => {
                                let mut found = false;
                                if let Some(documents) = data.get("documents").and_then(|d| d.as_array()) {
                                    // Look for the blog post with matching title-based ID
                                    for doc in documents {
                                        if let Some(fields) = doc.get("fields") {
                                            let title = fields.get("title")
                                                .and_then(|f| f.get("stringValue"))
                                                .and_then(|v| v.as_str())
                                                .unwrap_or("")
                                                .to_string();
                                                
                                            // Create title-based ID to match
                                            let title_id = title.to_lowercase()
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
                                                
                                            if title_id == post_id {
                                                if let Some(blog_post) = parse_firestore_blog_post(doc, &title_id) {
                                                    if blog_post.is_published {
                                                        post.set(Some(blog_post.clone()));
                                                        
                                                        // Update the browser URL to show the correct blog path
                                                        if let Some(window) = window() {
                                                            let location = window.location();
                                                            let expected_path = format!("/blog/{}", post_id);
                                                            if let Ok(current_path) = location.pathname() {
                                                                if current_path != expected_path {
                                                                    // Update the URL without triggering a page reload
                                                                    if let Ok(history) = window.history() {
                                                                        let _ = history.push_state_with_url(
                                                                            &wasm_bindgen::JsValue::NULL,
                                                                            &blog_post.title,
                                                                            Some(&expected_path)
                                                                        );
                                                                    }
                                                                }
                                                            }
                                                            
                                                            // Update the document title
                                                            if let Some(document) = window.document() {
                                                                let title = format!("{} - Benjamin Niccum", blog_post.title);
                                                                document.set_title(&title);
                                                            }
                                                        }
                                                        
                                                        found = true;
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                if !found {
                                    error.set(Some("Blog post not found".to_string()));
                                }
                            }
                            Err(_) => {
                                error.set(Some("Failed to parse response".to_string()));
                            }
                        }
                    } else {
                        let status = response.status();
                        error.set(Some(format!("Failed to fetch blog post: {}", status)));
                    }
                }
                Err(_) => {
                    error.set(Some("Network error occurred".to_string()));
                }
            }
            loading.set(false);
        });

        || ()
    });

    let navigator = use_navigator().unwrap();

    if *loading {
        html! {
            <div class="blog-post-page">
                <div class="blog-content-container">
                    <div class="blog-header">
                        <button class="back-button" onclick={
                            let navigator = navigator.clone();
                            move |_| navigator.push(&Route::Home)
                        }>
                            {"← Back to Blog List"}
                        </button>
                    </div>
                    <div class="loading-content">
                        <div class="loading-indicator">
                            <h2>{"Loading Blog Post..."}</h2>
                            <p>{"Please wait while we fetch the content..."}</p>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else if let Some(error_msg) = (*error).as_ref() {
        html! {
            <div class="blog-post-page">
                <div class="blog-content-container">
                    <div class="blog-header">
                        <button class="back-button" onclick={
                            let navigator = navigator.clone();
                            move |_| navigator.push(&Route::Home)
                        }>
                            {"← Back to Blog List"}
                        </button>
                    </div>
                    <div class="error-content">
                        <div class="error-message">
                            <h2>{"Error Loading Blog Post"}</h2>
                            <p>{error_msg}</p>
                            <p>{"Please try again or go back to the blog list."}</p>
                        </div>
                    </div>
                </div>
            </div>
        }
    } else if let Some(blog_post) = (*post).as_ref() {
        html! {
            <div class="blog-post-page">
                <div class="blog-content-container">
                    <div class="blog-header">
                        <button class="back-button" onclick={
                            let navigator = navigator.clone();
                            move |_| navigator.push(&Route::Home)
                        }>
                            {"← Back to Blog List"}
                        </button>
                        <div class="post-meta">
                            <h1 class="post-title">{&blog_post.title}</h1>
                            <div class="post-info">
                                <span class="author">{"By "}{&blog_post.author}</span>
                                <span class="date-separator">{" • "}</span>
                                <span class="date">{&blog_post.date_created}</span>
                                {if blog_post.date_created != blog_post.date_updated {
                                    html! {
                                        <>
                                            <span class="date-separator">{" • "}</span>
                                            <span class="date updated">{"Updated "}{&blog_post.date_updated}</span>
                                        </>
                                    }
                                } else {
                                    html! {}
                                }}
                            </div>
                            {if !blog_post.tags.is_empty() {
                                html! {
                                    <div class="tags">
                                        {for blog_post.tags.iter().map(|tag| {
                                            html! {
                                                <span class="tag">{tag}</span>
                                            }
                                        })}
                                    </div>
                                }
                            } else {
                                html! {}
                            }}
                        </div>
                    </div>
                    <div class="blog-content">
                        <div class="content-text">
                            {format_content(&blog_post.content)}
                        </div>
                    </div>
                </div>
            </div>
        }
    } else {
        html! {
            <div class="blog-post-page">
                <div class="blog-content-container">
                    <div class="blog-header">
                        <button class="back-button" onclick={
                            let navigator = navigator.clone();
                            move |_| navigator.push(&Route::Home)
                        }>
                            {"← Back to Blog List"}
                        </button>
                    </div>
                    <div class="error-content">
                        <div class="error-message">
                            <h2>{"Blog Post Not Found"}</h2>
                            <p>{"The requested blog post could not be found."}</p>
                            <p>{"It may have been moved or deleted."}</p>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

fn format_content(content: &str) -> Html {
    let lines: Vec<&str> = content.split('\n').collect();
    
    html! {
        <div class="formatted-content">
            {for lines.iter().enumerate().map(|(i, line)| {
                if line.trim().is_empty() {
                    html! { <br /> }
                } else if line.starts_with("# ") {
                    html! { <h2 class="content-h2">{&line[2..]}</h2> }
                } else if line.starts_with("## ") {
                    html! { <h3 class="content-h3">{&line[3..]}</h3> }
                } else if line.starts_with("### ") {
                    html! { <h4 class="content-h4">{&line[4..]}</h4> }
                } else if line.starts_with("```") {
                    html! { <div class="code-block-delimiter">{line}</div> }
                } else if line.starts_with("- ") || line.starts_with("* ") {
                    html! { <div class="list-item">{line}</div> }
                } else if line.starts_with("> ") {
                    html! { <div class="blockquote">{&line[2..]}</div> }
                } else {
                    html! { <p class="content-paragraph" key={i}>{line}</p> }
                }
            })}
        </div>
    }
}