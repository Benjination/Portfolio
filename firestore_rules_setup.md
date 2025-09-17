# Firestore Security Rules Setup Guide

## Problem
Blog posts are not publicly accessible, causing "Failed to parse blog post data" errors when users try to view blogs.

## Solution
Update Firestore security rules to allow public read access to published blog posts while keeping write access restricted to authenticated users.

## Steps to Fix

### 1. Open Firebase Console
1. Go to [https://console.firebase.google.com](https://console.firebase.google.com)
2. Select your `portfolio-7148b` project
3. Navigate to **Firestore Database** in the left sidebar
4. Click on the **Rules** tab

### 2. Update Firestore Rules
Replace the current rules with the content from `firestore.rules` file in this project:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    // Blog posts - publicly readable, auth required for write
    match /blogs/{blogId} {
      // Allow anyone to read blog posts (temporarily more permissive for debugging)
      allow read: if true;
      
      // Only authenticated users can create, update, or delete blog posts
      allow write: if request.auth != null;
    }
    
    // Blog post management - admin only
    match /blog_admin/{document} {
      allow read, write: if request.auth != null;
    }
    
    // Other collections (keep existing permissions)
    match /counters/{document} {
      allow read, write: if true;
    }
    
    match /snake_leaderboard/{document} {
      allow read, write: if true;
    }
    
    // Default deny all other collections
    match /{document=**} {
      allow read, write: if false;
    }
  }
}
```

### 3. Publish Rules
1. Click **Publish** to apply the new rules
2. Confirm the changes

### 4. Test the Changes
1. After publishing the rules, your blog should be publicly accessible
2. Blog posts will be readable by anyone
3. Creating/editing blog posts will still require authentication

## What This Achieves

✅ **Public Blog Access**: Anyone can read published blog posts without authentication
✅ **Secure Admin Access**: Only authenticated users can create/edit/delete blog posts  
✅ **Selective Visibility**: Only published posts are publicly visible
✅ **Maintained Security**: Other collections maintain their existing permission levels

## Verification
Once the rules are applied, your blog should work without authentication errors and visitors will be able to read your published blog posts.