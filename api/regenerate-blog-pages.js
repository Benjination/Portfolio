// Serverless function to trigger blog page regeneration
// This can be deployed to Vercel, Netlify, or similar platform

const https = require('https');

// GitHub configuration
const GITHUB_TOKEN = process.env.GITHUB_TOKEN; // Set this in your serverless platform
const GITHUB_REPO = 'Benjination/Portfolio';

module.exports = async (req, res) => {
    // Enable CORS
    res.setHeader('Access-Control-Allow-Origin', 'https://benjaminniccum.com');
    res.setHeader('Access-Control-Allow-Methods', 'POST, OPTIONS');
    res.setHeader('Access-Control-Allow-Headers', 'Content-Type, Authorization');
    
    if (req.method === 'OPTIONS') {
        res.status(200).end();
        return;
    }
    
    if (req.method !== 'POST') {
        res.status(405).json({ error: 'Method not allowed' });
        return;
    }
    
    // Simple authentication - check for a secret key
    const authHeader = req.headers.authorization;
    const expectedSecret = process.env.WEBHOOK_SECRET || 'your-secret-key';
    
    if (!authHeader || authHeader !== `Bearer ${expectedSecret}`) {
        res.status(401).json({ error: 'Unauthorized' });
        return;
    }
    
    try {
        // Trigger GitHub Actions workflow dispatch
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
                    res.status(200).json({ 
                        success: true, 
                        message: 'Static page regeneration triggered successfully' 
                    });
                } else {
                    console.error('GitHub API Error:', githubRes.statusCode, data);
                    res.status(500).json({ 
                        error: 'Failed to trigger regeneration',
                        details: data
                    });
                }
            });
        });
        
        githubReq.on('error', (error) => {
            console.error('Request error:', error);
            res.status(500).json({ error: 'Network error' });
        });
        
        githubReq.write(payload);
        githubReq.end();
        
    } catch (error) {
        console.error('Error:', error);
        res.status(500).json({ error: 'Internal server error' });
    }
};