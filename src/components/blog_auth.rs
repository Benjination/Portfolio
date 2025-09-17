use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::window;

// Firebase Auth configuration
const FIREBASE_API_KEY: &str = "AIzaSyAsmk3uImdPFOPLZrEsK6J1c20gk8S3hbY"; // Your Firebase API key
const FIREBASE_PROJECT_ID: &str = "portfolio-7148b";

#[derive(Serialize)]
struct SignInRequest {
    email: String,
    password: String,
    #[serde(rename = "returnSecureToken")]
    return_secure_token: bool,
}

#[derive(Deserialize)]
struct SignInResponse {
    #[serde(rename = "idToken")]
    id_token: String,
    #[serde(rename = "localId")]
    local_id: String,
    email: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: ErrorDetails,
}

#[derive(Deserialize)]
struct ErrorDetails {
    code: u32,
    message: String,
}

// Authentication state management
#[derive(Clone, PartialEq)]
pub struct AuthState {
    pub is_authenticated: bool,
    pub user_id: Option<String>,
    pub email: Option<String>,
    pub token: Option<String>,
}

impl Default for AuthState {
    fn default() -> Self {
        Self {
            is_authenticated: false,
            user_id: None,
            email: None,
            token: None,
        }
    }
}

// For backward compatibility with the terminal, we'll use your real admin email
const ADMIN_EMAIL: &str = "Benjination2@gmail.com";

pub async fn verify_blog_password(password: &str) -> bool {
    // For the terminal interface, we'll authenticate using the admin email and password
    authenticate_admin(ADMIN_EMAIL, password).await.is_ok()
}

pub async fn verify_blog_credentials(email: &str, password: &str) -> bool {
    authenticate_admin(email, password).await.is_ok()
}

pub async fn authenticate_admin(email: &str, password: &str) -> Result<AuthState, String> {
    // Add console logging for debugging
    web_sys::console::log_1(&format!("Attempting authentication for email: {}", email).into());
    
    let sign_in_request = SignInRequest {
        email: email.to_string(),
        password: password.to_string(),
        return_secure_token: true,
    };

    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}",
        FIREBASE_API_KEY
    );

    web_sys::console::log_1(&format!("Firebase Auth URL: {}", url).into());

    match Request::post(&url)
        .header("Content-Type", "application/json")
        .json(&sign_in_request)
    {
        Ok(request) => {
            match request.send().await {
                Ok(response) => {
                    let status = response.status();
                    web_sys::console::log_1(&format!("Response status: {}", status).into());
                    
                    if status == 200 {
                        match response.json::<SignInResponse>().await {
                            Ok(auth_response) => {
                                web_sys::console::log_1(&"Authentication successful!".into());
                                
                                // Store token in localStorage for session persistence
                                if let Some(storage) = window()
                                    .and_then(|w| w.local_storage().ok())
                                    .flatten()
                                {
                                    let _ = storage.set_item("blog_auth_token", &auth_response.id_token);
                                    let _ = storage.set_item("blog_user_id", &auth_response.local_id);
                                    let _ = storage.set_item("blog_user_email", &auth_response.email);
                                }

                                Ok(AuthState {
                                    is_authenticated: true,
                                    user_id: Some(auth_response.local_id),
                                    email: Some(auth_response.email),
                                    token: Some(auth_response.id_token),
                                })
                            }
                            Err(e) => {
                                web_sys::console::log_1(&format!("Failed to parse success response: {:?}", e).into());
                                Err("Failed to parse authentication response".to_string())
                            }
                        }
                    } else {
                        // Handle authentication errors
                        match response.text().await {
                            Ok(error_text) => {
                                web_sys::console::log_1(&format!("Error response: {}", error_text).into());
                                
                                // Try to parse as error response
                                if let Ok(error_response) = serde_json::from_str::<ErrorResponse>(&error_text) {
                                    match error_response.error.message.as_str() {
                                        "INVALID_PASSWORD" => Err("Invalid password".to_string()),
                                        "EMAIL_NOT_FOUND" => Err("Email not found - make sure the user exists in Firebase Auth".to_string()),
                                        "USER_DISABLED" => Err("User account disabled".to_string()),
                                        "INVALID_EMAIL" => Err("Invalid email format".to_string()),
                                        msg => Err(format!("Authentication failed: {}", msg)),
                                    }
                                } else {
                                    Err(format!("Authentication failed with status {}: {}", status, error_text))
                                }
                            }
                            Err(_) => Err(format!("Authentication failed with status {}", status)),
                        }
                    }
                }
                Err(e) => {
                    web_sys::console::log_1(&format!("Network error: {:?}", e).into());
                    Err("Network error during authentication".to_string())
                }
            }
        }
        Err(e) => {
            web_sys::console::log_1(&format!("Request creation error: {:?}", e).into());
            Err("Failed to create authentication request".to_string())
        }
    }
}

pub fn logout() {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        let _ = storage.remove_item("blog_auth_token");
        let _ = storage.remove_item("blog_user_id");
        let _ = storage.remove_item("blog_user_email");
    }
}

pub fn get_stored_auth() -> Option<AuthState> {
    if let Some(storage) = window()
        .and_then(|w| w.local_storage().ok())
        .flatten()
    {
        if let (Ok(Some(token)), Ok(Some(user_id)), Ok(Some(email))) = (
            storage.get_item("blog_auth_token"),
            storage.get_item("blog_user_id"),
            storage.get_item("blog_user_email"),
        ) {
            return Some(AuthState {
                is_authenticated: true,
                user_id: Some(user_id),
                email: Some(email),
                token: Some(token),
            });
        }
    }
    None
}

// Verify if stored token is still valid
pub async fn verify_token(token: &str) -> bool {
    let url = format!(
        "https://identitytoolkit.googleapis.com/v1/accounts:lookup?key={}",
        FIREBASE_API_KEY
    );

    let payload = serde_json::json!({
        "idToken": token
    });

    match Request::post(&url)
        .header("Content-Type", "application/json")
        .json(&payload)
    {
        Ok(request) => {
            match request.send().await {
                Ok(response) => response.status() == 200,
                Err(_) => false,
            }
        }
        Err(_) => false,
    }
}
