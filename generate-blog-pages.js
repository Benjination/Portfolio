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
    // Enhanced content formatting to match main app
    const formattedContent = post.content
        // Split into paragraphs and wrap in <p> tags
        .split(/\n\s*\n/)
        .map(paragraph => {
            if (paragraph.trim()) {
                // Handle bold text
                let formatted = paragraph.replace(/\*\*(.*?)\*\*/g, '<strong>$1</strong>');
                // Handle italic text
                formatted = formatted.replace(/\*(.*?)\*/g, '<em>$1</em>');
                // Handle single line breaks within paragraphs
                formatted = formatted.replace(/\n/g, '<br>');
                return `<p>${formatted}</p>`;
            }
            return '';
        })
        .filter(p => p)
        .join('\n\n');
    
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
    
    <!-- Static page styles matching main app -->
    <style>
        /* Terminal-inspired retro theme variables */
        :root {
            --primary-green: #00ff00;
            --secondary-green: #00cc00;
            --dark-green: #008800;
            --terminal-bg: #0a0a0a;
            --terminal-bg-light: #1a1a1a;
            --amber: #ffb000;
            --white: #ffffff;
            --gray: #cccccc;
            --light-gray: #999999;
            --dark-gray: #666666;
            --mono-font: 'Fira Code', 'JetBrains Mono', 'Courier New', monospace;
        }

        * {
            margin: 0;
            padding: 0;
            box-sizing: border-box;
        }

        body {
            font-family: 'Georgia', 'Times New Roman', serif;
            background: var(--terminal-bg);
            color: var(--white);
            line-height: 1.6;
            margin: 0;
            padding: 0;
        }

        /* Blog post page styling - exact match to main app */
        .blog-post-page {
            min-height: 100vh;
            background: linear-gradient(135deg, #0a0a0a 0%, #1a1a1a 100%);
            padding: 2rem 1rem;
            font-family: 'Georgia', 'Times New Roman', serif;
            color: var(--white);
        }

        .blog-content-container {
            max-width: 1200px;
            margin: 0 auto;
            background: rgba(26, 26, 26, 0.95);
            border-radius: 12px;
            overflow: hidden;
            box-shadow: 0 20px 40px rgba(0, 0, 0, 0.3);
        }

        .blog-header {
            background: linear-gradient(135deg, #1a1a1a 0%, #2a2a2a 100%);
            padding: 2rem;
            border-bottom: 2px solid var(--primary-green);
        }

        .back-button {
            background: transparent;
            border: 2px solid var(--primary-green);
            color: var(--primary-green);
            padding: 0.75rem 1.5rem;
            border-radius: 6px;
            font-family: var(--mono-font);
            font-size: 0.9rem;
            cursor: pointer;
            transition: all 0.3s ease;
            margin-bottom: 2rem;
            text-decoration: none;
            display: inline-block;
        }

        .back-button:hover {
            background: var(--primary-green);
            color: var(--terminal-bg);
            text-shadow: 0 0 5px currentColor, 0 0 10px currentColor, 0 0 15px currentColor;
        }

        .post-meta .post-title {
            font-size: 2.5rem;
            font-weight: 700;
            line-height: 1.2;
            margin: 0 0 1rem 0;
            color: var(--white);
            font-family: 'Georgia', serif;
        }

        .post-meta .post-info {
            display: flex;
            align-items: center;
            flex-wrap: wrap;
            margin-bottom: 1.5rem;
            font-size: 1rem;
            color: var(--light-gray);
        }

        .post-info .author {
            color: var(--primary-green);
            font-weight: 500;
        }

        .post-info .date-separator {
            margin: 0 0.5rem;
            color: var(--dark-gray);
        }

        .post-info .date {
            color: var(--light-gray);
        }

        .blog-content {
            padding: 3rem 4rem;
        }

        .content-text {
            font-size: 1.125rem;
            line-height: 1.8;
            color: #e0e0e0;
        }

        .content-text p {
            margin: 0 0 1.5rem 0;
            text-align: justify;
        }

        .content-text h1, 
        .content-text h2, 
        .content-text h3, 
        .content-text h4, 
        .content-text h5, 
        .content-text h6 {
            color: var(--white);
            margin: 2rem 0 1rem 0;
            font-family: 'Georgia', serif;
        }

        .content-text h1:first-child,
        .content-text h2:first-child,
        .content-text h3:first-child,
        .content-text h4:first-child,
        .content-text h5:first-child,
        .content-text h6:first-child {
            margin-top: 0;
        }

        .content-text h1 { font-size: 2.25rem; }
        .content-text h2 { font-size: 1.875rem; }
        .content-text h3 { font-size: 1.5rem; }
        .content-text h4 { font-size: 1.25rem; }

        .content-text strong {
            color: var(--primary-green);
            font-weight: 600;
        }

        .content-text em {
            color: var(--amber);
            font-style: italic;
        }

        .content-text code {
            background: rgba(0, 255, 0, 0.1);
            color: var(--primary-green);
            padding: 0.2rem 0.4rem;
            border-radius: 4px;
            font-family: var(--mono-font);
            font-size: 0.9em;
        }

        .content-text pre {
            background: var(--terminal-bg);
            color: var(--primary-green);
            padding: 1.5rem;
            border-radius: 8px;
            overflow-x: auto;
            margin: 1.5rem 0;
            border: 1px solid rgba(0, 255, 0, 0.3);
        }

        .content-text pre code {
            background: none;
            padding: 0;
        }

        .content-text blockquote {
            border-left: 4px solid var(--primary-green);
            padding-left: 1.5rem;
            margin: 1.5rem 0;
            font-style: italic;
            color: var(--light-gray);
            background: rgba(0, 255, 0, 0.05);
            padding: 1rem 1.5rem;
            border-radius: 0 8px 8px 0;
        }

        .content-text ul, 
        .content-text ol {
            margin: 1rem 0;
            padding-left: 2rem;
        }

        .content-text ul li, 
        .content-text ol li {
            margin: 0.5rem 0;
            color: #e0e0e0;
        }

        .content-text a {
            color: var(--primary-green);
            text-decoration: none;
            border-bottom: 1px solid rgba(0, 255, 0, 0.3);
            transition: all 0.3s ease;
        }

        .content-text a:hover {
            border-bottom-color: var(--primary-green);
            text-shadow: 0 0 5px currentColor, 0 0 10px currentColor, 0 0 15px currentColor;
        }

        /* Responsive design */
        @media (max-width: 768px) {
            .blog-post-page {
                padding: 1rem 0.5rem;
            }

            .blog-header {
                padding: 1.5rem;
            }

            .post-meta .post-title {
                font-size: 2rem;
            }

            .blog-content {
                padding: 2rem 1.5rem;
            }

            .content-text {
                font-size: 1rem;
            }
        }
    </style>
</head>
<body>
    <div class="blog-post-page">
        <div class="blog-content-container">
            <div class="blog-header">
                <a href="/#blog" class="back-button">← Back to Blog List</a>
                
                <div class="post-meta">
                    <h1 class="post-title">${post.title}</h1>
                    <div class="post-info">
                        <span class="author">${post.author}</span>
                        <span class="date-separator">•</span>
                        <span class="date">${new Date(post.dateCreated).toLocaleDateString()}</span>
                    </div>
                </div>
            </div>

            <div class="blog-content">
                <div class="content-text">
                    ${formattedContent}
                </div>
            </div>
        </div>
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