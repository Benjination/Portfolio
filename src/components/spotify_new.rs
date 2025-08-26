use yew::prelude::*;

#[function_component(SpotifyWidget)]
pub fn spotify_widget() -> Html {
    let fallback_visible = use_state(|| false);
    let image_loaded = use_state(|| false);
    
    let on_image_load = {
        let image_loaded = image_loaded.clone();
        Callback::from(move |_| {
            image_loaded.set(true);
        })
    };
    
    let on_image_error = {
        let fallback_visible = fallback_visible.clone();
        Callback::from(move |_| {
            fallback_visible.set(true);
        })
    };

    html! {
        <div class="spotify-widget">
            <div class="spotify-header">
                <span class="spotify-icon">{"🎵"}</span>
                <span class="spotify-title">{"Currently Playing"}</span>
            </div>
            <div class="spotify-content">
                if !*fallback_visible {
                    <img 
                        src="https://spotify-github-profile.vercel.app/api/spotify?background_color=0a0a0a&border_color=00ff00" 
                        alt="Spotify Playing" 
                        class={classes!("spotify-image", if *image_loaded { "loaded" } else { "" })}
                        onload={on_image_load}
                        onerror={on_image_error}
                    />
                } else {
                    <div class="spotify-fallback">
                        <div class="spotify-placeholder">
                            <div class="spotify-album-art">
                                <span class="spotify-icon-large">{"🎧"}</span>
                            </div>
                            <div class="spotify-info">
                                <div class="spotify-track">{"Setting up Spotify integration..."}</div>
                                <div class="spotify-artist">{"Check SPOTIFY_SETUP.md for instructions"}</div>
                            </div>
                        </div>
                        <div class="spotify-actions">
                            <a href="https://open.spotify.com/user/BennyThePooh" target="_blank" class="spotify-profile-link">
                                <span class="spotify-icon">{"🎵"}</span>
                                <span>{"View my Spotify profile"}</span>
                            </a>
                            <span class="spotify-note">{"This will show live music when configured"}</span>
                        </div>
                    </div>
                }
            </div>
        </div>
    }
}
