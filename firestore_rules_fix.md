# Firestore Security Rules Fix

## Problem
You're getting a 403 Forbidden error when trying to access your Firestore database. This is because your Firestore security rules require authentication, but the authentication isn't working properly.

## Solution Options

### Option 1: Allow Public Read Access for Blog Posts (Recommended)
This allows anyone to read blog posts, but only authenticated users can write.

1. Go to the Firebase Console: https://console.firebase.google.com/
2. Select your project: `portfolio-7148b`
3. Go to Firestore Database
4. Click on "Rules" tab
5. Replace your rules with:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    // Allow public read access to blog posts
    match /blogs/{document} {
      allow read: if true;
      allow write: if request.auth != null && request.auth.token.email == "Benjination2@gmail.com";
    }
    
    // Allow authenticated admin to read/write everything
    match /{document=**} {
      allow read, write: if request.auth != null && request.auth.token.email == "Benjination2@gmail.com";
    }
  }
}
```

### Option 2: Fix Authentication (More Complex)
If you want to keep authentication required, you need to:

1. Ensure your Firebase Auth token is being generated correctly
2. Make sure the token isn't expired
3. Verify the token is being sent in the correct format

## Current Issue Details
- Your app is making requests to: `https://firestore.googleapis.com/v1/projects/portfolio-7148b/databases/(default)/documents/blogs`
- The request includes an Authorization header with Bearer token
- Firestore is rejecting the request with 403 Forbidden

## Test the Fix
After updating your Firestore rules:
1. Save the rules in Firebase Console
2. Refresh your portfolio website
3. Try accessing the blog admin page again
4. Check the browser console for any remaining errors

## Security Notes
- Option 1 makes blog posts publicly readable, which is typically what you want for a blog
- Write access is still restricted to your admin email
- You can always make the rules more restrictive later if needed