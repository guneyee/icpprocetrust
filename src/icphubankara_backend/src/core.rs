use std::sync::{Arc, RwLock};

// Donation Place data structure
#[derive(Debug)]
struct DonationPlace {
    id: usize,
    name: String,
    location: String,
}

// User data structure
#[derive(Debug)]
struct User {
    id: usize,
    name: String,
    donated_amount: usize,
}

// Smart contract
struct NaturalDisasterRelief {
    // List of users
    users: Arc<RwLock<Vec<User>>>,
    // List of donation places
    donation_places: Arc<RwLock<Vec<DonationPlace>>>,
}

impl NaturalDisasterRelief {
    // Constructor
    fn new() -> Self {
        NaturalDisasterRelief {
            users: Arc::new(RwLock::new(Vec::new())),
            donation_places: Arc::new(RwLock::new(Vec::new())),
        }
    }

    // Function to add a user
    fn add_user(&self, name: String) -> usize {
        let mut users = self.users.write().unwrap();
        let id = users.len();
        let new_user = User {
            id,
            name,
            donated_amount: 0,
        };
        users.push(new_user);
        id
    }

    // Function to make a donation
    fn make_donation(&self, user_id: usize, amount: usize) -> bool {
        let mut users = self.users.write().unwrap();
        if user_id >= users.len() {
            return false; // Invalid user ID
        } else {
            users[user_id].donated_amount += amount;
            return true;
        }
    }

    // Function to distribute aid
    fn distribute_aid(&self, total_amount: usize) -> bool {
        let users = self.users.read().unwrap();
        let total_donations: usize = users.iter().map(|user| user.donated_amount).sum();
        if total_donations < total_amount {
            return false; // Insufficient donations
        } else {
            // Distribution process is carried out, for example transferred to areas in need of assistance
            return true;
        }
    }

    // Function to get user information
    fn get_user_info(&self, user_id: usize) -> Option<User> {
        let users = self.users.read().unwrap();
        if user_id >= users.len() {
            return None; // Invalid user ID
        } else {
            return Some(users[user_id].clone());
        }
    }

    // Function to get the total donation amount
    fn total_donations(&self) -> usize {
        let users = self.users.read().unwrap();
        users.iter().map(|user| user.donated_amount).sum()
    }

    // Function to distribute donations equally to those in need
    fn distribute_to_needy(&self, needy_users: &[usize]) -> bool {
        let total_donations = self.total_donations();
        let donation_per_user = total_donations / needy_users.len();

        let mut users = self.users.write().unwrap();
        for &user_id in needy_users {
            if user_id >= users.len() {
                return false; // Invalid user ID
            } else {
                users[user_id].donated_amount += donation_per_user;
            }
        }
        true
    }

    // Add more functions as needed...
}

fn main() {
    let natural_disaster_relief = NaturalDisasterRelief::new();

    // Test the functions
    let user1_id = natural_disaster_relief.add_user("Alice".to_string());
    let user2_id = natural_disaster_relief.add_user("Bob".to_string());

    natural_disaster_relief.make_donation(user1_id, 100);
    natural_disaster_relief.make_donation(user2_id, 200);

    println!("Total donations: {}", natural_disaster_relief.total_donations());
}
