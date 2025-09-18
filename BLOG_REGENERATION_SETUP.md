# Blog Static Page Regeneration Setup

## Problem
When editing a blog using the sudo blog feature, it updates the Firestore database but doesn't regenerate the static HTML pages in `dist/blog/`. This means:
- ✅ Dynamic blog view (from home page) shows updated content
- ❌ Static landing pages (shared links) show old content

## Solution
This setup provides multiple ways to regenerate static blog pages when content is updated.

## Option 1: Manual Regeneration (Simplest)

After editing a blog post:

1. **Run the regeneration script:**
   ```bash
   npm run regenerate-blogs
   ```

2. **Commit and push the changes:**
   ```bash
   git add dist/blog/
   git commit -m "Update static blog pages"
   git push
   ```

3. **The static pages are now updated!**

## Option 2: Automated GitHub Actions (Recommended)

### Setup

1. **Create a GitHub Personal Access Token:**
   - Go to GitHub Settings → Developer settings → Personal access tokens
   - Create a token with `repo` and `workflow` permissions
   - Copy the token

2. **Deploy the webhook server:**
   - Deploy `webhook-server.js` to Vercel, Railway, or similar platform
   - Set environment variables:
     - `GITHUB_TOKEN` = your GitHub token
     - `WEBHOOK_SECRET` = a secret key for authentication

3. **Update the webhook URL in the code:**
   - Edit `src/components/blog_admin.rs`
   - Replace `your-serverless-function.vercel.app` with your actual webhook URL

### How it works

1. User edits blog post via sudo blog
2. Blog admin triggers webhook after successful save
3. Webhook calls GitHub Actions to regenerate static pages
4. Static pages are automatically updated and committed

## Option 3: Local Webhook Server (Development)

For local testing:

1. **Set up environment variables:**
   ```bash
   export GITHUB_TOKEN="your_github_token_here"
   export WEBHOOK_SECRET="your_secret_key"
   ```

2. **Run the webhook server:**
   ```bash
   node webhook-server.js
   ```

3. **Update the webhook URL to localhost:**
   ```rust
   let webhook_url = "http://localhost:3001/regenerate-blog-pages";
   ```

## Files Modified

1. **`.github/workflows/regenerate-blog-pages.yml`** - GitHub Actions workflow
2. **`src/components/blog_admin.rs`** - Added regeneration trigger
3. **`package.json`** - Added convenience scripts
4. **`webhook-server.js`** - Webhook server for automation
5. **`api/regenerate-blog-pages.js`** - Serverless function version

## Testing

1. **Edit a blog post** using the sudo blog feature
2. **Check the console** for regeneration messages
3. **Run manual regeneration** if needed:
   ```bash
   npm run regenerate-blogs
   ```
4. **Verify** that static pages in `dist/blog/` are updated

## Troubleshooting

- **Static pages not updating?** Run `npm run regenerate-blogs` manually
- **Webhook not working?** Check environment variables and URL
- **GitHub Actions failing?** Check token permissions and workflow syntax

## Benefits

- ✅ Dynamic blog view always shows current content
- ✅ Static landing pages show updated content after regeneration
- ✅ Shared blog links work correctly
- ✅ SEO and social sharing work properly
- ✅ Multiple options for different deployment scenarios