# Firebase Auth Setup Guide

## Step 1: Enable Authentication
1. Go to [Firebase Console](https://console.firebase.google.com/)
2. Select your project: `portfolio-7148b`
3. Click **Authentication** in the left sidebar
4. Click **Get started** if not already enabled
5. Go to **Sign-in method** tab
6. Enable **Email/Password** authentication

## Step 2: Create Admin User
1. Go to **Users** tab in Authentication
2. Click **Add user**
3. Enter:
   - **Email**: `admin@benjination.com`
   - **Password**: Choose a secure password (this replaces "test123")
4. Click **Add user**

## Step 3: Test the Authentication
1. Your portfolio terminal should now authenticate against Firebase Auth
2. Use the email `admin@benjination.com` and your chosen password
3. For the terminal, just use the password (email is hardcoded to admin@benjination.com)

## Security Features Now Active:
- ✅ Secure password hashing (Firebase handles this)
- ✅ Session tokens stored in localStorage
- ✅ Token validation
- ✅ Proper authentication flow
- ✅ User management through Firebase Console

## What Changed:
- Passwords are no longer stored in plain text
- Authentication uses Firebase Auth APIs
- Sessions persist across browser refreshes
- You can manage users through Firebase Console
- More secure and professional authentication system

## Managing Users:
- Add/remove users in Firebase Console → Authentication → Users
- Change passwords through Firebase Console
- Monitor authentication activity
