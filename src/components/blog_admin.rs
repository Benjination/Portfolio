use yew::prelude::*;
use web_sys::{HtmlInputElement, HtmlTextAreaElement, window};
use wasm_bindgen_futures::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use wasm_bindgen::JsCast;
use crate::components::blog_auth::{AuthState, logout};

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
    // Split content fields for large blog posts
    pub main_content: Option<String>,
    pub overflow1: Option<String>,
    pub overflow2: Option<String>,
    pub overflow3: Option<String>,
    pub overflow4: Option<String>,
}

// Helper function to parse Firestore document format
fn parse_firestore_blog_post(fields: &Value, doc_name: &str) -> Result<BlogPost, String> {
    let get_string_field = |field_name: &str| -> String {
        fields.get(field_name)
            .and_then(|f| f.get("stringValue"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };
    
    let get_bool_field = |field_name: &str| -> bool {
        fields.get(field_name)
            .and_then(|f| f.get("booleanValue"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
    };
    
    let get_array_field = |field_name: &str| -> Vec<String> {
        fields.get(field_name)
            .and_then(|f| f.get("arrayValue"))
            .and_then(|a| a.get("values"))
            .and_then(|v| v.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|item| item.get("stringValue"))
                    .filter_map(|v| v.as_str())
                    .map(|s| s.to_string())
                    .collect()
            })
            .unwrap_or_default()
    };
    
    // Extract document ID from the full path
    let id = doc_name.split('/').last().unwrap_or("").to_string();
    
    // Handle content - check if we have split content fields or legacy single content field
    let main_content = fields.get("main_content")
        .and_then(|f| f.get("stringValue"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let overflow1 = fields.get("overflow1")
        .and_then(|f| f.get("stringValue"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let overflow2 = fields.get("overflow2")
        .and_then(|f| f.get("stringValue"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let overflow3 = fields.get("overflow3")
        .and_then(|f| f.get("stringValue"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    let overflow4 = fields.get("overflow4")
        .and_then(|f| f.get("stringValue"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    
    // Reconstruct full content from split fields or use legacy content field
    let full_content = if main_content.is_some() {
        // Use new split content system
        let mut content_parts = Vec::new();
        if let Some(ref main) = main_content {
            content_parts.push(main.clone());
        }
        if let Some(ref o1) = overflow1 {
            content_parts.push(o1.clone());
        }
        if let Some(ref o2) = overflow2 {
            content_parts.push(o2.clone());
        }
        if let Some(ref o3) = overflow3 {
            content_parts.push(o3.clone());
        }
        if let Some(ref o4) = overflow4 {
            content_parts.push(o4.clone());
        }
        content_parts.join("")
    } else {
        // Fall back to legacy single content field
        get_string_field("content")
    };
    
    Ok(BlogPost {
        id,
        title: get_string_field("title"),
        content: full_content,
        excerpt: get_string_field("excerpt"),
        author: get_string_field("author"),
        created_at: get_string_field("created_at"),
        updated_at: get_string_field("updated_at"),
        published: get_bool_field("published"),
        tags: get_array_field("tags"),
        main_content,
        overflow1,
        overflow2,
        overflow3,
        overflow4,
    })
}

// Split content into manageable chunks for Firestore storage
fn split_content_for_storage(content: &str) -> (Option<String>, Option<String>, Option<String>, Option<String>, Option<String>) {
    // Conservative chunk size to account for other document fields
    const CHUNK_SIZE: usize = 180_000; // ~180KB per chunk, leaves room for other fields
    
    web_sys::console::log_1(&format!("split_content_for_storage: Input content length: {} characters", content.len()).into());
    
    if content.is_empty() {
        web_sys::console::log_1(&"split_content_for_storage: Content is empty, returning None values".into());
        return (None, None, None, None, None);
    }
    
    if content.len() <= CHUNK_SIZE {
        // Content fits in main_content, use legacy approach for smaller posts
        web_sys::console::log_1(&format!("split_content_for_storage: Content fits in single chunk ({} <= {})", content.len(), CHUNK_SIZE).into());
        return (Some(content.to_string()), None, None, None, None);
    }
    
    web_sys::console::log_1(&format!("split_content_for_storage: Content needs splitting ({} > {})", content.len(), CHUNK_SIZE).into());
    
    // Split into chunks
    let mut chunks = Vec::new();
    let mut start = 0;
    
    while start < content.len() {
        let end = std::cmp::min(start + CHUNK_SIZE, content.len());
        
        // Try to break at a natural boundary (paragraph, sentence, or word)
        let mut actual_end = end;
        if end < content.len() {
            // Look for paragraph break first (within 500 chars of target)
            if let Some(para_pos) = content[start..end + 500.min(content.len() - end)]
                .rfind("\n\n") {
                actual_end = start + para_pos + 2;
                web_sys::console::log_1(&format!("Found paragraph break at position {}", actual_end).into());
            }
            // If no paragraph break, look for sentence break
            else if let Some(sent_pos) = content[start..end + 100.min(content.len() - end)]
                .rfind(". ") {
                actual_end = start + sent_pos + 2;
                web_sys::console::log_1(&format!("Found sentence break at position {}", actual_end).into());
            }
            // If no sentence break, look for word boundary
            else if let Some(word_pos) = content[start..end]
                .rfind(' ') {
                actual_end = start + word_pos + 1;
                web_sys::console::log_1(&format!("Found word break at position {}", actual_end).into());
            }
        }
        
        let chunk_content = content[start..actual_end].to_string();
        web_sys::console::log_1(&format!("Creating chunk {}: {} characters (positions {}-{})", 
            chunks.len(), chunk_content.len(), start, actual_end).into());
        chunks.push(chunk_content);
        start = actual_end;
        
        // Safety limit - max 5 chunks
        if chunks.len() >= 5 {
            web_sys::console::log_1(&"Reached maximum 5 chunks limit".into());
            break;
        }
    }
    
    web_sys::console::log_1(&format!("Total chunks created: {}", chunks.len()).into());
    
    // Pad with None values to fill tuple
    while chunks.len() < 5 {
        chunks.push(String::new());
    }
    
    (
        if chunks[0].is_empty() { None } else { Some(chunks[0].clone()) },
        if chunks[1].is_empty() { None } else { Some(chunks[1].clone()) },
        if chunks[2].is_empty() { None } else { Some(chunks[2].clone()) },
        if chunks[3].is_empty() { None } else { Some(chunks[3].clone()) },
        if chunks[4].is_empty() { None } else { Some(chunks[4].clone()) },
    )
}

// Function to trigger GitHub Actions workflow to regenerate static blog pages
async fn trigger_static_page_regeneration() {
    web_sys::console::log_1(&"🔄 Blog post saved successfully!".into());
    web_sys::console::log_1(&"📝 To update static blog pages for shared links:".into());
    web_sys::console::log_1(&"   1. Open terminal in your project directory".into());
    web_sys::console::log_1(&"   2. Run: npm run regenerate-blogs".into());
    web_sys::console::log_1(&"   3. Commit and push the updated dist/blog/ files".into());
    web_sys::console::log_1(&"".into());
    web_sys::console::log_1(&"💡 This will ensure shared blog links show the updated content!".into());
    
    // Show an alert to the user as well
    if let Some(window) = window() {
        let message = "Blog post saved! \n\nTo update static pages for shared links:\n1. Run 'npm run regenerate-blogs' in terminal\n2. Commit and push the changes\n\nThis ensures shared blog links show updated content.";
        let _ = window.alert_with_message(message);
    }
}

#[derive(Properties, PartialEq)]
pub struct BlogAdminProps {
    pub auth_state: AuthState,
    pub on_logout: Callback<()>,
}

#[function_component(BlogAdmin)]
pub fn blog_admin(props: &BlogAdminProps) -> Html {
    let posts = use_state(|| Vec::<BlogPost>::new());
    let loading = use_state(|| false);
    let show_new_post = use_state(|| false);
    let current_post = use_state(|| None::<BlogPost>);
    
    // Form states
    let post_title = use_state(|| String::new());
    let post_content = use_state(|| String::new());
    let post_excerpt = use_state(|| String::new());
    let post_tags = use_state(|| String::new());
    let post_published = use_state(|| false);

    // Load posts on component mount
    {
        let posts = posts.clone();
        let loading = loading.clone();
        let auth_token = props.auth_state.token.clone();
        
        use_effect_with((), move |_| {
            if let Some(token) = auth_token {
                loading.set(true);
                spawn_local(async move {
                    // Use Firestore instead of Realtime Database
                    let url = format!(
                        "https://firestore.googleapis.com/v1/projects/portfolio-7148b/databases/(default)/documents/blogs"
                    );
                    
                    web_sys::console::log_1(&format!("Making request to: {}", url).into());
                    web_sys::console::log_1(&format!("Using token: {}...", &token[..std::cmp::min(20, token.len())]).into());
                    
                    match Request::get(&url)
                        .header("Authorization", &format!("Bearer {}", token))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            web_sys::console::log_1(&format!("Response status: {}", response.status()).into());
                            if response.status() == 200 {
                                if let Ok(data) = response.json::<Value>().await {
                                    web_sys::console::log_1(&format!("Received data: {:?}", data).into());
                                    let mut blogs = Vec::new();
                                    
                                    // Parse Firestore document format
                                    if let Some(documents) = data.get("documents").and_then(|d| d.as_array()) {
                                        for doc in documents {
                                            if let Some(fields) = doc.get("fields") {
                                                // Extract blog post data from Firestore format
                                                if let Ok(post) = parse_firestore_blog_post(fields, doc.get("name").and_then(|n| n.as_str()).unwrap_or("")) {
                                                    blogs.push(post);
                                                }
                                            }
                                        }
                                    }
                                    
                                    // Sort by creation date (newest first)
                                    blogs.sort_by(|a, b| b.created_at.cmp(&a.created_at));
                                    posts.set(blogs);
                                } else {
                                    web_sys::console::log_1(&"Failed to parse response as JSON".into());
                                }
                            } else if response.status() == 404 {
                                // Collection doesn't exist yet - that's ok for a new blog
                                web_sys::console::log_1(&"Blog collection doesn't exist yet - creating empty posts list".into());
                                posts.set(Vec::new());
                            } else if response.status() == 403 {
                                web_sys::console::log_1(&"Authentication error (403 Forbidden). This usually means:".into());
                                web_sys::console::log_1(&"1. Your Firebase auth token has expired".into());
                                web_sys::console::log_1(&"2. Firestore security rules don't allow read access".into());
                                web_sys::console::log_1(&"3. The token format is incorrect".into());
                                
                                // Try to get the error details from the response
                                if let Ok(error_text) = response.text().await {
                                    web_sys::console::log_1(&format!("Error details: {}", error_text).into());
                                }
                                
                                // Clear the token from localStorage since it's not working
                                if let Some(storage) = window()
                                    .and_then(|w| w.local_storage().ok())
                                    .flatten()
                                {
                                    let _ = storage.remove_item("blog_auth_token");
                                    let _ = storage.remove_item("blog_user_id");
                                    let _ = storage.remove_item("blog_user_email");
                                }
                                
                                // Set empty posts for now
                                posts.set(Vec::new());
                            } else {
                                web_sys::console::log_1(&format!("HTTP error: {}", response.status()).into());
                                if let Ok(error_text) = response.text().await {
                                    web_sys::console::log_1(&format!("Error details: {}", error_text).into());
                                }
                            }
                        }
                        Err(e) => {
                            web_sys::console::log_1(&format!("Network error: {:?}", e).into());
                        }
                    }
                    loading.set(false);
                });
            }
            || ()
        });
    }

    let on_new_post = {
        let show_new_post = show_new_post.clone();
        let current_post = current_post.clone();
        let post_title = post_title.clone();
        let post_content = post_content.clone();
        let post_excerpt = post_excerpt.clone();
        let post_tags = post_tags.clone();
        let post_published = post_published.clone();
        
        Callback::from(move |_| {
            // Reset form
            post_title.set(String::new());
            post_content.set(String::new());
            post_excerpt.set(String::new());
            post_tags.set(String::new());
            post_published.set(false);
            current_post.set(None);
            show_new_post.set(!*show_new_post);
        })
    };

    let on_save_post = {
        let posts = posts.clone();
        let show_new_post = show_new_post.clone();
        let current_post = current_post.clone();
        let post_title = post_title.clone();
        let post_content = post_content.clone();
        let post_excerpt = post_excerpt.clone();
        let post_tags = post_tags.clone();
        let post_published = post_published.clone();
        let auth_token = props.auth_state.token.clone();
        let author_email = props.auth_state.email.clone();
        
        Callback::from({
            move |_| {
            if let (Some(token), Some(email)) = (auth_token.clone(), author_email.clone()) {
                let title = (*post_title).clone();
                let content = (*post_content).clone();
                let excerpt = (*post_excerpt).clone();
                let tags_str = (*post_tags).clone();
                let published = *post_published;
                
                if !title.trim().is_empty() && !content.trim().is_empty() {
                    let posts = posts.clone();
                    let show_new_post = show_new_post.clone();
                    let current_post = current_post.clone();
                    let post_title = post_title.clone();
                    let post_content = post_content.clone();
                    let post_excerpt = post_excerpt.clone();
                    let post_tags = post_tags.clone();
                    let post_published = post_published.clone();
                    let current_post_data = (*current_post).clone();
                    
                    spawn_local(async move {
                        let now = js_sys::Date::new_0().to_iso_string().as_string()
                            .unwrap_or_else(|| {
                                web_sys::console::log_1(&"Failed to generate ISO date string, using fallback".into());
                                // Fallback to a basic timestamp
                                format!("{}", js_sys::Date::now())
                            });
                        
                        // Determine if we're editing or creating
                        let (post_id, created_at) = if let Some(ref existing_post) = current_post_data {
                            (existing_post.id.clone(), existing_post.created_at.clone())
                        } else {
                            (format!("post_{}", js_sys::Date::now() as u64), now.clone())
                        };
                        
                        let tags: Vec<String> = tags_str
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .filter(|s| !s.is_empty())
                            .collect();
                        
                        // Split content into manageable chunks
                        let (main_content, overflow1, overflow2, overflow3, overflow4) = split_content_for_storage(&content);
                        
                        let updated_post = BlogPost {
                            id: post_id.clone(),
                            title,
                            content, // Keep full content for display
                            excerpt: if excerpt.trim().is_empty() { 
                                "No excerpt provided".to_string() 
                            } else { 
                                excerpt 
                            },
                            author: email,
                            created_at: created_at,
                            updated_at: now,
                            published,
                            tags: tags.clone(),
                            main_content: main_content.clone(),
                            overflow1: overflow1.clone(),
                            overflow2: overflow2.clone(),
                            overflow3: overflow3.clone(),
                            overflow4: overflow4.clone(),
                        };
                        
                        // Create Firestore document with split content fields
                        let mut firestore_fields = json!({
                            "title": {"stringValue": updated_post.title},
                            "excerpt": {"stringValue": updated_post.excerpt},
                            "author": {"stringValue": updated_post.author},
                            "created_at": {"stringValue": updated_post.created_at},
                            "updated_at": {"stringValue": updated_post.updated_at},
                            "published": {"booleanValue": updated_post.published},
                            "tags": {
                                "arrayValue": {
                                    "values": tags.iter().map(|tag| json!({"stringValue": tag})).collect::<Vec<_>>()
                                }
                            }
                        });
                        
                        // Add content fields (new split system or legacy single field)
                        if main_content.is_some() || overflow1.is_some() || overflow2.is_some() || overflow3.is_some() || overflow4.is_some() {
                            // Use new split content system
                            if let Some(ref mc) = main_content {
                                firestore_fields["main_content"] = json!({"stringValue": mc});
                            }
                            if let Some(ref o1) = overflow1 {
                                firestore_fields["overflow1"] = json!({"stringValue": o1});
                            }
                            if let Some(ref o2) = overflow2 {
                                firestore_fields["overflow2"] = json!({"stringValue": o2});
                            }
                            if let Some(ref o3) = overflow3 {
                                firestore_fields["overflow3"] = json!({"stringValue": o3});
                            }
                            if let Some(ref o4) = overflow4 {
                                firestore_fields["overflow4"] = json!({"stringValue": o4});
                            }
                        } else {
                            // Fallback to legacy single content field for empty posts
                            firestore_fields["content"] = json!({"stringValue": updated_post.content});
                        }
                        
                        let firestore_doc = json!({
                            "fields": firestore_fields
                        });
                        
                        // Final size check of the complete document
                        let doc_string = firestore_doc.to_string();
                        let doc_size = doc_string.len();
                        const MAX_FIRESTORE_DOC_SIZE: usize = 1_000_000; // 1MB
                        
                        web_sys::console::log_1(&format!("Final Firestore document size: {} bytes", doc_size).into());
                        
                        if doc_size > MAX_FIRESTORE_DOC_SIZE {
                            web_sys::console::log_1(&format!("Error: Final document too large ({} bytes). Firestore limit: {} bytes", 
                                doc_size, MAX_FIRESTORE_DOC_SIZE).into());
                            window().unwrap().alert_with_message("The complete blog post is too large for Firestore. Please reduce content size.").unwrap();
                            return;
                        }
                        
                        let url = format!(
                            "https://firestore.googleapis.com/v1/projects/portfolio-7148b/databases/(default)/documents/blogs/{}",
                            post_id
                        );
                        
                        web_sys::console::log_1(&format!("Saving post to: {}", url).into());
                        
                        match Request::patch(&url)
                            .header("Authorization", &format!("Bearer {}", token))
                            .header("Content-Type", "application/json")
                            .json(&firestore_doc)
                        {
                            Ok(request) => {
                                match request.send().await {
                                    Ok(response) => {
                                        web_sys::console::log_1(&format!("Save response status: {}", response.status()).into());
                                        if response.status() == 200 || response.status() == 201 {
                                            let mut current_posts = (*posts).clone();
                                            
                                            // Check if we're editing an existing post
                                            if let Some(ref existing_post) = current_post_data {
                                                // Find and update the existing post
                                                if let Some(index) = current_posts.iter().position(|p| p.id == existing_post.id) {
                                                    current_posts[index] = updated_post;
                                                }
                                            } else {
                                                // Add new post to beginning
                                                current_posts.insert(0, updated_post);
                                            }
                                            
                                            posts.set(current_posts);
                                            show_new_post.set(false);
                                            current_post.set(None); // Clear editing state
                                            web_sys::console::log_1(&"Blog post saved successfully!".into());
                                            
                                            // Trigger static page regeneration
                                            spawn_local(async move {
                                                trigger_static_page_regeneration().await;
                                            });
                                            
                                            // Clear form
                                            post_title.set(String::new());
                                            post_content.set(String::new());
                                            post_excerpt.set(String::new());
                                            post_tags.set(String::new());
                                            post_published.set(false);
                                        } else {
                                            // Try to get more detailed error information
                                            let status = response.status();
                                            match response.text().await {
                                                Ok(error_text) => {
                                                    web_sys::console::log_1(&format!("Failed to save blog post: {} - {}", status, error_text).into());
                                                    if status == 400 {
                                                        window().unwrap().alert_with_message("Bad Request: The blog post data is invalid or too large. Please check content size and try again.").unwrap();
                                                    } else if status == 401 {
                                                        window().unwrap().alert_with_message("Authentication failed. Please sign in again.").unwrap();
                                                    } else if status == 403 {
                                                        window().unwrap().alert_with_message("Permission denied. Please check your authentication.").unwrap();
                                                    } else {
                                                        window().unwrap().alert_with_message(&format!("Failed to save blog post. Error: {}", status)).unwrap();
                                                    }
                                                }
                                                Err(_) => {
                                                    web_sys::console::log_1(&format!("Failed to save blog post: {}", status).into());
                                                    window().unwrap().alert_with_message(&format!("Failed to save blog post. Error: {}", status)).unwrap();
                                                }
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        web_sys::console::log_1(&format!("Network error saving post: {:?}", e).into());
                                    }
                                }
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Failed to create save request: {:?}", e).into());
                            }
                        }
                    });
                }
            }
        }})
    };

    let on_logout = {
        let on_logout = props.on_logout.clone();
        Callback::from(move |_| {
            logout();
            on_logout.emit(());
        })
    };

    html! {
        <div class="blog-admin">
            <div class="blog-admin-header">
                <h1>{"🚀 Blog Administration Panel"}</h1>
                <div class="admin-info">
                    <span>{"Welcome, "}{props.auth_state.email.as_ref().unwrap_or(&"Admin".to_string())}</span>
                    <button class="logout-btn" onclick={on_logout}>{"Logout"}</button>
                </div>
            </div>
            
            <div class="blog-admin-content">
                <div class="admin-actions">
                    <button class="new-post-btn" onclick={on_new_post}>
                        {if *show_new_post { "Cancel" } else { "New Post" }}
                    </button>
                </div>
                
                if *show_new_post {
                    <div class="post-editor">
                        <h2>{"Create New Blog Post"}</h2>
                        <div class="form-group">
                            <label for="post-title">{"Title:"}</label>
                            <input 
                                id="post-title"
                                type="text" 
                                value={(*post_title).clone()}
                                oninput={
                                    let post_title = post_title.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        post_title.set(input.value());
                                    })
                                }
                                placeholder="Enter blog post title..."
                                class="form-input"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="post-excerpt">{"Excerpt:"}</label>
                            <input 
                                id="post-excerpt"
                                type="text" 
                                value={(*post_excerpt).clone()}
                                oninput={
                                    let post_excerpt = post_excerpt.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        post_excerpt.set(input.value());
                                    })
                                }
                                placeholder="Brief description of the post..."
                                class="form-input"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="post-tags">{"Tags (comma-separated):"}</label>
                            <input 
                                id="post-tags"
                                type="text" 
                                value={(*post_tags).clone()}
                                oninput={
                                    let post_tags = post_tags.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlInputElement = e.target_unchecked_into();
                                        post_tags.set(input.value());
                                    })
                                }
                                placeholder="rust, programming, tutorial"
                                class="form-input"
                            />
                        </div>
                        
                        <div class="form-group">
                            <label for="post-content">{"Content:"}</label>
                            <div class="content-stats">
                                <span class="char-count">
                                    {format!("Characters: {} / 500,000", post_content.len())}
                                    {if post_content.len() > 400_000 { " ⚠️" } else { "" }}
                                </span>
                                <span class="word-count">
                                    {format!("Words: ~{}", post_content.split_whitespace().count())}
                                </span>
                            </div>
                            <textarea 
                                id="post-content"
                                value={(*post_content).clone()}
                                oninput={
                                    let post_content = post_content.clone();
                                    Callback::from(move |e: InputEvent| {
                                        let input: HtmlTextAreaElement = e.target_unchecked_into();
                                        post_content.set(input.value());
                                    })
                                }
                                placeholder="Write your blog post content here..."
                                class="form-textarea"
                                rows="15"
                            ></textarea>
                        </div>
                        
                        <div class="form-group">
                            <label class="checkbox-label">
                                <input 
                                    type="checkbox" 
                                    checked={*post_published}
                                    onchange={
                                        let post_published = post_published.clone();
                                        Callback::from(move |e: Event| {
                                            let input: HtmlInputElement = e.target_unchecked_into();
                                            post_published.set(input.checked());
                                        })
                                    }
                                />
                                {"Publish immediately"}
                            </label>
                        </div>
                        
                        <div class="form-actions">
                            <button 
                                class="save-btn" 
                                onclick={on_save_post}
                            >
                                {"Save Post"}
                            </button>
                        </div>
                    </div>
                }
                
                <div class="posts-list">
                    <h2>{"Blog Posts"}</h2>
                    if *loading {
                        <div class="loading">{"Loading posts..."}</div>
                    } else if posts.is_empty() {
                        <div class="no-posts">{"No blog posts yet. Create your first post!"}</div>
                    } else {
                        <div class="posts-grid">
                            {for posts.iter().map(|post| {
                                html! {
                                    <div class="post-card" key={post.id.clone()}>
                                        <div class="post-header">
                                            <h3>{&post.title}</h3>
                                            <div class="post-status">
                                                {if post.published { "✅ Published" } else { "📝 Draft" }}
                                            </div>
                                        </div>
                                        <p class="post-excerpt">{&post.excerpt}</p>
                                        <div class="post-meta">
                                            <span class="post-date">{&post.created_at[..10]}</span>
                                            <div class="post-tags">
                                                {for post.tags.iter().map(|tag| html! {
                                                    <span class="tag" key={tag.clone()}>{tag}</span>
                                                })}
                                            </div>
                                        </div>
                                        <div class="post-actions">
                                            <button class="edit-btn" 
                                                onclick={{
                                                    let post_clone = post.clone();
                                                    let current_post = current_post.clone();
                                                    let show_new_post = show_new_post.clone();
                                                    let post_title = post_title.clone();
                                                    let post_content = post_content.clone();
                                                    let post_excerpt = post_excerpt.clone();
                                                    let post_tags = post_tags.clone();
                                                    let post_published = post_published.clone();
                                                    
                                                    Callback::from(move |_| {
                                                        // Set current post for editing
                                                        current_post.set(Some(post_clone.clone()));
                                                        
                                                        // Populate form with existing data
                                                        post_title.set(post_clone.title.clone());
                                                        post_content.set(post_clone.content.clone());
                                                        post_excerpt.set(post_clone.excerpt.clone());
                                                        post_tags.set(post_clone.tags.join(", "));
                                                        post_published.set(post_clone.published);
                                                        
                                                        // Show the form
                                                        show_new_post.set(true);
                                                    })
                                                }}>
                                                {"Edit"}
                                            </button>
                                            <button class="delete-btn">{"Delete"}</button>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                    }
                </div>
            </div>
        </div>
    }
}
