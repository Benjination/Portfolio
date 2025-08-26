# Spotify Integration Setup Guide

## Current Implementation

Your portfolio now includes a Spotify "currently playing" widget in the contact section! Here's what we've implemented:

### ✅ What's Done
- **Spotify Widget Component**: A custom Yew component that displays your current playing track
- **Retro Terminal Styling**: Matches your portfolio's terminal aesthetic with green glow effects
- **Error Handling**: Falls back gracefully when Spotify isn't playing or service is unavailable
- **Direct Spotify Link**: Added to the contact links section
- **Responsive Design**: Looks great on all screen sizes

### 🎵 How It Works
The widget attempts to load an image from `https://novatorem-benjination.vercel.app/api/spotify` which should show:
- Album artwork
- Song title and artist
- Real-time updates
- "Not playing" state when nothing is active

## 🚀 Next Steps: Setting Up the Novatorem Service

To get the live Spotify data working, you need to set up the Novatorem service:

### Option 1: Use Novatorem (Recommended)
1. **Fork this repository**: https://github.com/novatorem/novatorem
2. **Create a Spotify App**:
   - Go to: https://developer.spotify.com/dashboard/
   - Create new app with these settings:
     - Name: `Portfolio Spotify Widget`
     - Description: `For portfolio website`
     - Website: `https://benjination.github.io/Portfolio`
     - Redirect URI: `https://benjination.github.io/Portfolio`

3. **Deploy to Vercel**:
   - Connect your forked repo to Vercel
   - Add environment variables:
     - `SPOTIFY_CLIENT_ID`: Your Client ID from step 2
     - `SPOTIFY_CLIENT_SECRET`: Your Client Secret from step 2
     - `SPOTIFY_REFRESH_TOKEN`: Get this from the auth process

4. **Update the URL**: Change the URL in `src/components/spotify.rs` from:
   ```rust
   src="https://novatorem-benjination.vercel.app/api/spotify"
   ```
   to your Vercel deployment URL:
   ```rust
   src="https://your-app-name.vercel.app/api/spotify"
   ```

### Option 2: Alternative Static Badge
If you prefer a simpler static approach, we can replace the dynamic widget with a styled Spotify badge that links to your profile.

## 🎨 Styling Features
- **Terminal glow effect** on the "Currently Playing" title
- **Pulsing animation** on the music note icon
- **CRT-style border** and scanlines
- **Hover effects** with green glow
- **Smooth transitions** between states

## 🛠️ Technical Details
- **Framework**: Yew (Rust WebAssembly)
- **State Management**: React-style hooks with `use_state`
- **Error Handling**: Graceful fallback to static content
- **Accessibility**: Proper alt text and semantic HTML

The widget is already integrated and will show the fallback content until you set up the Novatorem service!
