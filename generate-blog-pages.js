#!/usr/bin/env node

const https = require('https');
const fs = require('fs');
const path = require('path');

// Firebase configuration
const FIREBASE_API_KEY = 'AIzaSyAsmk3uImdPFOPLZrEsK6J1c20gk8S3hbY';
const PROJECT_ID = 'portfolio-7148b';

// Fetch blog posts from Firestore
async function fetchBlogPosts() {
    const url = `https://firestore.googleapis.com/v1/projects/${PROJECT_ID}/databases/(default)/documents/blogs?key=${FIREBASE_API_KEY}`;
    
    return new Promise((resolve, reject) => {
        https.get(url, (res) => {
            let data = '';
            res.on('data', (chunk) => {
                data += chunk;
            });
            res.on('end', () => {
                try {
                    const parsed = JSON.parse(data);
                    resolve(parsed);
                } catch (e) {
                    reject(e);
                }
            });
        }).on('error', (err) => {
            reject(err);
        });
    });
}

// Parse Firestore document to blog post
function parseBlogPost(doc) {
    const fields = doc.fields;
    if (!fields) return null;
    
    const getStringValue = (fieldName) => {
        return fields[fieldName]?.stringValue || '';
    };
    
    const getBoolValue = (fieldName) => {
        return fields[fieldName]?.booleanValue || false;
    };
    
    const title = getStringValue('title') || 'Untitled Post';
    const content = getStringValue('main_content') || getStringValue('content') || 'No content available';
    const excerpt = getStringValue('excerpt') || getStringValue('description') || content.substring(0, 150) + '...';
    const author = getStringValue('author') || 'Benjamin Niccum';
    const isPublished = getBoolValue('is_published') || getBoolValue('published');
    
    // Create ID from first 30 characters of title (URL-safe)
    const titleId = title.toLowerCase()
        .replace(/[^a-z0-9\s]/g, '') // Remove special characters
        .replace(/\s+/g, '-') // Replace spaces with hyphens
        .substring(0, 30) // Take first 30 characters
        .replace(/-+$/, ''); // Remove trailing hyphens
    
    return {
        id: titleId,
        title,
        content,
        excerpt,
        author,
        isPublished,
        dateCreated: getStringValue('date_created') || getStringValue('created_at') || '2024-01-01',
        dateUpdated: getStringValue('date_updated') || getStringValue('updated_at') || '2024-01-01'
    };
}

// Generate HTML for a blog post
function generateBlogPostHTML(post) {
    const formattedContent = post.content
        .replace(/\n/g, '<br>')
        .replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>')
        .replace(/\*(.*?)\*/g, '<em>$1</em>');
    
    return `<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="utf-8">
    <title>${post.title} - Benjamin Niccum</title>
    <meta name="description" content="${post.excerpt}">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <meta name="author" content="${post.author}">
    
    <!-- Open Graph Meta Tags for social sharing -->
    <meta property="og:title" content="${post.title}">
    <meta property="og:description" content="${post.excerpt}">
    <meta property="og:type" content="article">
    <meta property="og:url" content="https://benjaminniccum.com/blog/${post.id}">
    <meta property="og:image" content="https://benjaminniccum.com/og-image.png">
    <meta property="og:site_name" content="Benjamin Niccum - Portfolio">
    
    <!-- Twitter Card Meta Tags -->
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="${post.title}">
    <meta name="twitter:description" content="${post.excerpt}">
    <meta name="twitter:image" content="https://benjaminniccum.com/og-image.png">
    
    <!-- Article Meta -->
    <meta property="article:author" content="${post.author}">
    <meta property="article:published_time" content="${post.dateCreated}">
    <meta property="article:modified_time" content="${post.dateUpdated}">
    
    <!-- Canonical URL -->
    <link rel="canonical" href="https://benjaminniccum.com/blog/${post.id}">
    
    <!-- Favicon -->
    <link rel="icon" type="image/svg+xml" href="/favicon.svg">
    
    <!-- Immediate redirect to prevent static page display -->
    <script type="text/javascript">
        // Immediate redirect - don't let users see the static page
        (function() {
            const currentPath = window.location.pathname;
            console.log('🔗 Static page redirect from:', currentPath);
            
            // Extract the blog ID from the current path
            if (currentPath.includes('/blog/')) {
                const blogId = currentPath.split('/blog/')[1].replace(/\/$/, '').replace('.html', '');
                console.log('📍 Extracted blog ID:', blogId);
                
                // Immediate redirect to home with the blog ID in a hash
                window.location.replace('/#redirect-blog=' + encodeURIComponent(blogId));
            } else {
                // Fallback redirect to home
                window.location.replace('/');
            }
        })();
    </script>
    
    <!-- Meta refresh as backup in case JavaScript is disabled -->
    <meta http-equiv="refresh" content="0; url=/">
    
    <!-- Fallback styles for static page -->
    <style>
        body {
            font-family: 'Fira Code', 'JetBrains Mono', monospace;
            background: #0a0a0a;
            color: #e0e0e0;
            line-height: 1.6;
            margin: 0;
            padding: 0;
        }
        
        /* Header styles matching main site */
        .header {
            background: #0a0a0a;
            border-bottom: 2px solid #00ff00;
            padding: 1rem 0;
            margin-bottom: 2rem;
        }
        
        .header-content {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 2rem;
            display: flex;
            justify-content: space-between;
            align-items: center;
        }
        
        .header-left {
            display: flex;
            align-items: center;
            gap: 1rem;
        }
        
        .site-title {
            color: #00ff00;
            font-size: 1.5rem;
            font-weight: bold;
            text-decoration: none;
        }
        
        .nav-links {
            display: flex;
            gap: 2rem;
            list-style: none;
            margin: 0;
            padding: 0;
        }
        
        .nav-links a {
            color: #e0e0e0;
            text-decoration: none;
            padding: 0.5rem 1rem;
            border: 1px solid transparent;
            transition: all 0.3s ease;
        }
        
        .nav-links a:hover {
            color: #00ff00;
            border-color: #00ff00;
        }
        
        /* Main content container */
        .container {
            max-width: 1200px;
            margin: 0 auto;
            padding: 0 2rem 2rem 2rem;
        }
        
        /* Back button */
        .back-button {
            background: transparent;
            border: 2px solid #00ff00;
            color: #00ff00;
            padding: 0.75rem 1.5rem;
            font-family: inherit;
            font-size: 1rem;
            cursor: pointer;
            text-decoration: none;
            display: inline-block;
            margin-bottom: 2rem;
            transition: all 0.3s ease;
        }
        
        .back-button:hover {
            background: #00ff00;
            color: #0a0a0a;
        }
        
        /* Blog content */
        .blog-header {
            border-bottom: 2px solid #00ff00;
            padding-bottom: 2rem;
            margin-bottom: 3rem;
        }
        
        .blog-title {
            color: #00ff00;
            font-size: 2.5rem;
            margin: 0 0 1rem 0;
            font-weight: 700;
        }
        
        .blog-meta {
            color: #888;
            font-size: 1rem;
            margin-bottom: 1rem;
        }
        
        .blog-content {
            font-size: 1.125rem;
            line-height: 1.8;
        }
        
        .js-redirect-notice {
            background: #1a1a1a;
            border: 1px solid #00ff00;
            color: #00ff00;
            padding: 1rem;
            margin-bottom: 2rem;
            text-align: center;
            font-size: 0.9rem;
        }
        
        /* Terminal-style touches */
        .terminal-prompt {
            color: #00ff00;
            font-family: inherit;
        }
    </style>
</head>
<body>
    <!-- Header matching main portfolio -->
    <header class="header">
        <div class="header-content">
            <div class="header-left">
                <a href="/" class="site-title">Benjamin Niccum</a>
            </div>
            <nav>
                <ul class="nav-links">
                    <li><a href="/#about">About</a></li>
                    <li><a href="/#skills">Skills</a></li>
                    <li><a href="/#projects">Projects</a></li>
                    <li><a href="/#blog">Blog</a></li>
                    <li><a href="/#contact">Contact</a></li>
                </ul>
            </nav>
        </div>
    </header>

    <div class="container">
        <div class="js-redirect-notice">
            <span class="terminal-prompt">user@portfolio:~$</span> Redirecting to full interactive experience...
        </div>
        
        <!-- Back to main page button -->
        <a href="/#blog" class="back-button">← Back to Blog List</a>
        
        <article>
            <header class="blog-header">
                <h1 class="blog-title">${post.title}</h1>
                <div class="blog-meta">
                    <span class="terminal-prompt">author@</span>${post.author} • ${new Date(post.dateCreated).toLocaleDateString()}
                </div>
            </header>
            
            <div class="blog-content">
                ${formattedContent}
            </div>
        </article>
    </div>
    
    <noscript>
        <style>
            .js-redirect-notice { display: none; }
        </style>
    </noscript>
</body>
</html>`;
}

// Main function
async function generateStaticBlogPages() {
    try {
        console.log('Fetching blog posts from Firestore...');
        const response = await fetchBlogPosts();
        
        if (!response.documents) {
            console.log('No blog posts found.');
            return;
        }
        
        const blogPosts = response.documents
            .map(parseBlogPost)
            .filter(post => post && post.isPublished);
        
        console.log(`Found ${blogPosts.length} published blog posts.`);
        
        // Create blog directory
        const blogDir = path.join(__dirname, 'dist', 'blog');
        if (!fs.existsSync(blogDir)) {
            fs.mkdirSync(blogDir, { recursive: true });
        }
        
        // Generate static pages for each blog post
        for (const post of blogPosts) {
            const postDir = path.join(blogDir, post.id);
            if (!fs.existsSync(postDir)) {
                fs.mkdirSync(postDir, { recursive: true });
            }
            
            const htmlContent = generateBlogPostHTML(post);
            
            // Create both /blog/post_id/index.html and /blog/post_id.html
            const indexFilePath = path.join(postDir, 'index.html');
            const directFilePath = path.join(blogDir, `${post.id}.html`);
            
            fs.writeFileSync(indexFilePath, htmlContent);
            fs.writeFileSync(directFilePath, htmlContent);
            
            console.log(`Generated: /blog/${post.id}/index.html`);
            console.log(`Generated: /blog/${post.id}.html`);
        }
        
        console.log('✅ Static blog pages generated successfully!');
        
    } catch (error) {
        console.error('Error generating static blog pages:', error);
        process.exit(1);
    }
}

// Run the script
generateStaticBlogPages();