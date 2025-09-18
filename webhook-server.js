#!/usr/bin/env node

// Simple webhook server to trigger blog regeneration
// Run with: node webhook-server.js
// Or deploy to a cloud function platform

const http = require('http');
const https = require('https');
const url = require('url');

const PORT = process.env.PORT || 3001;

// Configuration
const GITHUB_TOKEN = process.env.GITHUB_TOKEN || 'your-github-token-here';
const GITHUB_REPO = 'Benjination/Portfolio';
const WEBHOOK_SECRET = process.env.WEBHOOK_SECRET || 'your-secret-key';

const server = http.createServer((req, res) => {
    // Enable CORS
    res.setHeader('Access-Control-Allow-Origin', '*');
    res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
    
    if (req.method === 'OPTIONS') {
        res.writeHead(200);
        res.end();
        return;
    }
    
    const parsedUrl = url.parse(req.url, true);
    
    if (req.method === 'POST' && parsedUrl.pathname === '/regenerate-blog-pages') {
        // Check authorization
        const authHeader = req.headers.authorization;
        if (!authHeader || authHeader !== `Bearer ${WEBHOOK_SECRET}`) {
            res.writeHead(401, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Unauthorized' }));
            return;
        }
        
        // Trigger GitHub Actions workflow
        const payload = JSON.stringify({
            event_type: 'regenerate-blog-pages',
            client_payload: {
                message: 'Blog post updated, regenerating static pages',
                timestamp: new Date().toISOString()
            }
        });
        
        const options = {
            hostname: 'api.github.com',
            port: 443,
            path: `/repos/${GITHUB_REPO}/dispatches`,
            method: 'POST',
            headers: {
                'Authorization': `token ${GITHUB_TOKEN}`,
                'Accept': 'application/vnd.github+json',
                'X-GitHub-Api-Version': '2022-11-28',
                'Content-Type': 'application/json',
                'Content-Length': Buffer.byteLength(payload),
                'User-Agent': 'Portfolio-Blog-Regenerator'
            }
        };
        
        const githubReq = https.request(options, (githubRes) => {
            let data = '';
            
            githubRes.on('data', (chunk) => {
                data += chunk;
            });
            
            githubRes.on('end', () => {
                if (githubRes.statusCode === 204) {
                    res.writeHead(200, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ 
                        success: true, 
                        message: 'Static page regeneration triggered successfully' 
                    }));
                } else {
                    console.error('GitHub API Error:', githubRes.statusCode, data);
                    res.writeHead(500, { 'Content-Type': 'application/json' });
                    res.end(JSON.stringify({ 
                        error: 'Failed to trigger regeneration',
                        details: data
                    }));
                }
            });
        });
        
        githubReq.on('error', (error) => {
            console.error('Request error:', error);
            res.writeHead(500, { 'Content-Type': 'application/json' });
            res.end(JSON.stringify({ error: 'Network error' }));
        });
        
        githubReq.write(payload);
        githubReq.end();
        
    } else {
        res.writeHead(404, { 'Content-Type': 'application/json' });
        res.end(JSON.stringify({ error: 'Not found' }));
    }
});

server.listen(PORT, () => {
    console.log(`Webhook server running on port ${PORT}`);
    console.log(`Endpoint: http://localhost:${PORT}/regenerate-blog-pages`);
    console.log('Set environment variables:');
    console.log('  GITHUB_TOKEN - Your GitHub personal access token');
    console.log('  WEBHOOK_SECRET - Secret key for authentication');
});