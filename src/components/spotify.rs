use yew::prelude::*;
use web_sys::window;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Deserialize)]
struct SpotifyTrack {
    name: String,
    artists: Vec<SpotifyArtist>,
    album: SpotifyAlbum,
}

#[derive(Clone, PartialEq, Deserialize)]
struct SpotifyArtist {
    name: String,
}

#[derive(Clone, PartialEq, Deserialize)]
struct SpotifyAlbum {
    name: String,
    images: Vec<SpotifyImage>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct SpotifyImage {
    url: String,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Clone, PartialEq, Deserialize)]
struct SpotifyCurrentlyPlaying {
    item: Option<SpotifyTrack>,
    is_playing: bool,
}

#[function_component(SpotifyWidget)]
pub fn spotify_widget() -> Html {
    let current_track = use_state(|| None::<SpotifyTrack>);
    let is_playing = use_state(|| false);
    let is_loading = use_state(|| true);

    // For now, let's create a beautiful static widget that shows your Spotify profile
    // We can add the API integration later if needed
    html! {
        <div class="spotify-widget">
            <div class="spotify-header">
                <span class="spotify-icon">{"🎵"}</span>
                <span class="spotify-title">{"Music & Coding"}</span>
                <span class="spotify-status">{"LIVE"}</span>
            </div>
            <div class="spotify-content">
                <div class="spotify-fallback">
                    <div class="spotify-placeholder">
                        <div class="spotify-album-art">
                            <span class="spotify-icon-large">{"🎧"}</span>
                        </div>
                        <div class="spotify-info">
                            <div class="spotify-track">{"See what I'm listening to"}</div>
                            <div class="spotify-artist">{"Follow my Spotify for real-time updates"}</div>
                        </div>
                    </div>
                    <div class="spotify-actions">
                        <a href="https://open.spotify.com/user/31jc2jyehqkhi3uqnre3fh3ic2by" target="_blank" class="spotify-profile-link">
                            <span class="spotify-icon">{"🎵"}</span>
                            <span>{"Follow on Spotify"}</span>
                        </a>
                        <span class="spotify-note">{"Check out my playlists and see what I'm currently vibing to!"}</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
