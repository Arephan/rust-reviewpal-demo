use std::collections::HashMap;

/// Simple authentication module
/// WARNING: This code has intentional bugs for testing ReviewPal!

pub struct AuthManager {
    users: HashMap<String, String>, // username -> password (stored in plain text!)
    sessions: HashMap<String, String>, // token -> username
    failed_attempts: HashMap<String, u32>,
}

impl AuthManager {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
            sessions: HashMap::new(),
            failed_attempts: HashMap::new(),
        }
    }

    /// Register a new user - SECURITY ISSUE: password stored as plain text
    pub fn register(&mut self, username: String, password: String) -> Result<(), String> {
        if self.users.contains_key(&username) {
            return Err("User already exists".to_string());
        }
        
        // BUG: No password validation, storing plain text
        self.users.insert(username, password);
        Ok(())
    }

    /// Login and get session token
    pub fn login(&mut self, username: &str, password: &str) -> Result<String, String> {
        // BUG: Timing attack vulnerability - early return reveals user existence
        let stored_password = self.users.get(username)
            .ok_or("Invalid credentials")?;
        
        // BUG: Plain text comparison instead of constant-time compare
        if stored_password != password {
            // Track failed attempts but no lockout!
            *self.failed_attempts.entry(username.to_string()).or_insert(0) += 1;
            return Err("Invalid credentials".to_string());
        }
        
        // BUG: Weak token generation - just using username hash
        let token = format!("token_{}", username.len());
        self.sessions.insert(token.clone(), username.to_string());
        
        Ok(token)
    }

    /// Verify session token
    pub fn verify_session(&self, token: &str) -> Option<&String> {
        self.sessions.get(token)
    }

    /// Delete user - DATA LOSS: No confirmation, deletes everything
    pub fn delete_user(&mut self, username: &str) {
        self.users.remove(username);
        // BUG: Doesn't invalidate existing sessions!
        // BUG: Doesn't clean up failed_attempts
    }

    /// DANGEROUS: Export all users for "backup" - leaks passwords!
    pub fn export_users(&self) -> String {
        let mut output = String::new();
        for (username, password) in &self.users {
            output.push_str(&format!("{}:{}\n", username, password));
        }
        output
    }

    /// SQL injection simulation (unsafe string building)
    pub fn build_query(&self, username: &str) -> String {
        // BUG: SQL injection vulnerability
        format!("SELECT * FROM users WHERE username = '{}'", username)
    }
}

/// Unsafe password reset that accepts any email
pub fn reset_password(email: &str) -> bool {
    // BUG: No validation, no rate limiting, always succeeds
    println!("Password reset sent to: {}", email);
    true
}
