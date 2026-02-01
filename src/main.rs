use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    email: String,
    role: UserRole,
}

#[derive(Debug, Serialize, Deserialize)]
enum UserRole {
    Admin,
    Member,
    Guest,
}

#[derive(Debug)]
struct UserStore {
    users: HashMap<u64, User>,
    next_id: u64,
}

impl UserStore {
    fn new() -> Self {
        Self {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_user(&mut self, name: String, email: String, role: UserRole) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        
        let user = User { id, name, email, role };
        self.users.insert(id, user);
        id
    }

    fn get_user(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }

    fn list_users(&self) -> Vec<&User> {
        self.users.values().collect()
    }

    fn delete_user(&mut self, id: u64) -> bool {
        self.users.remove(&id).is_some()
    }
}

fn main() {
    println!("ReviewPal Demo - Rust Edition");
    
    let mut store = UserStore::new();
    
    // Add some users
    let admin_id = store.add_user(
        "Alice".to_string(),
        "alice@example.com".to_string(),
        UserRole::Admin,
    );
    
    let member_id = store.add_user(
        "Bob".to_string(),
        "bob@example.com".to_string(),
        UserRole::Member,
    );
    
    println!("Created admin with ID: {}", admin_id);
    println!("Created member with ID: {}", member_id);
    
    // List all users
    println!("\nAll users:");
    for user in store.list_users() {
        println!("  {:?}", user);
    }
}
