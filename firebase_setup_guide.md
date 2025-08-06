# Firebase Setup Guide for Snake Game Leaderboard

## Your Current Setup: Firestore Database

I can see you already have Firestore set up at: https://console.firebase.google.com/u/1/project/portfolio-7148b/firestore/

### Step 1: Set Up Security Rules (REQUIRED)
1. Go to your [Firestore Console](https://console.firebase.google.com/u/1/project/portfolio-7148b/firestore/rules)
2. Click on the **"Rules"** tab
3. Replace the existing rules with:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /snake_leaderboard/{document} {
      allow read, write: if true;
    }
    match /counters/{document} {
      allow read, write: if true;
    }
  }
}
```

4. Click **"Publish"** to save the rules

### Step 2: Verify the Collections
The code will automatically create:
- `snake_leaderboard` collection when the first score is submitted
- `counters` collection for tracking site visits and game plays

### What This Enables:
- ✅ **Cross-device sharing**: Scores are shared across all devices
- ✅ **Persistent storage**: Scores are saved permanently 
- ✅ **Real global leaderboard**: All users see the same leaderboard
- ✅ **Automatic sorting**: Top 10 scores are maintained

### Security Note:
The rule `allow read, write: if true` makes the snake_leaderboard collection publicly readable and writable. This is fine for a game leaderboard but should not be used for sensitive data.

## Next Steps:
1. Set up the security rules above
2. The code is already updated to use Firestore
3. Test by setting a high score - it should appear in Firestore Console

## Option 2: Firestore (More complex but scalable)

### Step 1: Enable Firestore
1. In Firebase Console, click **"Firestore Database"**
2. Click **"Create database"**
3. Choose **"Start in test mode"**
4. Select your location

### Step 2: Set Up Security Rules
In Firestore, go to **"Rules"** tab and use:

```javascript
rules_version = '2';
service cloud.firestore {
  match /databases/{database}/documents {
    match /snake_leaderboard/{document} {
      allow read, write: if true;
    }
  }
}
```

## What to Send Me:

Please provide:
1. **Database URL**: The full URL from step 3 above
2. **Confirmation**: Which database type you chose (Realtime or Firestore)
3. **Screenshot**: Of the security rules to confirm they're set correctly

## Alternative: Keep localStorage + Add Firebase Later

We can also keep the current working localStorage solution and add Firebase as an enhancement later. The localStorage version works perfectly for demonstrating the feature.
